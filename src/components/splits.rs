use livesplit_core::component::splits::{
  Component,
  Settings,
  ColumnSettings,
  ColumnStartWith,
  ColumnUpdateWith,
  ColumnUpdateTrigger,
};
use livesplit_core::settings::{
  SemanticColor,
  ListGradient::Same,
  Gradient::Plain, Color
};
use gtk::{
  ContainerExt,
  LabelExt,
  WidgetExt,
  StyleContextExt,
  Orientation,
};

use crate::state::State;

pub struct Splits {
  component: Component,
  pub widget: gtk::Box,
}

// https://docs.rs/livesplit-core/0.11.0/livesplit_core/component/splits/index.html

impl Splits {
  pub fn new(state: &State) -> Splits {
    let settings = Splits::default_settings();
    let mut component = Component::with_settings(settings.clone());

    let widget = gtk::Box::new(Orientation::Vertical, 0);
    widget.set_hexpand(true);
    widget.set_vexpand(true);
    widget.get_style_context().add_class("splits-container");

    let splits = Splits::draw_splits(&mut component, &state);
    for s in splits {
      widget.add(&s);
    }

    Splits {
      component,
      widget,
    }
  }

  pub fn draw_splits(component: &mut Component, state: &State) -> Vec<gtk::Box> {
    let mut rows: Vec<gtk::Box> = Vec::new();

    for s in &component.state(&state.timer.read().snapshot(), &state.general_layout_settings).splits {
      let row = gtk::Box::new(Orientation::Horizontal, 0);
      row.get_style_context().add_class("split-container");
      row.set_hexpand(true);

      // set row name
      let name = gtk::Label::new(None);
      name.set_text(&s.name);
      name.set_halign(gtk::Align::Start);
      name.set_hexpand(true);
      row.add(&name);

      // create column boxes
      for c in s.columns.iter() {
        let label = gtk::Label::new(None);
        label.get_style_context().add_class("split-column");

        let semantic_color = match c.semantic_color {
          SemanticColor::Default => "default",
          SemanticColor::AheadGainingTime => "ahead-gaining-time",
          SemanticColor::AheadLosingTime => "ahead-losing-time",
          SemanticColor::BehindLosingTime => "behind-losing-time",
          SemanticColor::BehindGainingTime => "behind-gaining-time",
          SemanticColor::BestSegment => "best-segment",
          SemanticColor::NotRunning => "not-running",
          SemanticColor::Paused => "paused",
          SemanticColor::PersonalBest => "personal-best",
        };

        label.set_text(&c.value);
        label.get_style_context().add_class(semantic_color);
        label.set_halign(gtk::Align::End);
        row.add(&label);
      };

      if s.index % 2 == 0 {
        row.get_style_context().add_class("even");
      } else {
        row.get_style_context().add_class("odd");
      }
      if state.timer.read().current_phase() == livesplit_core::TimerPhase::Running {
        let current_split_index = state.timer.read().current_split_index().unwrap();
        if s.index == current_split_index {
          row.get_style_context().add_class("current-split");
        }
      }
      rows.push(row);
    }

    rows
  }

  pub fn redraw(&mut self, state: &State) {
    for c in self.widget.get_children() {
      self.widget.remove(&c);
    }
    let splits = Splits::draw_splits(&mut self.component, &state);
    for s in splits {
      self.widget.add(&s);
    }
    self.widget.show_all();
  }

  fn default_settings() -> Settings {
    Settings {
        ..Default::default()
    }
  }
}
