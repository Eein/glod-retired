extern crate gtk;
extern crate glib;

mod app;
mod config;
mod state;
mod util;
mod components;

use gtk::*;
use std::sync::{Arc};

use app::App;
fn main() {
    if gtk::init().is_err() { eprintln!("Failed to initialize glod"); }
    let app = Arc::new(App::new());

    let c_app = app.clone();
    {
      app.test_button.clone().connect_clicked(move |_| {
        c_app.clone().state.timer.write().start();
        println!("TIMER STARTED");
        if let Some(time) = c_app.state.timer.read().current_time().real_time {
          println!("ms: {}", time.total_milliseconds());
        }
      });
    }

    app.window.show_all();
    // let tick = || { 
    //   Continue(true)
    // };
    // glib::source::timeout_add(1, yeehaw);

    gtk::main()
}
