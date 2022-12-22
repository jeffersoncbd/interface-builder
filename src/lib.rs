mod builder;

use crossterm::event::KeyCode;
use builder::{tools, components::{Page, Line::*}};

pub use builder::components;

pub struct Application<'a> {
  home: Option<Page<'a>>
}
impl<'a> Application<'a> {
  pub fn new() -> Application<'a> {
    tools::clear_terminal();
    Application { home: None }
  }
  pub fn hello_builder(&self) {
    let mut page = Page::new(37, None);
    page.title("Hello builder");
    page.content(vec![
      Str("Welcome to Interface Builder")
    ]);
    page.footer(vec![
      Str("Press ESC to exit...")
    ]);
    page.print();

    tools::await_key_code(KeyCode::Esc);

    tools::clear_terminal();
  }
  pub fn set_home(&mut self, home: Page<'a>) {
    self.home = Some(home);
  }
  pub fn run(&mut self) {
    match &mut self.home {
      Some(home) => {
        home.print();
        tools::await_key_code(KeyCode::Esc);
        tools::clear_terminal();
      },
      None => panic!(
        "home is not implemented, use \"application.home(Page)\" before \"application.run()\""
      )
    }
  }
}
