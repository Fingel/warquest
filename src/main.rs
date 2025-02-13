use crossterm::{cursor::MoveTo, event, execute, style::Print};
use log2::*;
use std::io::{stdout, Result};

mod terminal;

#[derive(Debug)]
struct World {
    tiles: Vec<Vec<char>>,
}

impl World {
    fn new(cols: u16, rows: u16) -> Self {
        let tiles = vec![vec!['.'; cols as usize]; rows as usize];
        Self { tiles }
    }
}

#[derive(Debug)]
struct Coord {
    x: u16,
    y: u16,
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

fn move_character(app_data: &mut AppData, y: i16, x: i16) {
    let max_y = app_data.world.tiles.len() as u16;
    let max_x = app_data.world.tiles[0].len() as u16;
    if app_data.player_coord.x == 0 && x < 0 || app_data.player_coord.x == max_x && x > 0 {
        return;
    }
    if app_data.player_coord.y == 0 && y < 0 || app_data.player_coord.y == max_y && y > 0 {
        return;
    }
    match x {
        -1 => app_data.player_coord.x -= 1,
        1 => app_data.player_coord.x += 1,
        _ => {}
    }
    match y {
        -1 => app_data.player_coord.y -= 1,
        1 => app_data.player_coord.y += 1,
        _ => {}
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
                    move_character(&mut app_data, -1, 0);
                }
                event::KeyCode::Down => {
                    move_character(&mut app_data, 1, 0);
                }
                event::KeyCode::Left => {
                    move_character(&mut app_data, 0, -1);
                }
                event::KeyCode::Right => {
                    move_character(&mut app_data, 0, 1);
                }
                _ => {}
            }
        }
    }

    // Be a good citizen, cleanup
    terminal::cleanup()?;
    Ok(())
}
