use std::{io::stdout, time::Duration};

use crossterm::{
  execute, cursor,
  terminal::{self, ClearType, enable_raw_mode, disable_raw_mode},
  event::{KeyCode, Event, self, poll}
};

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

pub fn await_key_code(key_code: KeyCode) {
  enable_raw_mode().expect("Failed on enable raw mode");
  execute!(
    stdout(),
    cursor::MoveTo(0,0),
    cursor::Hide
  ).expect("Move failed");
  loop {
    if poll(Duration::from_millis(50)).expect("Error") {
      let event = event::read().expect("Failed on read event");

      if event == Event::Key(key_code.into()) {
        execute!(
          stdout(),
          cursor::Show
        ).expect("Cleaning failed");
        break;
      }
    } else {
      execute!(
        stdout(),
        cursor::MoveTo(0,0),
        terminal::Clear(ClearType::CurrentLine),
        cursor::Hide
      ).expect("Cleaning failed");
    }
  }
  disable_raw_mode().expect("Failed on disable raw mode");
}
