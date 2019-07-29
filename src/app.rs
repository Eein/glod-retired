use crate::state::State;
use crate::components::title::Title;
use crate::components::splits::Splits;
use gtk::*;

const CSS: &str = include_str!("styles/app.css");

pub struct App{
  pub state: State,
  pub window: Window,
  pub title: gtk::Box,
  pub splits: Splits,
  pub test_button: gtk::Button,
  pub split_button: gtk::Button,
}

impl App {

  pub fn new() -> App {
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    let screen = window.get_screen().unwrap();
    let style = CssProvider::new();
    let _ = CssProviderExt::load_from_data(&style, CSS.as_bytes());
    StyleContext::add_provider_for_screen(&screen, &style, STYLE_PROVIDER_PRIORITY_USER);
    let s = State::new();

    let container = gtk::Box::new(Orientation::Vertical, 0);
    let title = Title::new().widget(&s);
    let splits = Splits::new(&s);
    let test_button = gtk::Button::new_with_label("START/PAUSE");
    let split_button = gtk::Button::new_with_label("SPLIT");

    container.add(&title);
    container.add(&splits.widget);
    container.add(&test_button);
    container.add(&split_button);

    window.add(&container);

    App {
      state: s,
      window,
      title,
      splits,
      test_button,
      split_button,
    }
  }
}