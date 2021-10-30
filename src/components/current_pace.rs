use livesplit_core::component::current_pace::{
  Component,
  Settings,
};
use livesplit_core::settings::{Gradient::Plain, Color};
use gtk::*;

use crate::state::State;

pub struct CurrentPace {
  component: Component,
  time: gtk::Label,
  title: gtk::Label,
  pub widget: gtk::Box,
}

impl CurrentPace {
  pub fn new(state: &State) -> CurrentPace {
    let settings = CurrentPace::default_settings();
    let component = Component::with_settings(settings.clone());
    let widget = gtk::Box::new(Orientation::Horizontal, 0);
    widget.get_style_context().add_class("current-pace-container");
    widget.get_style_context().add_class("comparison-container");
    widget.set_hexpand(true);

    let state = component.state(&state.timer.read().snapshot());
    let title = gtk::Label::new(None);
    title.set_text(&state.key);
    title.get_style_context().add_class("title");
    title.set_halign(gtk::Align::Start);
    title.set_hexpand(true);

    let time = gtk::Label::new(None);
    time.set_text(&state.value);
    time.get_style_context().add_class("time");
    time.set_halign(gtk::Align::End);

    widget.add(&title);
    widget.add(&time);

    CurrentPace {
      component,
      widget,
      title,
      time,
    }
  }

  pub fn redraw(&mut self, state: &State) {
    let state = self.component.state(&state.timer.read().snapshot());
    self.title.set_text(&state.key);
    self.time.set_text(&state.value);
    self.widget.show_all();
  }

  fn default_settings() -> Settings {
    Settings {
        ..Default::default()
    }
  }
}
