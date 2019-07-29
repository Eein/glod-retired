use super::state::State;
use gtk::{
  Window,
};

pub struct App{
  pub state: State,
  pub window: Window,
}

impl App {
  pub fn new() -> App {
    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    App {
      state: State::new(),
      window,
    }
  }
}