use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Colors, Print, SetColors},
};
use log2::*;
use std::fmt;
use std::io::{Result, Stdout};

use crate::{Coord, Direction};

#[derive(Debug)]
pub struct World {
    entities: Vec<Entity>,
    tiles: Vec<Vec<Tile>>,
    player: Entity,
}

impl World {
    pub fn new(rows: usize, cols: usize, map: String) -> Self {
        let tiles = map
            .lines()
            .take(rows)
            .map(|line| line.chars().take(cols).map(Tile::from).collect())
            .collect();

        let player = Entity {
            name: String::from("An Adventurer"),
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

        let kobold = Entity {
            name: String::from("Kobold"),
            kind: EntityType::Enemy,
            rune: Rune {
                display: '&',
                color: Color::Red,
                background: Color::Black,
            },
            position: Coord { col: 60, row: 22 },
        };

        let marshall = Entity {
            name: String::from("Marshall McDoogal"),
            kind: EntityType::Npc,
            rune: Rune {
                display: '$',
                color: Color::Blue,
                background: Color::Black,
            },
            position: Coord { col: 60, row: 25 },
        };

        let entities = vec![kobold, marshall];
        Self {
            entities,
            tiles,
            player,
        }
    }

    pub fn move_player(&mut self, direction: Direction) {
        debug!("{:?} - {:?}", self.player.kind, self.player.position);
        let new_position = self.player.position + direction;

        if self.can_move_to(new_position.col, new_position.row) {
            self.player.position = new_position;
        }
    }

    pub fn closest_entity(&self) -> &Entity {
        self.entities
            .iter()
            .min_by_key(|e| e.position.distance(&self.player.position))
            .unwrap_or(&self.player)
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
        queue!(
            stdout,
            MoveTo(
                self.player.position.col as u16,
                self.player.position.row as u16
            ),
            SetColors(Colors::new(
                self.player.rune.color,
                self.player.rune.background
            )),
            Print(&self.player.rune),
        )?;

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
pub struct Entity {
    pub name: String,
    kind: EntityType,
    position: Coord,
    rune: Rune,
}

impl Entity {
    pub fn hail(&self) -> &'static str {
        match self.kind {
            EntityType::Player => "Hello!",
            EntityType::Enemy => "Grrr!",
            EntityType::Npc => "Hi there!",
        }
    }
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

    #[test]
    fn test_move_character() {
        let map = ".....\n\
                   .....\n\
                   ..@..\n\
                   .....\n\
                   .....";
        let mut world = World::new(5, 5, map.to_string());
        world.player.position = Coord { col: 2, row: 2 };
        world.move_player(Direction::North);
        assert_eq!(world.player.position, Coord { col: 2, row: 1 });
        world.move_player(Direction::South);
        assert_eq!(world.player.position, Coord { col: 2, row: 2 });
        world.move_player(Direction::West);
        assert_eq!(world.player.position, Coord { col: 1, row: 2 });
        world.move_player(Direction::East);
        assert_eq!(world.player.position, Coord { col: 2, row: 2 });
    }

    #[test]
    fn test_move_character_bounded() {
        let map = "@....\n\
                   .....\n\
                   .....\n\
                   .....\n\
                   .....";
        let mut world = World::new(5, 5, map.to_string());
        world.player.position = Coord { col: 0, row: 0 };
        world.move_player(Direction::North);
        assert_eq!(world.player.position, Coord { col: 0, row: 0 });
        world.move_player(Direction::South);
        assert_eq!(world.player.position, Coord { col: 0, row: 1 });
        world.move_player(Direction::West);
        assert_eq!(world.player.position, Coord { col: 0, row: 1 });
        world.move_player(Direction::East);
        assert_eq!(world.player.position, Coord { col: 1, row: 1 });
    }

    #[test]
    fn test_can_move_character_walls() {
        let map = ".....\n\
                   #####\n\
                   .#@..\n\
                   .....\n\
                   .....";
        let mut world = World::new(5, 5, map.to_string());
        world.move_player(Direction::North);
        world.player.position = Coord { col: 2, row: 2 };
        assert_eq!(world.player.position, Coord { col: 2, row: 2 });
        world.move_player(Direction::West);
        assert_eq!(world.player.position, Coord { col: 2, row: 2 });
        world.move_player(Direction::South);
        assert_eq!(world.player.position, Coord { col: 2, row: 3 });
        world.move_player(Direction::East);
        assert_eq!(world.player.position, Coord { col: 3, row: 3 });
    }
}
