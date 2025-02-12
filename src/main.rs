use crossterm::{
    cursor::{self, DisableBlinking, Hide, Show},
    queue,
    style::{self, Color, SetBackgroundColor, Stylize},
    terminal::{self, Clear, EnterAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use std::{
    io::{self, stdout, Write},
    thread::sleep,
    time::Duration,
};

fn main() -> io::Result<()> {
    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(EnterAlternateScreen)?;
    let width = 120;
    let height = 30;

    for y in 0..height {
        for x in 0..width {
            stdout
                .queue(cursor::MoveTo(x, y))?
                .queue(style::PrintStyledContent("█".black()))?;
        }
    }
    queue!(
        stdout,
        cursor::MoveTo(width / 2, height / 2),
        SetBackgroundColor(Color::Black),
        style::PrintStyledContent("WarQuest".red().bold()),
        Hide,
    )?;
    stdout.flush()?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(Show)?;
    Ok(())
}
