use std::io::stdout;
use crossterm::{execute, style::Print, cursor};

pub fn print(width: u16, height: u16) {
  let mut stdout = stdout();
  execute!( stdout, cursor::MoveTo(2, 1), Print("╭"))
    .expect("Failed to box print");
  for _ in 2..width {
    execute!(stdout, Print("─")).expect("Failed to box print");
  }
  execute!(stdout, Print("╮")).expect("Failed to box print");

  for y in 2..height {
    execute!(
      stdout,
      cursor::MoveTo(2, y),
      Print("│"),
      cursor::MoveTo(width + 1, y),
      Print("│")
    ).expect("Failed to box print");
  }

  execute!( stdout, cursor::MoveTo(2, height), Print("╰"))
    .expect("Failed to box print");
  for _ in 2..width {
    execute!(stdout, Print("─")).expect("Failed to box print");
  }
  execute!(stdout, Print("╯")).expect("Failed to box print");
}
