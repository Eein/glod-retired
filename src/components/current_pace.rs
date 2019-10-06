use livesplit_core::component::current_pace::{
  Component, 
  Settings, 
};
use livesplit_core::settings::{Gradient::Plain, Color};
use livesplit_core::palette::LinSrgba;
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

    let state = component.state(&state.timer.read());
    let title = gtk::Label::new(None);
    title.set_text(&state.text);
    title.get_style_context().add_class("title");
    title.set_halign(gtk::Align::Start);
    title.set_hexpand(true);

    let time = gtk::Label::new(None);
    time.set_text(&state.time);
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
    let state = self.component.state(&state.timer.read());
    self.title.set_text(&state.text);
    self.time.set_text(&state.time);
    self.widget.show_all();
  }

  fn default_settings() -> Settings {
    let background = Plain(Color { rgba: LinSrgba::new(1.0, 0.5, 0.5, 0.8) });
    let comparison_override = None;
    let display_two_rows = false;
    let label_color = None;
    let value_color = None;
    let accuracy = livesplit_core::timing::formatter::Accuracy::Hundredths;

    Settings {
      background,
      comparison_override,
      display_two_rows,
      label_color,
      value_color,
      accuracy,
    }
  }
}