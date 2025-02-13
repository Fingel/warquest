use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event, execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen, ScrollUp, SetSize,
    },
};
use log2::*;
use std::io::{stdout, Result};

#[derive(Debug)]
struct Coord {
    x: u16,
    y: u16,
}

struct Size {
    width: u16,
    height: u16,
}

fn setup() -> Result<Size> {
    enable_raw_mode()?;
    let (cols, rows) = size()?;
    execute!(
        stdout(),
        EnterAlternateScreen,
        SetSize(cols, rows),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        ScrollUp(rows),
        Hide,
    )?;
    Ok(Size {
        width: cols,
        height: rows,
    })
}

fn cleanup() -> Result<()> {
    let (cols, rows) = size()?;
    execute!(stdout(), LeaveAlternateScreen, SetSize(cols, rows), Show)?;
    disable_raw_mode()?;
    Ok(())
}

#[derive(Debug)]
struct AppData {
    player_coord: Coord,
}

impl AppData {
    fn new(cols: u16, rows: u16) -> Self {
        Self {
            player_coord: Coord {
                x: cols / 2,
                y: rows / 2,
            },
        }
    }
}

fn render(app_data: &AppData) -> Result<()> {
    debug!("{:?}", app_data);
    execute!(
        stdout(),
        Clear(ClearType::All),
        MoveTo(app_data.player_coord.x, app_data.player_coord.y),
        Print("@")
    )?;
    Ok(())
}

fn main() -> Result<()> {
    let _log2 = log2::open("warquest.log").start();
    let size = setup()?;
    let mut app_data = AppData::new(size.width, size.height);
    loop {
        render(&app_data)?;
        let event = event::read()?;
        if let event::Event::Key(event) = event {
            match event.code {
                event::KeyCode::Esc => break,
                event::KeyCode::Up => {
                    app_data.player_coord.y -= 1;
                }
                event::KeyCode::Down => {
                    app_data.player_coord.y += 1;
                }
                event::KeyCode::Left => {
                    app_data.player_coord.x -= 1;
                }
                event::KeyCode::Right => {
                    app_data.player_coord.x += 1;
                }
                _ => {}
            }
        }
    }

    // Be a good citizen, cleanup
    cleanup()?;
    Ok(())
}
