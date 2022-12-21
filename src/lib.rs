use crossterm::event::KeyCode;

mod builder;

pub struct Application {
}
impl Application {
  pub fn hello_builder(&self) {
    let mut page = builder::components::Page::new(37, None);
    page.title("Hello builder");
    page.content(vec!["Welcome to Interface Builder"]);
    page.footer(vec!["Press ESC to exit..."]);
    page.print();

    builder::await_key_code(KeyCode::Esc);

    builder::tools::clear_terminal();
  }

  pub fn new() -> Application {
    Application { }
  }
}
