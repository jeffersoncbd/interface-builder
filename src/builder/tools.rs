use std::io::stdout;

use crossterm::{execute, cursor, terminal::{self, ClearType}};

pub fn clear_terminal() {
  execute!(
    stdout(),
    cursor::MoveTo(0, 0),
    terminal::Clear(ClearType::All),
    terminal::Clear(ClearType::Purge),
  ).expect("Failed to end application");
}
