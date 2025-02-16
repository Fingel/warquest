use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, Stylize},
};
use std::fmt;
use std::io::{Result, Stdout};

#[derive(Debug)]
pub struct World {
    tiles: Vec<Vec<Tile>>,
}

impl World {
    pub fn new(rows: usize, cols: usize, map: String) -> Self {
        let tiles = map
            .lines()
            .take(rows)
            .map(|line| line.chars().take(cols).map(Tile::from).collect())
            .collect();
        Self { tiles }
    }

    pub fn can_move_to(&self, col: usize, row: usize) -> bool {
        row < self.tiles.len() && col < self.tiles[0].len() && !self.tiles[row][col].solid
    }

    pub fn render(&self, stdout: &mut Stdout) -> Result<()> {
        for (row, cols) in self.tiles.iter().enumerate() {
            for (col, tile) in cols.iter().enumerate() {
                queue!(stdout, MoveTo(col as u16, row as u16), Print(tile))?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Tile {
    color: Color,
    solid: bool,
    display: char,
}

impl Tile {
    fn other(c: char) -> Tile {
        Tile {
            color: Color::White,
            solid: false,
            display: c,
        }
    }
    fn floor() -> Tile {
        Tile {
            color: Color::White,
            solid: false,
            display: '.',
        }
    }
    fn path() -> Tile {
        Tile {
            color: Color::Yellow,
            solid: false,
            display: '/',
        }
    }
    fn wall() -> Tile {
        Tile {
            color: Color::White,
            solid: true,
            display: '#',
        }
    }
    fn death() -> Tile {
        Tile {
            color: Color::Red,
            solid: false,
            display: '☠',
        }
    }
    fn mountain() -> Tile {
        Tile {
            color: Color::DarkGrey,
            solid: true,
            display: '^',
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::floor(),
            '/' => Tile::path(),
            '#' => Tile::wall(),
            '☠' => Tile::death(),
            '^' => Tile::mountain(),
            _ => Tile::other(c),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display.with(self.color))
    }
}
