use crate::Coord;
use crossterm::style::{Color, Stylize};
use crossterm::{cursor::MoveTo, queue, style::Print};
use std::collections::VecDeque;
use std::io::{stdout, Result};

#[derive(Debug)]
pub struct Region {
    start: Coord,
    cols: usize,
    rows: usize,
    messages: VecDeque<String>,
}

impl Region {
    pub fn new(start: Coord, cols: usize, rows: usize) -> Self {
        let messages: VecDeque<String> = VecDeque::with_capacity(rows);
        Self {
            start,
            cols,
            rows,
            messages,
        }
    }
    pub fn print(&mut self, text: &str) -> Result<()> {
        let mut stdout = stdout();
        self.messages.push_front(String::from(text));
        self.messages.truncate(self.rows);
        for (index, line) in self.messages.iter().rev().enumerate() {
            queue!(
                stdout,
                MoveTo(self.start.col as u16, (self.start.row + index) as u16),
                Print(format!("{:<width$}", line, width = self.cols).with(Color::White))
            )?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct UI {
    start: Coord,
    cols: usize,
    rows: usize,
    pub system: Region,
    pub combat: Region,
}

impl UI {
    pub fn new(start: Coord, cols: usize, rows: usize) -> Self {
        let system_start = Coord {
            col: start.col + 1,
            row: start.row + 1,
        };
        let combat_start = Coord {
            col: cols / 2 + 1,
            row: start.row + 1,
        };
        let system = Region::new(system_start, cols / 2 - 2, rows - 2);
        let combat = Region::new(combat_start, cols / 2 - 2, rows - 2);
        Self {
            start,
            cols,
            rows,
            system,
            combat,
        }
    }

    /// Render the chrome of the ui
    pub fn render(&mut self, layout: String) -> Result<()> {
        let mut stdout = stdout();
        for (index, line) in layout.lines().take(self.rows).enumerate() {
            queue!(
                stdout,
                MoveTo(0, (self.start.row + index) as u16),
                Print(format!("{:<width$}", line, width = self.cols).with(Color::White))
            )?;
        }
        Ok(())
    }
}
