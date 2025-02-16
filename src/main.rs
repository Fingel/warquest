use crossterm::{
    cursor::MoveTo,
    event, queue,
    style::{Color, Print, Stylize},
};
use log2::*;
use std::{
    fs,
    io::{stdout, Result, Write},
    ops::Add,
};

use map::World;
use ui::UI;

mod map;
mod terminal;
mod ui;

const WORLD_COLS: usize = 100;
const WORLD_ROWS: usize = 35;
const SCREEN_COLS: usize = WORLD_COLS;
const SCREEN_ROWS: usize = WORLD_ROWS + 5;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coord {
    col: usize,
    row: usize,
}

impl Add<Direction> for Coord {
    type Output = Coord;

    fn add(self, other: Direction) -> Coord {
        match other {
            Direction::North => Coord {
                col: self.col,
                row: self.row.saturating_sub(1),
            },
            Direction::South => Coord {
                col: self.col,
                row: self.row.saturating_add(1),
            },
            Direction::East => Coord {
                col: self.col.saturating_add(1),
                row: self.row,
            },
            Direction::West => Coord {
                col: self.col.saturating_sub(1),
                row: self.row,
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
                col: WORLD_COLS / 2,
                row: WORLD_ROWS / 2,
            },
        }
    }
}

fn render(app_data: &AppData) -> Result<()> {
    debug!("{:?}", app_data.player_coord);
    let mut stdout = stdout();
    app_data.world.render(&mut stdout)?;
    queue!(
        stdout,
        MoveTo(
            app_data.player_coord.col as u16,
            app_data.player_coord.row as u16
        ),
        Print("@".with(Color::White))
    )?;
    stdout.flush()?;
    Ok(())
}

fn move_character(app_data: &mut AppData, direction: Direction) {
    let new_coord = app_data.player_coord + direction;
    if app_data.world.can_move_to(new_coord.col, new_coord.row) {
        app_data.player_coord = app_data.player_coord + direction;
    }
}

fn main() -> Result<()> {
    let _log2 = log2::open("warquest.log").start();
    let map = fs::read_to_string("map.txt").expect("Failed to read map file");
    terminal::setup(SCREEN_COLS, SCREEN_ROWS)?;
    let ui_start = Coord {
        col: 0,
        row: WORLD_ROWS,
    };
    let ui = UI::new(ui_start, SCREEN_COLS, SCREEN_ROWS - WORLD_ROWS);
    ui.render()?;
    let mut app_data = AppData::new(map);
    loop {
        render(&app_data)?;
        let event = event::read()?;
        if let event::Event::Key(event) = event {
            match event.code {
                event::KeyCode::Esc | event::KeyCode::Char('q') => break,
                event::KeyCode::Up => {
                    move_character(&mut app_data, Direction::North);
                    ui.print_line("YOU MOVED NORTH")?;
                }
                event::KeyCode::Down => {
                    move_character(&mut app_data, Direction::South);
                    ui.print_line("YOU MOVED SOUTH")?;
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
            player_coord: Coord { col: 2, row: 2 },
            world,
        };
        move_character(&mut app_data, Direction::North);
        assert_eq!(app_data.player_coord, Coord { col: 2, row: 1 });
        move_character(&mut app_data, Direction::South);
        assert_eq!(app_data.player_coord, Coord { col: 2, row: 2 });
        move_character(&mut app_data, Direction::West);
        assert_eq!(app_data.player_coord, Coord { col: 1, row: 2 });
        move_character(&mut app_data, Direction::East);
        assert_eq!(app_data.player_coord, Coord { col: 2, row: 2 });
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
            player_coord: Coord { col: 0, row: 0 },
            world,
        };
        move_character(&mut app_data, Direction::North);
        assert_eq!(app_data.player_coord, Coord { col: 0, row: 0 });
        move_character(&mut app_data, Direction::South);
        assert_eq!(app_data.player_coord, Coord { col: 0, row: 1 });
        move_character(&mut app_data, Direction::West);
        assert_eq!(app_data.player_coord, Coord { col: 0, row: 1 });
        move_character(&mut app_data, Direction::East);
        assert_eq!(app_data.player_coord, Coord { col: 1, row: 1 });
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
            player_coord: Coord { col: 2, row: 2 },
            world,
        };
        move_character(&mut app_data, Direction::North);
        assert_eq!(app_data.player_coord, Coord { col: 2, row: 2 });
        move_character(&mut app_data, Direction::West);
        assert_eq!(app_data.player_coord, Coord { col: 2, row: 2 });
        move_character(&mut app_data, Direction::South);
        assert_eq!(app_data.player_coord, Coord { col: 2, row: 3 });
        move_character(&mut app_data, Direction::East);
        assert_eq!(app_data.player_coord, Coord { col: 3, row: 3 });
    }
}
