use livesplit_core::component::splits::{Component, Settings, ColumnSettings};
use livesplit_core::settings::{ListGradient::Same, Gradient::Plain, Color};
use livesplit_core::palette::LinSrgba;
use gtk::*;

use crate::state::State;

pub struct Splits {
  component: Component,
  settings: Settings,
}

// https://docs.rs/livesplit-core/0.11.0/livesplit_core/component/splits/index.html

impl Splits {
  pub fn new() -> Splits {
    let settings = Splits::default_settings();
    let component = Component::with_settings(settings.clone());
    Splits {
      settings,
      component,
    }
  }

  pub fn widget(&mut self, state: &State) -> gtk::Box {
    let container = gtk::Box::new(Orientation::Vertical, 0);
    gtk::WidgetExt::set_name(&container, "splits-container");
    container.get_style_context().add_class("splits-container");

    for s in &self.component.state(&state.timer.read(), &state.general_layout_settings).splits {
      // -- css --
      let split = gtk::Box::new(Orientation::Horizontal, 0);
      if s.is_current_split == true {
        println!("is current split");
        split.get_style_context().add_class("current-split");
      }
      if s.index % 2 == 0 {
        split.get_style_context().add_class("even");
      } else {
        split.get_style_context().add_class("odd");
      }
      // -- fields --
      let name = gtk::Label::new(None);
      name.set_text(&s.name);

      split.add(&name);

      container.pack_start(&split, false, false, 0);
    }


    container
  }

  fn default_settings() -> Settings {
    let background = Same(Plain(Color { rgba: LinSrgba::new(1.0, 0.5, 0.5, 0.8) }));
    let visual_split_count = 0;
    let split_preview_count = 0;
    let show_thin_separators = true;
    let separator_last_split = true;
    let always_show_last_split = true;
    let fill_with_blank_space = true;
    let display_two_rows = false;
    let current_split_gradient = Plain(Color { rgba: LinSrgba::new(1.0, 0.5, 0.5, 0.8) });
    let show_column_labels = true;
    let columns: Vec<ColumnSettings> = Vec::new();


    Settings {
      background,
      visual_split_count,
      split_preview_count,
      show_thin_separators,
      separator_last_split,
      always_show_last_split,
      fill_with_blank_space,
      display_two_rows,
      current_split_gradient,
      show_column_labels,
      columns,
    }
  }
}