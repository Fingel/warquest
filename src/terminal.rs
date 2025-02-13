use crossterm::{
    cursor::{Hide, Show},
    execute,
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
        ScrollUp, SetSize,
    },
};
use std::io::{stdout, Result};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub fn setup() -> Result<Size> {
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

pub fn cleanup() -> Result<()> {
    let (cols, rows) = size()?;
    execute!(stdout(), LeaveAlternateScreen, SetSize(cols, rows), Show)?;
    disable_raw_mode()?;
    Ok(())
}
