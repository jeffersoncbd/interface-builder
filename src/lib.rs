mod builder;

use crossterm::event::KeyCode;
use builder::components;

pub struct Page {
  pub title: Option<String>,
  pub content: Vec<String>,
  pub footer: Option<Vec<String>>,
  pub width: u16,
  pub height: Option<u16>,
}

pub struct Application {
  home: Option<Page>
}
impl Application {
  pub fn new() -> Application {
    builder::tools::clear_terminal();
    Application { home: None }
  }

  pub fn run_hello_builder(&self) {
    self.print_page(&Page {
      title: Some(String::from("Hello builder")),
      content: vec![String::from("Welcome to Interface Builder")],
      footer: Some(vec![String::from("Press ESC to exit...")]),
      width: 37,
      height: None
    });

    builder::await_key_code(KeyCode::Esc);

    builder::tools::clear_terminal();
  }

  pub fn home(&mut self, page: Page) {
    self.home = Some(page);
  }

  pub fn run(&self) {
    match &self.home {
      Some(home) => self.print_page(home),
      None => panic!("home is not implemented, use application.home(Page) before application.run()")
    }
  }

  fn print_page(&self, page_data: &Page) {
    let mut page = components::Page::new(page_data.width, page_data.height);
    if let Some(title) = &page_data.title {
      page.title(&title);
    }

    let mut content: Vec<&str> = Vec::new();
    for line in &page_data.content {
      content.push(line);
    }
    page.content(content);

    if let Some(footer) =  &page_data.footer {
      let mut lines: Vec<&str> = Vec::new();
      for line in footer {
        lines.push(line);
      }
      page.footer(lines);
    }

    page.print();
  }
}
