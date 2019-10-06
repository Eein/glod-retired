use livesplit_core::TimingMethod;
use livesplit_core::component::timer::{
  Component, 
  Settings, 
};
use livesplit_core::settings::{Gradient::Plain, Color};
use livesplit_core::palette::LinSrgba;
use gtk::*;

use crate::state::State;

pub struct Timer {
  component: Component,
  settings: Settings,
  seconds: gtk::Label,
  fraction: gtk::Label,
  pub widget: gtk::Box,
}

impl Timer {
  pub fn new(state: &State) -> Timer {
    let settings = Timer::default_settings();
    let mut component = Component::with_settings(settings.clone());
    let widget= gtk::Box::new(Orientation::Horizontal, 0);
    widget.get_style_context().add_class("timer-container");

    let timer = component.state(&state.timer.read(), &state.general_layout_settings);
    let seconds = gtk::Label::new(None);
    seconds.set_text(&timer.time);
    seconds.get_style_context().add_class("seconds");
    seconds.set_valign(Align::End);

    let fraction = gtk::Label::new(None);
    fraction.set_text(&timer.fraction);
    fraction.get_style_context().add_class("fraction");
    fraction.set_valign(Align::End);
    
    widget.pack_end(&fraction, false, false, 0);
    widget.pack_end(&seconds, false, false, 0);

    Timer {
      settings,
      component,
      widget,
      seconds,
      fraction,
    }
  }

  pub fn redraw(&mut self, state: &State) {
    let timer = &self.component.state(&state.timer.read(), &state.general_layout_settings);
    // This can be formatted into two configurable sizes like Livesplit
    self.seconds.set_text(&timer.time);
    self.fraction.set_text(&timer.fraction);

    self.widget.show_all();
  }

  fn default_settings() -> Settings {
    let background = Plain(Color { rgba: LinSrgba::new(1.0, 0.5, 0.5, 0.8) });
    let timing_method = Some(TimingMethod::RealTime);
    let height = 100;
    let color_override = None;
    let show_gradient = false;
    let digits_format = livesplit_core::timing::formatter::DigitsFormat::SingleDigitSeconds;
    let accuracy = livesplit_core::timing::formatter::Accuracy::Hundredths;

    Settings {
      background,
      timing_method,
      height,
      color_override,
      show_gradient,
      digits_format,
      accuracy,
    }
  }
}