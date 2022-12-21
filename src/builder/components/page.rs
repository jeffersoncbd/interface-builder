use std::io::{stdout, Stdout};

use crossterm::{execute, terminal::{self, ClearType}, cursor, style::Print};

use super::tools;

pub struct Page {
  stdout: Stdout,
  width: u16,
  height: Option<u16>,
  title: Option<String>,
  content: Vec<String>,
  footer: Vec<String>
}
impl Page {
  pub fn new(width: u16, height: Option<u16>) -> Page {
    Page {
      stdout: stdout(),
      width, height,
      title: None,
      content: Vec::new(),
      footer: Vec::new()
    }
  }
  pub fn title(&mut self, title: &str) {
    self.title = Some(String::from(title));
  }
  pub fn content(&mut self, content: Vec<&str>){
    for line in content {
      self.content.push(String::from(line));
    }
  }
  pub fn footer(&mut self, footer: Vec<&str>) {
    for line in footer {
      self.footer.push(String::from(line));
    }
  }

  fn print_box(&self) {
    match self.height {
      Some(height) => tools::simple_box::print(self.width, height),
      None => tools::simple_box::print(self.width, self.content.len() as u16 + 4)
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
        cursor::MoveTo(6, i as u16 + 3),
        Print(line)
      ).expect("Failed on print content");
    }
  }
  fn print_footer(&mut self) {
    for (i, line) in self.footer.iter().enumerate() {
      let y: u16 = i as u16 + self.content.len() as u16 + 5;
      execute!(
        self.stdout,
        cursor::MoveTo(6, y),
        Print(line)
      ).expect("Failed on print content");
    }
  }
  pub fn print(&mut self) {
    execute!(
      self.stdout,
      terminal::Clear(ClearType::All),
      terminal::Clear(ClearType::Purge),
    ).expect("Print Page failed");
    self.print_box();
    self.print_title();
    self.print_content();
    self.print_footer();
  }
}
