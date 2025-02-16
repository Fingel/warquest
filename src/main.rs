use crossterm::{
    cursor::MoveTo,
    event, execute,
    style::{Color, Print, Stylize},
};
use log2::*;
use std::{
    fs,
    io::{stdout, Result},
    ops::Add,
};

use map::World;

mod map;
mod terminal;

const WORLD_ROWS: usize = 40;
const WORLD_COLS: usize = 100;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Add<Direction> for Coord {
    type Output = Coord;

    fn add(self, other: Direction) -> Coord {
        match other {
            Direction::North => Coord {
                x: self.x,
                y: self.y.saturating_sub(1),
            },
            Direction::South => Coord {
                x: self.x,
                y: self.y.saturating_add(1),
            },
            Direction::East => Coord {
                x: self.x.saturating_add(1),
                y: self.y,
            },
            Direction::West => Coord {
                x: self.x.saturating_sub(1),
                y: self.y,
            },
        }
    }
}

#[derive(Debug)]
struct AppData {
    world: World,
    player_coord: Coord,
}

impl AppData {
    fn new(map: String) -> Self {
        Self {
            world: World::new(WORLD_ROWS, WORLD_COLS, map),
            player_coord: Coord {
                x: WORLD_COLS / 2,
                y: WORLD_ROWS / 2,
            },
        }
    }
}

fn render(app_data: &AppData) -> Result<()> {
    debug!("{:?}", app_data.player_coord);
    for (y, row) in app_data.world.tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            execute!(stdout(), MoveTo(x as u16, y as u16), Print(tile))?;
        }
    }
    execute!(
        stdout(),
        MoveTo(
            app_data.player_coord.x as u16,
            app_data.player_coord.y as u16
        ),
        Print("@".with(Color::White))
    )?;
    Ok(())
}

fn move_character(app_data: &mut AppData, direction: Direction) {
    let new_coord = app_data.player_coord + direction;
    if app_data.world.can_move_to(new_coord.y, new_coord.x) {
        app_data.player_coord = app_data.player_coord + direction;
    }
}

fn main() -> Result<()> {
    let _log2 = log2::open("warquest.log").start();
    let map = fs::read_to_string("map.txt").expect("Failed to read map file");
    terminal::setup(WORLD_COLS, WORLD_ROWS)?;
    let mut app_data = AppData::new(map);
    loop {
        render(&app_data)?;
        let event = event::read()?;
        if let event::Event::Key(event) = event {
            match event.code {
                event::KeyCode::Esc | event::KeyCode::Char('q') => break,
                event::KeyCode::Up => {
                    move_character(&mut app_data, Direction::North);
                }
                event::KeyCode::Down => {
                    move_character(&mut app_data, Direction::South);
                }
                event::KeyCode::Left => {
                    move_character(&mut app_data, Direction::West);
                }
                event::KeyCode::Right => {
                    move_character(&mut app_data, Direction::East);
                }
                _ => {}
            }
        }
    }

    // Be a good citizen, cleanup
    terminal::cleanup()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_character() {
        let map = ".....\n\
                   .....\n\
                   ..@..\n\
                   .....\n\
                   .....";
        let world = World::new(5, 5, map.to_string());
        let mut app_data = AppData {
            player_coord: Coord { x: 2, y: 2 },
            world,
        };
        move_character(&mut app_data, Direction::North);
        assert_eq!(app_data.player_coord, Coord { x: 2, y: 1 });
        move_character(&mut app_data, Direction::South);
        assert_eq!(app_data.player_coord, Coord { x: 2, y: 2 });
        move_character(&mut app_data, Direction::West);
        assert_eq!(app_data.player_coord, Coord { x: 1, y: 2 });
        move_character(&mut app_data, Direction::East);
        assert_eq!(app_data.player_coord, Coord { x: 2, y: 2 });
    }

    #[test]
    fn test_move_character_bounded() {
        let map = "@....\n\
                   .....\n\
                   .....\n\
                   .....\n\
                   .....";
        let world = World::new(5, 5, map.to_string());
        let mut app_data = AppData {
            player_coord: Coord { x: 0, y: 0 },
            world,
        };
        move_character(&mut app_data, Direction::North);
        assert_eq!(app_data.player_coord, Coord { x: 0, y: 0 });
        move_character(&mut app_data, Direction::South);
        assert_eq!(app_data.player_coord, Coord { x: 0, y: 1 });
        move_character(&mut app_data, Direction::West);
        assert_eq!(app_data.player_coord, Coord { x: 0, y: 1 });
        move_character(&mut app_data, Direction::East);
        assert_eq!(app_data.player_coord, Coord { x: 1, y: 1 });
    }

    #[test]
    fn test_can_move_character_walls() {
        let map = ".....\n\
                   #####\n\
                   .#@..\n\
                   .....\n\
                   .....";
        let world = World::new(5, 5, map.to_string());
        let mut app_data = AppData {
            player_coord: Coord { x: 2, y: 2 },
            world,
        };
        move_character(&mut app_data, Direction::North);
        assert_eq!(app_data.player_coord, Coord { x: 2, y: 2 });
        move_character(&mut app_data, Direction::West);
        assert_eq!(app_data.player_coord, Coord { x: 2, y: 2 });
        move_character(&mut app_data, Direction::South);
        assert_eq!(app_data.player_coord, Coord { x: 2, y: 3 });
        move_character(&mut app_data, Direction::East);
        assert_eq!(app_data.player_coord, Coord { x: 3, y: 3 });
    }
}
