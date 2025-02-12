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

fn setup() -> io::Result<()> {
    enable_raw_mode()?;
    let (cols, rows) = size()?;
    execute!(
        io::stdout(),
        EnterAlternateScreen,
        SetSize(cols, rows),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        ScrollUp(rows),
        MoveTo(cols / 2, rows / 2),
        Print("@"),
        Hide,
    )?;
    Ok(())
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

fn main() -> io::Result<()> {
    setup()?;
    loop {
        let event = event::read()?;
        if let event::Event::Key(event) = event {
            match event.code {
                event::KeyCode::Esc => break,
                event::KeyCode::Up => {
                    execute!(
                        io::stdout(),
                        Clear(ClearType::All),
                        MoveTo(10, 10),
                        Print("@")
                    )?;
                }
                _ => {}
            }
        }
    }

    // Be a good citizen, cleanup
    cleanup()?;
    Ok(())
}
