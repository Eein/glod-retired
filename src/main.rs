extern crate gtk;

mod app;
mod config;
mod state;
mod util;
mod components;

use gtk::{
  WidgetExt,
};

use app::App;
fn main() {
    if gtk::init().is_err() { eprintln!("Failed to initialize glod"); }

    let app = App::new();
    app.window.show_all();
    gtk::main()
}
