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

pub fn move_cursor_to(x: u16, y: u16) {
  execute!(
    stdout(),
    cursor::MoveTo(x, y)
  ).expect("Failed to move cursor");
}
