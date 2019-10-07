extern crate gtk;
extern crate glib;

mod app;
mod config;
mod state;
mod util;
mod components;
mod formatters;

use gtk::*;
use std::sync::{Arc};

use app::App;

fn main() {
    if gtk::init().is_err() { eprintln!("Failed to initialize glod"); }
    let app = Arc::new(App::new());

    {
      let c_app = app.clone();
      app.test_button.clone().connect_clicked(move |_| {
        c_app.clone().state.timer.write().toggle_pause_or_start();
      });
    }

    {
      let c_app = app.clone();
      app.split_button.clone().connect_clicked(move |_| {
        c_app.clone().state.timer.write().split();
      });
    }

    {
      let mut callback_storage = bindkey::CallbackStorage::new();
      let hotkey = bindkey::HotKey {
        key: 65514, // alt r
        modifiers: vec![],
        trigger: bindkey::TriggerOn::Press,
      };

      callback_storage.add(&hotkey, || println!("CALLED"));
      bindkey::start_async(callback_storage);
    }

    app.window.show_all();

    {
        let c_app = app.clone();
        let tick = move || {
          c_app.splits.write().redraw(&c_app.state);
          c_app.blank_space.write().redraw(&c_app.state);
          c_app.timer.write().redraw(&c_app.state);
          c_app.total_playtime.write().redraw(&c_app.state);
          c_app.current_pace.write().redraw(&c_app.state);
          c_app.delta.write().redraw(&c_app.state);
          c_app.possible_time_save.write().redraw(&c_app.state);
          c_app.previous_segment.write().redraw(&c_app.state);
          c_app.sum_of_best.write().redraw(&c_app.state);
          Continue(true)
        };
        timeout_add(16, tick);
    }

    gtk::main()
}
