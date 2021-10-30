use livesplit_core::component::title::{Component, Settings};
use livesplit_core::settings::{Alignment::Auto, Gradient::Plain, Color};
use gtk::*;

use crate::state::State;

pub struct Title {
  component: Component,
}

// https://docs.rs/livesplit-core/0.11.0/livesplit_core/component/title/index.html

impl Title {
  pub fn new() -> Title {
    let settings = Title::default_settings();
    let component = Component::with_settings(settings.clone());
    Title {
      component,
    }
  }

  pub fn widget(&mut self, state: &State) -> gtk::Box {
    let container = gtk::Box::new(Orientation::Horizontal, 0);
    let title = gtk::Box::new(Orientation::Vertical, 0);

    container.pack_start(&title, true, false, 0);

    // let line_1 = gtk::Label::new(None);
    // line_1.set_text(&self.component.state(&state.timer.read()).line1);
    // title.add(&line_1);

    // if let Some(w) = &self.component.state(&state.timer.read().snapshot()).line2 {
    //   let line_2 = gtk::Label::new(Some(w));
    //   title.add(&line_2);
    // }

    // if let Some(finished_runs) = &self.component.state(&state.timer.read()).finished_runs {
    //   let runs = gtk::Label::new(Some(&finished_runs.to_string()));
    //   container.pack_end(&runs, false, false, 5);
    // }

    // if let Some(attempts) = &self.component.state(&state.timer.read()).attempts {
    //   let runs = gtk::Label::new(Some(&attempts.to_string()));
    //   container.pack_end(&runs, false, false, 5);
    // }
    container
  }

  fn default_settings() -> Settings {
    Settings {
        ..Default::default()
    }
  }
}
