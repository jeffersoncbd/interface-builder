mod builder;

use crossterm::event::KeyCode;
use builder::components;

pub struct Page {
  title: Option<String>,
  content: Vec<String>,
  footer: Option<Vec<String>>,
  width: u16,
  height: Option<u16>,
}
impl Page {
  fn convert_vec_str(vec: Vec<&str>) -> Vec<String> {
    let mut converted: Vec<String> = Vec::new();
    for item in vec {
      converted.push(String::from(item));
    }
    converted
  }

  pub fn new(
    title: Option<&str>,
    content: Vec<&str>,
    footer: Option<Vec<&str>>,
    width: u16,
    height: Option<u16>,
  ) -> Page {
    let title: Option<String> = match title {
      Some(title) => Some(String::from(title)), None => None
    };
    let content = Page::convert_vec_str(content);
    let footer: Option<Vec<String>> = match footer {
      Some(footer) => Some(Page::convert_vec_str(footer)), None => None
    };
    Page { title, content, footer, width, height }
  }
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
    self.print_page(&Page::new(
      Some("Hello builder"),
      vec!["Welcome to Interface Builder"],
      Some(vec!["Press ESC to exit..."]),
      37, None
    ));

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
