use crossterm::{cursor::MoveTo, event, execute, style::Print};
use log2::*;
use std::{
    io::{stdout, Result},
    ops::Add,
};

mod terminal;

#[derive(Debug)]
struct World {
    tiles: Vec<Vec<char>>,
}

impl World {
    fn new(cols: u16, rows: u16) -> Self {
        let mut tiles = vec![vec!['.'; cols as usize]; rows as usize];
        for y in 0..rows {
            for x in 0..cols {
                if y == 19 && x > 40 && x < 60 {
                    tiles[y as usize][x as usize] = '#';
                }
            }
        }
        Self { tiles }
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone)]
struct Coord {
    x: u16,
    y: u16,
}

impl Add<Direction> for Coord {
    type Output = Coord;

    fn add(self, other: Direction) -> Coord {
        match other {
            Direction::North => Coord {
                x: self.x,
                y: self.y - 1,
            },
            Direction::South => Coord {
                x: self.x,
                y: self.y + 1,
            },
            Direction::East => Coord {
                x: self.x + 1,
                y: self.y,
            },
            Direction::West => Coord {
                x: self.x - 1,
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
    fn new(cols: u16, rows: u16) -> Self {
        Self {
            world: World::new(cols, rows),
            player_coord: Coord {
                x: cols / 2,
                y: rows / 2,
            },
        }
    }
}

fn render(app_data: &AppData) -> Result<()> {
    debug!("{:?}", app_data.player_coord);
    for (y, row) in app_data.world.tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            execute!(stdout(), MoveTo(x as u16, y as u16), Print(tile),)?;
        }
    }
    execute!(
        stdout(),
        MoveTo(app_data.player_coord.x, app_data.player_coord.y),
        Print("@")
    )?;
    Ok(())
}
fn can_move_character(app_data: &AppData, direction: Direction) -> bool {
    let max_y = app_data.world.tiles.len() as u16;
    let max_x = app_data.world.tiles[0].len() as u16;
    if app_data.player_coord.x == 0 && direction == Direction::West
        || app_data.player_coord.x == max_x && direction == Direction::East
    {
        return false;
    }
    if app_data.player_coord.y == 0 && direction == Direction::North
        || app_data.player_coord.y == max_y && direction == Direction::South
    {
        return false;
    }
    let new_coord = app_data.player_coord + direction;
    if app_data.world.tiles[new_coord.y as usize][new_coord.x as usize] == '#' {
        return false;
    }
    true
}

fn move_character(app_data: &mut AppData, direction: Direction) {
    if can_move_character(app_data, direction) {
        app_data.player_coord = app_data.player_coord + direction;
    }
}

fn main() -> Result<()> {
    let _log2 = log2::open("warquest.log").start();
    let size = terminal::setup()?;
    let mut app_data = AppData::new(size.width, size.height);
    loop {
        render(&app_data)?;
        let event = event::read()?;
        if let event::Event::Key(event) = event {
            match event.code {
                event::KeyCode::Esc => break,
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
