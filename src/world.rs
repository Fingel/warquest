use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Colors, Print, SetColors},
};
use std::fmt;
use std::io::{Result, Stdout};

use crate::Coord;

#[derive(Debug)]
pub struct World {
    entities: Vec<Entity>,
    tiles: Vec<Vec<Tile>>,
}

impl World {
    pub fn new(rows: usize, cols: usize, map: String) -> Self {
        let tiles = map
            .lines()
            .take(rows)
            .map(|line| line.chars().take(cols).map(Tile::from).collect())
            .collect();

        let player = Entity {
            kind: EntityType::Player,
            rune: Rune {
                display: '@',
                color: Color::Yellow,
                background: Color::Green,
            },
            position: Coord {
                col: cols / 2,
                row: rows / 2,
            },
        };
        let entities = vec![player];
        Self { entities, tiles }
    }

    pub fn can_move_to(&self, col: usize, row: usize) -> bool {
        row < self.tiles.len() && col < self.tiles[0].len() && !self.tiles[row][col].solid
    }

    pub fn render(&self, stdout: &mut Stdout) -> Result<()> {
        for (row, cols) in self.tiles.iter().enumerate() {
            for (col, tile) in cols.iter().enumerate() {
                queue!(
                    stdout,
                    MoveTo(col as u16, row as u16),
                    SetColors(Colors::new(tile.rune.color, tile.rune.background)),
                    Print(tile)
                )?;
            }
        }

        for entity in &self.entities {
            queue!(
                stdout,
                MoveTo(entity.position.col as u16, entity.position.row as u16),
                SetColors(Colors::new(entity.rune.color, entity.rune.background)),
                Print(&entity.rune),
            )?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum EntityType {
    Player,
    Enemy,
    Npc,
}

#[derive(Debug)]
struct Entity {
    kind: EntityType,
    position: Coord,
    rune: Rune,
}

#[derive(Debug, PartialEq)]
struct Tile {
    solid: bool,
    rune: Rune,
}

impl Tile {
    fn other(c: char) -> Tile {
        Tile {
            rune: Rune {
                display: c,
                color: Color::White,
                background: Color::Black,
            },
            solid: false,
        }
    }
    fn floor() -> Tile {
        Tile {
            rune: Rune {
                display: '.',
                color: Color::Grey,
                background: Color::Black,
            },
            solid: false,
        }
    }
    fn path() -> Tile {
        Tile {
            rune: Rune {
                display: '/',
                color: Color::Yellow,
                background: Color::Black,
            },
            solid: false,
        }
    }
    fn wall() -> Tile {
        Tile {
            rune: Rune {
                display: '#',
                color: Color::White,
                background: Color::Black,
            },
            solid: true,
        }
    }
    fn death() -> Tile {
        Tile {
            rune: Rune {
                display: '☠',
                color: Color::Red,
                background: Color::Black,
            },
            solid: false,
        }
    }
    fn mountain() -> Tile {
        Tile {
            rune: Rune {
                display: '^',
                color: Color::DarkMagenta,
                background: Color::Grey,
            },
            solid: true,
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
        write!(f, "{}", self.rune.display)
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

#[derive(Debug, PartialEq)]
struct Rune {
    color: Color,
    background: Color,
    display: char,
}

impl fmt::Display for Rune {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display)
    }
}
