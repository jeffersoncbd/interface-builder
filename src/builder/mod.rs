pub mod components;
pub mod tools;

use std::{io::stdout, time::Duration};

use crossterm::{execute, terminal::{self, ClearType, enable_raw_mode, disable_raw_mode}, cursor, Result, event::{Event, KeyEvent, KeyCode, self, poll}, style};

pub fn read_line() -> String {
  fn read() -> Result<String> {
    let mut line = String::new();
    while let Event::Key(KeyEvent { code, .. }) = event::read()? {
      match code {
        KeyCode::Enter => {
          break;
        }
        KeyCode::Char(c) => {
          line.push(c);
        }
        _ => {}
      }
    }
    Ok(line)
  }
  read().expect("User entry failed")
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
pub fn read_char() -> char {
  pub fn read() -> Result<char> {
    loop {
      if let Event::Key(KeyEvent {
        code: KeyCode::Char(c),
        ..
      }) = event::read()?
      {
        return Ok(c);
      }
    }
  }
  read().expect("User entry failed")
}
