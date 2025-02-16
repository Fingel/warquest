use crate::Coord;
use crossterm::{
    cursor::MoveTo,
    execute, queue,
    style::Print,
};
use log2::*;
use std::
    io::{stdout, Result, Write}
;

#[derive(Debug)]
pub struct UI {
    start: Coord,
    cols: usize,
    rows: usize,
}

impl UI {
    pub fn new(start: Coord, cols: usize, rows: usize) -> Self {
        Self { start, cols, rows }
    }

    pub fn render(&self) -> Result<()> {
        let mut stdout = stdout();
        for row in 0..self.rows {
            for col in 0..self.cols {
                debug!("UI render: col: {:?} row: {:?}", col, row);
                queue!(
                    stdout,
                    MoveTo(col as u16, row as u16 + self.start.row as u16),
                    Print(" ")
                )?;
            }
        }
        stdout.flush()?;
        Ok(())
    }

    pub fn print_line(&self, text: &str) -> Result<()> {
        let mut stdout = stdout();
        execute!(stdout, MoveTo(0, self.start.row as u16), Print(text))?;
        Ok(())
    }
}
