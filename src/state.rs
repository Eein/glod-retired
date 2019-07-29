use livesplit_core::{SharedTimer, Timer, GeneralLayoutSettings};
use crate::util::parser::SplitParser;

use livesplit_core::parking_lot::RwLock;
use std::sync::{Arc};
use crate::config::Config;

pub struct State {
  pub timer: SharedTimer,
  pub general_layout_settings: GeneralLayoutSettings,
}

impl State {
  pub fn new() -> State {
    let run = SplitParser.load();
    let timer = Arc::new(RwLock::new(Timer::new(run).expect("Run with at least one segment provided")));
    let general_layout_settings: GeneralLayoutSettings = Config::default_config();

    State {
      timer,
      general_layout_settings,
    }
  }
}