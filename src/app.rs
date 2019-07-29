use super::state::State;
use super::components::title::Title;
use gtk::*;

const CSS: &str = include_str!("styles/app.css");

pub struct App{
  pub state: State,
  pub window: Window,
}

impl App {
  pub fn new() -> App {

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    let screen = window.get_screen().unwrap();
    let style = CssProvider::new();
    let _ = CssProviderExt::load_from_data(&style, CSS.as_bytes());
    StyleContext::add_provider_for_screen(&screen, &style, STYLE_PROVIDER_PRIORITY_USER);
    let state = State::new();

    window.add(&Title::new().widget(&state.timer));

    App {
      state,
      window,
    }
  }
}