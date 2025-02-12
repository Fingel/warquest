use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event, execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen, ScrollUp, SetSize,
    },
};
use std::io::{self};

struct Coord {
    x: u16,
    y: u16,
}

struct Size {
    width: u16,
    height: u16,
}

fn setup() -> io::Result<Size> {
    enable_raw_mode()?;
    let (cols, rows) = size()?;
    execute!(
        io::stdout(),
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

fn cleanup() -> io::Result<()> {
    let (cols, rows) = size()?;
    execute!(
        io::stdout(),
        LeaveAlternateScreen,
        SetSize(cols, rows),
        Show
    )?;
    disable_raw_mode()?;
    Ok(())
}

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

fn render(app_data: &AppData) -> io::Result<()> {
    execute!(
        io::stdout(),
        Clear(ClearType::All),
        MoveTo(app_data.player_coord.x, app_data.player_coord.y),
        Print("@")
    )?;
    Ok(())
}

fn main() -> io::Result<()> {
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
