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

#[derive(Debug, PartialEq)]
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
            color: Color::Grey,
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
            color: Color::DarkMagenta,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_from_map() {
        let map = String::from(".#/\n^A ");
        let tiles = vec![
            vec![Tile::floor(), Tile::wall(), Tile::path()],
            vec![Tile::mountain(), Tile::other('A'), Tile::other(' ')],
        ];
        let world = World::new(2, 3, map);
        assert_eq!(world.tiles, tiles);
    }

    #[test]
    fn test_world_truncates_map() {
        let map = String::from("..NOT_HERE\nNOT_THERE");
        let tiles = vec![vec![Tile::floor(), Tile::floor()]];
        let world = World::new(1, 2, map);
        assert_eq!(world.tiles, tiles);
    }
}
