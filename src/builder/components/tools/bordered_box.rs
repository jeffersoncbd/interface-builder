use std::io::stdout;
use crossterm::{execute, style::Print, cursor};

pub fn print(width: u16, height: u16) {
  let mut stdout = stdout();

  let mut horizontal_border = String::new();
  for _ in 2..width {
    horizontal_border.push('─');
  }

  execute!(
    stdout,
    cursor::MoveTo(2, 1),
    Print("╭"),
    Print(&horizontal_border),
    Print("╮"),
  ).expect("failed to print superior border.");

  for y in 2..height {
    execute!(
      stdout,
      cursor::MoveTo(2, y),
      Print("│"),
      cursor::MoveTo(width + 1, y),
      Print("│")
    ).expect("failed to print vertical borders.");
  }

  execute!(
    stdout,
    cursor::MoveTo(2, height),
    Print("╰"),
    Print(&horizontal_border),
    Print("╯"),
  ).expect("failed to print inferior border");
}
