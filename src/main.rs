use crossterm::{
    event, queue,
    style::{Color, Colors, SetColors},
};
use std::{
    fs,
    io::{Result, Write, stdout},
    ops::Add,
};

use ui::UI;
use world::World;

mod terminal;
mod ui;
mod world;

const WORLD_COLS: usize = 100;
const WORLD_ROWS: usize = 33;
const SCREEN_COLS: usize = WORLD_COLS;
const SCREEN_ROWS: usize = WORLD_ROWS + 7;

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

impl Coord {
    pub fn distance(&self, other: &Coord) -> usize {
        let dx = self.col.abs_diff(other.col);
        let dy = self.row.abs_diff(other.row);
        dx + dy
    }
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
}

impl AppData {
    fn new(map: String) -> Self {
        Self {
            world: World::new(WORLD_ROWS, WORLD_COLS, map),
        }
    }
}

fn render(app_data: &AppData) -> Result<()> {
    let mut stdout = stdout();
    app_data.world.render(&mut stdout)?;
    queue!(stdout, SetColors(Colors::new(Color::White, Color::Black)))?;
    stdout.flush()?;
    Ok(())
}

fn main() -> Result<()> {
    let _log2 = log2::open("warquest.log").start();
    let map = fs::read_to_string("map.txt").expect("Failed to read map file");
    let ui_layout = fs::read_to_string("ui.txt").expect("Failed to read map file");
    terminal::setup(SCREEN_COLS, SCREEN_ROWS)?;
    let ui_start = Coord {
        col: 0,
        row: WORLD_ROWS,
    };
    let mut ui = UI::new(ui_start, SCREEN_COLS, SCREEN_ROWS - WORLD_ROWS);
    ui.render(ui_layout)?;
    let mut app_data = AppData::new(map);
    ui.system.print("Connected to WarQuest!")?;
    ui.system.print("Daily login bonus: 1,000,000 WarBux!")?;
    loop {
        render(&app_data)?;
        let event = event::read()?;
        if let event::Event::Key(event) = event {
            match event.code {
                event::KeyCode::Esc | event::KeyCode::Char('q') => break,
                event::KeyCode::Up => {
                    app_data.world.move_player(Direction::North);
                    ui.combat.print("YOU PRESSED ⇧")?;
                }
                event::KeyCode::Down => {
                    app_data.world.move_player(Direction::South);
                    ui.combat.print("YOU PRESSED ⇩")?;
                }
                event::KeyCode::Left => {
                    app_data.world.move_player(Direction::West);
                    ui.combat.print("YOU PRESSED ⇦")?;
                }
                event::KeyCode::Right => {
                    app_data.world.move_player(Direction::East);
                    ui.combat.print("YOU PRESSED ⇨")?;
                }
                event::KeyCode::Char('h') => {
                    let entity = app_data.world.closest_entity();
                    let response = entity.hail();
                    ui.combat
                        .print(format!("{} says: {}", entity.name, response).as_str())?;
                }
                _ => {}
            }
        }
    }

    // Be a good citizen, cleanup
    terminal::cleanup()?;
    Ok(())
}
