use crate::Coord;
use crossterm::style::{Color, Stylize};
use crossterm::{cursor::MoveTo, queue, style::Print};
use std::collections::VecDeque;
use std::io::{stdout, Result, Write};

#[derive(Debug)]
pub struct UI {
    start: Coord,
    cols: usize,
    rows: usize,
    messages: VecDeque<String>,
}

impl UI {
    pub fn new(start: Coord, cols: usize, rows: usize) -> Self {
        let messages: VecDeque<String> = VecDeque::with_capacity(rows);
        Self {
            start,
            cols,
            rows,
            messages,
        }
    }

    pub fn render(&mut self, layout: String) -> Result<()> {
        let mut stdout = stdout();
        for (index, line) in layout.lines().enumerate() {
            queue!(
                stdout,
                MoveTo(0, (self.start.row + index) as u16),
                Print(format!("{:<width$}", line, width = self.cols).with(Color::White))
            )?;
        }
        for _ in 4..self.rows {
            self.print_line(" ".repeat(self.cols - 2).as_str())?
        }
        self.print_line("Connected to WarQuest!")?;
        self.print_line("Daily login bonus: 5,000,000,000 WarBucks.")?;
        Ok(())
    }

    pub fn print_line(&mut self, text: &str) -> Result<()> {
        let mut stdout = stdout();
        self.messages.push_front(String::from(text));
        self.messages.truncate(self.rows - 2);
        for (index, message) in self.messages.iter().rev().enumerate() {
            queue!(
                stdout,
                MoveTo(
                    (self.start.col + 1) as u16,
                    (self.start.row + 1 + index) as u16
                ),
                Print(format!("{:<width$}", message, width = self.cols - 2).with(Color::White))
            )?;
        }
        stdout.flush()?;
        Ok(())
    }
}
