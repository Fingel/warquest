use crossterm::{
    cursor::{Hide, Show},
    execute,
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
        ScrollUp, SetSize,
    },
};
use std::io::{stdout, Error, ErrorKind, Result};

pub fn setup(cols: usize, rows: usize) -> Result<()> {
    enable_raw_mode()?;
    let (term_cols, term_rows) = size()?;
    if term_cols < cols as u16 || term_rows < rows as u16 {
        return Err(Error::new(ErrorKind::Other, "Terminal size is too small"));
    }
    execute!(
        stdout(),
        EnterAlternateScreen,
        SetSize(cols as u16, rows as u16),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        ScrollUp(rows as u16),
        Hide,
    )?;
    Ok(())
}

pub fn cleanup() -> Result<()> {
    let (cols, rows) = size()?;
    execute!(stdout(), LeaveAlternateScreen, SetSize(cols, rows), Show)?;
    disable_raw_mode()?;
    Ok(())
}
