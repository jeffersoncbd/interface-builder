use std::{io::{stdout, Stdout}, fmt::Display};

use crossterm::{execute, cursor, style::Print};

use crate::builder::tools::{clear_terminal, move_cursor_to};

use super::tools;

pub enum Line<'a> {
  String(String),
  Str(&'a str),
}
impl<'a> Display for Line<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Line::String(value) => write!(f, "{}", value),
      Line::Str(value) => write!(f, "{}", value),
    }
  }
}

pub struct Page<'a> {
  stdout: Stdout,
  width: u16,
  height: Option<u16>,
  title: Option<&'a str>,
  content: Vec<Line<'a>>,
  footer: Vec<Line<'a>>
}
impl<'a> Page<'a> {
  pub fn new(width: u16, height: Option<u16>) -> Page<'a> {
    Page {
      stdout: stdout(),
      width, height,
      title: None,
      content: Vec::new(),
      footer: Vec::new()
    }
  }
  pub fn title(&mut self, title: &'a str) {
    self.title = Some(title);
  }
  pub fn content(&mut self, content: Vec<Line<'a>>){
    self.content = content
  }
  pub fn footer(&mut self, footer: Vec<Line<'a>>) {
    self.footer = footer
  }

  fn print_box(&self) {
    match self.height {
      Some(height) => tools::bordered_box::print(self.width, height),
      None => tools::bordered_box::print(self.width, self.content.len() as u16 + 4)
    }
  }
  fn print_title(&mut self) {
    match &self.title {
      Some(title) => {
        execute!(
          self.stdout,
          cursor::MoveTo(5, 1),
          Print(format!(" {} ", &title))
        ).expect("Print title failed");
      },
      None => ()
    }
  }
  fn print_content(&mut self) {
    for (i, line) in self.content.iter().enumerate() {
      execute!(
        self.stdout,
        cursor::MoveTo(5, i as u16 + 3),
        Print(line)
      ).expect("Failed on print content");
    }
  }
  fn print_footer(&mut self) {
    for (i, line) in self.footer.iter().enumerate() {
      let y = match self.height {
        Some(height) => height + i as u16 + 1,
        None => i as u16 + self.content.len() as u16 + 5
      };
      execute!(
        self.stdout,
        cursor::MoveTo(5, y),
        Print(line)
      ).expect("Failed on print content");
    }
  }
  pub fn print(&mut self) {
    clear_terminal();
    self.print_box();
    self.print_title();
    self.print_content();
    self.print_footer();
    let y = match self.height {
      Some(height) => height + self.footer.len() as u16 + 2,
      None => self.content.len() as u16 + self.footer.len() as u16 + 6
    };
    move_cursor_to(0, y)
  }
}
