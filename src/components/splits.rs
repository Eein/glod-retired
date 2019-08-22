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
use gtk::{
  BoxExt,
  ContainerExt,
  LabelExt,
  WidgetExt,
  StyleContextExt,
  Viewport,
  ViewportExt,
  ScrolledWindow,
  ScrolledWindowExt,
  Cast,
  Orientation,
  BinExt,
  NONE_ADJUSTMENT,
};

use crate::formatters::timespan::TimeSpanFormatter;
use crate::state::State;

pub struct Splits {
  component: Component,
  settings: Settings,
  pub widget: gtk::ScrolledWindow,
}

// https://docs.rs/livesplit-core/0.11.0/livesplit_core/component/splits/index.html


// Things to try:
// selecting the current widget by index and running an update against ath

// TODO: create an update_split method that is iterated on by 
// ContainerExt.foreach(|c| {}). This should update the times present
// in the state without destroying the times. After this, also revise the timer
// to not destroy and update its time internally as well.
// Performance may vary by update vs delete and create - but worth trying
// updates first as its non-desctructive and shouldn't destroy scrolling behavior.

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
    // This is nasty here, need to find out how to select through
    if let Some(vp) = self.widget.get_child() {
      if let Some(viewbox) = vp.downcast_ref::<gtk::Viewport>() {
        if let Some(child) = viewbox.get_child() {
          child.destroy();
        }
      }
      // vp.destroy();
    }

    let container = gtk::Box::new(Orientation::Vertical, 0);
    gtk::WidgetExt::set_name(&container, "splits-container");
    container.get_style_context().add_class("splits-container");

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

      container.add(&split);
    }

    if let Some(vp) = self.widget.get_child() {
      if let Some(viewbox) = vp.downcast_ref::<gtk::Viewport>() {
          viewbox.add(&container);
      }
    }
    self.widget.show_all();
  }

  pub fn widget(component: &mut Component, state: &State) -> gtk::ScrolledWindow {
    let window = ScrolledWindow::new(NONE_ADJUSTMENT, NONE_ADJUSTMENT);
    window.set_propagate_natural_height(true);
    window.set_overlay_scrolling(true);

    let viewport = Viewport::new(NONE_ADJUSTMENT, NONE_ADJUSTMENT);
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
      split.get_style_context().add_class("split-container");

      split.add(&name);


      // Render each ColumnState
      for c in s.columns.iter() {
        let column = gtk::Label::new(None);
        column.set_text(&c.value);
        &split.pack_end(&column, false, false, 5);
      }

      container.pack_start(&split, false, false, 0);
    }
    viewport.add(&container);
    window.add(&viewport);
    window
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