extern crate gtk;
extern crate glib;

mod app;
mod config;
mod state;
mod util;
mod components;
mod formatters;

use gtk::*;
use std::sync::{Arc, Mutex};

use app::App;

fn main() {
		if gtk::init().is_err() { eprintln!("Failed to initialize glod"); }
		let app = Arc::new(Mutex::new(App::new()));

		{
			let shared_app = app.clone();

			app.lock().unwrap().test_button.connect_clicked(move |_| {
				shared_app.lock().unwrap().state.timer.write().toggle_pause_or_start();
			});
		}

		{
			let shared_app = app.clone();

			app.lock().unwrap().split_button.connect_clicked(move |_| {
				shared_app.lock().unwrap().state.timer.write().split();
			});
		}

		app.lock().unwrap().window.show_all();

		{
				let shared_app = app.clone();
				let tick = move || {
					let c_app = shared_app.lock().unwrap();
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
