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
  pub widget: gtk::Box,
}

impl Timer {
  pub fn new(state: &State) -> Timer {
    let settings = Timer::default_settings();
    let mut component = Component::with_settings(settings.clone());
    let widget = Timer::widget(&mut component, &state);
    Timer {
      settings,
      component,
      widget,
    }
  }

  pub fn redraw(&mut self, state: &State) {
    self.widget.foreach(|c| {
      c.destroy();
    });

    self.widget.get_style_context().add_class("timer-container");
    
    let seconds_label = gtk::Label::new(None);
    let fraction_label = gtk::Label::new(None);
    let timer = &self.component.state(&state.timer.read(), &state.general_layout_settings);
    let time = &timer.time;
    let fraction = &timer.fraction;
    // This can be formatted into two configurable sizes like Livesplit
    seconds_label.set_text(&time);
    seconds_label.get_style_context().add_class("seconds");
    seconds_label.set_valign(Align::End);

    fraction_label.set_text(&fraction);
    fraction_label.get_style_context().add_class("fraction");
    fraction_label.set_valign(Align::End);

    self.widget.pack_end(&fraction_label, false, false, 0);
    self.widget.pack_end(&seconds_label, false, false, 0);
    self.widget.show_all();
  }

  pub fn widget(component: &mut Component, state: &State) -> gtk::Box {
    let container = gtk::Box::new(Orientation::Horizontal, 0);

    gtk::WidgetExt::set_name(&container, "timer-container");
    container.get_style_context().add_class("timer-container");

    let label = gtk::Label::new(None);
    let timer = &component.state(&state.timer.read(), &state.general_layout_settings);
    let time = &timer.time;
    let fraction = &timer.fraction;
    // This can be formatted into two configurable sizes like Livesplit
    let formatted = format!("{}{}", time, fraction);

    label.set_text(&formatted);
    container.add(&label);

    container
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