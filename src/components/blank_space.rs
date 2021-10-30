use livesplit_core::component::blank_space::{
  Settings,
};
use gtk::*;

use crate::state::State;

pub struct BlankSpace {
  pub widget: gtk::Box,
}

impl BlankSpace {
  pub fn new(_state: &State) -> BlankSpace {
    let widget = gtk::Box::new(Orientation::Horizontal, 0);
    widget.get_style_context().add_class("blank-space-container");
    widget.set_hexpand(true);
    let blank = gtk::Label::new(None);
    blank.set_text(" ");
    widget.add(&blank);

    BlankSpace {
      widget,
    }
  }

  pub fn redraw(&mut self, _state: &State) {
    // nothing required here
  }

  #[allow(dead_code)]
  fn default_settings() -> Settings {
    Settings {
        ..Default::default()
    }
  }
}
