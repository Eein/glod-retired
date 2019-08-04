use livesplit_core::component::splits::{
  Component, 
  Settings, 
  ColumnSettings, 
  ColumnStartWith, 
  ColumnUpdateWith,
  ColumnUpdateTrigger,
};
use livesplit_core::settings::{ListGradient::Same, Gradient::Plain, Color};
use livesplit_core::palette::LinSrgba;
use gtk::*;

use crate::formatters::timespan::TimeSpanFormatter;
use crate::state::State;

pub struct Splits {
  component: Component,
  settings: Settings,
  pub widget: gtk::Box,
}

// https://docs.rs/livesplit-core/0.11.0/livesplit_core/component/splits/index.html


// Things to try:
// selecting the current widget by index and running an update against ath
// ie update_widget(idx)
impl Splits {
  pub fn new(state: &State) -> Splits {
    let settings = Splits::default_settings();
    let mut component = Component::with_settings(settings.clone());
    let widget = Splits::widget(&mut component, &state);
    Splits {
      settings,
      component,
      widget,
    }
  }

  pub fn redraw(&mut self, state: &State) {
    // Look into possible removing the destroy step in 
    // favor of changing the nested widgets
    self.widget.foreach(|c| {
      c.destroy();
    });

    for s in &self.component.state(&state.timer.read(), &state.general_layout_settings).splits {
      // -- css --
      let split = gtk::Box::new(Orientation::Horizontal, 0);
      if s.is_current_split == true {
        split.get_style_context().add_class("current-split");
      }else{
        if s.index % 2 == 0 {
          split.get_style_context().add_class("even");
        } else {
          split.get_style_context().add_class("odd");
        }
      }
      split.get_style_context().add_class("split-container");
      // -- fields --
      let name = gtk::Label::new(None);
      name.set_text(&s.name);
      split.add(&name);

      // Render each ColumnState
      for c in s.columns.iter() {
        // live segment delta

        let column = gtk::Label::new(None);
        column.set_text(&c.value);
        &split.pack_end(&column, false, false, 5);

        // TODO: For deltas, feels like theres a better alternative...
        if let Some(split_index) = &state.timer.read().current_split_index() {
          let delta_label = gtk::Label::new(None);
          if s.is_current_split {
            let delta = livesplit_core::analysis::state_helper::live_segment_delta(
              &state.timer.read(),
              s.index,
              "Personal Best",
              livesplit_core::TimingMethod::RealTime
            );
            let formatted = TimeSpanFormatter::to_delta(delta);
            delta_label.set_text(&formatted);
          } else if &s.index < split_index {
            let delta = livesplit_core::analysis::state_helper::previous_segment_delta(
              &state.timer.read(),
              s.index,
              "Personal Best",
              livesplit_core::TimingMethod::RealTime
            );
            let formatted = TimeSpanFormatter::to_delta(delta);
            delta_label.set_text(&formatted);
          }
          &split.pack_end(&delta_label, false, false, 5);
        }
      }

      self.widget.add(&split);
      self.widget.show_all();
    }

  }

  pub fn widget(component: &mut Component, state: &State) -> gtk::Box {
    let container = gtk::Box::new(Orientation::Vertical, 0);
    gtk::WidgetExt::set_name(&container, "splits-container");
    container.get_style_context().add_class("splits-container");

    for s in &component.state(&state.timer.read(), &state.general_layout_settings).splits {
      // -- css --
      // -- fields --
      let name = gtk::Label::new(None);
      name.set_text(&s.name);

      let split = gtk::Box::new(Orientation::Horizontal, 0);
      if s.is_current_split == true {
        split.get_style_context().add_class("current-split");
      }
      if s.index % 2 == 0 {
        split.get_style_context().add_class("even");
      } else {
        split.get_style_context().add_class("odd");
      }

      split.add(&name);


      // Render each ColumnState
      for c in s.columns.iter() {
        let column = gtk::Label::new(None);
        column.set_text(&c.value);
        &split.pack_end(&column, false, false, 5);
      }

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
    let columns: Vec<ColumnSettings> = vec![
      ColumnSettings { 
        name: String::from("Personal Best"),
        start_with: ColumnStartWith::ComparisonTime,
        update_with: ColumnUpdateWith::SplitTime,
        update_trigger: ColumnUpdateTrigger::Contextual,
        comparison_override: None,
        timing_method: None,
      }
    ];


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