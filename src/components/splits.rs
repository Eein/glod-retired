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
use livesplit_core::palette::LinSrgba;
use gtk::{
  ContainerExt,
  LabelExt,
  WidgetExt,
  StyleContextExt,
  Viewport,
  ScrolledWindow,
  ScrolledWindowExt,
  Orientation,
  NONE_ADJUSTMENT,
};

use crate::state::State;

pub struct Splits {
  component: Component,
  split_rows: Vec<gtk::Box>,
  split_columns: Vec<Vec<gtk::Label>>, // indexes = [row][column]
  pub widget: gtk::ScrolledWindow,
}

// https://docs.rs/livesplit-core/0.11.0/livesplit_core/component/splits/index.html

impl Splits {
  pub fn new(state: &State) -> Splits {
    let settings = Splits::default_settings();
    let mut component = Component::with_settings(settings.clone());
    let split_columns = Splits::init_split_columns(&mut component, &state);
    let split_names = Splits::init_split_names(&mut component, &state);
    let split_rows = Splits::init_split_rows(&mut component, &state);

    let widget = ScrolledWindow::new(NONE_ADJUSTMENT, Some(&state.split_adjust));
    widget.set_min_content_width(300);
    widget.set_overlay_scrolling(true);
    widget.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
    widget.set_hexpand(true);
    widget.set_vexpand(true);

    let viewport = Viewport::new(NONE_ADJUSTMENT, NONE_ADJUSTMENT);
    let container = gtk::Box::new(Orientation::Vertical, 0);
    gtk::WidgetExt::set_name(&container, "splits-container");
    container.get_style_context().add_class("splits-container");

    for (index, r) in split_rows.iter().enumerate() {

      if index % 2 == 0 {
        r.get_style_context().add_class("even");
      } else {
        r.get_style_context().add_class("odd");
      }

      let name = &split_names[index];
      let columns = &split_columns[index];

      r.add(name);

      for c in columns {
        r.add(c);
      }
      container.add(r);
    }

    viewport.add(&container);
    widget.add(&viewport);

    Splits {
      component,
      widget,
      split_rows,
      split_columns,
    }
  }

  pub fn init_split_rows(component: &mut Component, state: &State) -> Vec<gtk::Box> {
    let mut rows: Vec<gtk::Box> = Vec::new();
    for _s in &component.state(&state.timer.read(), &state.general_layout_settings).splits {
      let row = gtk::Box::new(Orientation::Horizontal, 0);
      row.get_style_context().add_class("split-container");
      row.set_hexpand(true);
      rows.push(row);
    }
    rows
  }

  pub fn init_split_names(component: &mut Component, state: &State) -> Vec<gtk::Label> {
    let mut names: Vec<gtk::Label> = vec![];
    for s in &component.state(&state.timer.read(), &state.general_layout_settings).splits {
      let name = gtk::Label::new(None);
      name.set_text(&s.name);
      name.set_halign(gtk::Align::Start);
      name.set_hexpand(true);
      names.push(name);
    }
    names
  }

  pub fn init_split_columns(component: &mut Component, state: &State) -> Vec<Vec<gtk::Label>> {
    let mut rows: Vec<Vec<gtk::Label>> = vec![];
    for s in &component.state(&state.timer.read(), &state.general_layout_settings).splits {
      let mut columns: Vec<gtk::Label> = vec![];
      for c in s.columns.iter() {
        let label = gtk::Label::new(None);
        label.set_text(&c.value);
        label.get_style_context().add_class("column");
        columns.push(label);
      }
      rows.insert(s.index as usize, columns);
    }


    rows
  }

  pub fn redraw(&mut self, state: &State) {
    if state.timer.read().current_phase() == livesplit_core::TimerPhase::Running {
      let current_split_index = state.timer.read().current_split_index().unwrap();
      for s in self.component.state(&state.timer.read(), &state.general_layout_settings).splits {
        &self.split_rows[s.index].get_style_context().remove_class("current-split");
        if s.index == current_split_index {
          &self.split_rows[s.index].get_style_context().add_class("current-split");
        }
        let columns = &self.split_columns[s.index];
        for (index, c) in s.columns.iter().enumerate() {
          let value = &c.value;
          // there has to be a better way...
          columns[index].get_style_context().remove_class("default");
          columns[index].get_style_context().remove_class("ahead-gaining-time");
          columns[index].get_style_context().remove_class("ahead-losing-time");
          columns[index].get_style_context().remove_class("behind-losing-time");
          columns[index].get_style_context().remove_class("behind-gaining-time");
          columns[index].get_style_context().remove_class("best-segment");
          columns[index].get_style_context().remove_class("not-running");
          columns[index].get_style_context().remove_class("paused");
          columns[index].get_style_context().remove_class("personal-best");
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
          columns[index].set_text(&value);
          columns[index].get_style_context().add_class(semantic_color);
          columns[index].set_halign(gtk::Align::End);
        }
      }
      // checking width allocation to make deltas look nice
      let mut width = -1;
      for s in 0..self.split_rows.len() - 1 {
        for c in &self.split_columns[s] {
          let column_width = c.get_allocated_width();
          if  column_width > width {
            width = column_width;
          }
        }
      }
      for s in 0..self.split_rows.len() - 1 {
        for c in &self.split_columns[s] {
          c.set_xalign(width as f32);
          c.set_size_request(width, -1);
        }
      }
      self.widget.set_vadjustment(&state.split_adjust);
      self.widget.show_all();
    }
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
        name: String::from("+/-"),
        start_with: ColumnStartWith::Empty,
        update_with: ColumnUpdateWith::Delta,
        update_trigger: ColumnUpdateTrigger::Contextual,
        comparison_override: None,
        timing_method: None,
      },
      ColumnSettings {
        name: String::from("Time"),
        start_with: ColumnStartWith::ComparisonTime,
        update_with: ColumnUpdateWith::SplitTime,
        update_trigger: ColumnUpdateTrigger::OnEndingSegment,
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
