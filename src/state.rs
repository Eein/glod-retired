use livesplit_core::{Timer, GeneralLayoutSettings};
use crate::util::parser::SplitParser;

use super::config::Config;

pub struct State {
  pub timer: Timer,
  pub general_layout_settings: GeneralLayoutSettings,
}

impl State {
  pub fn new() -> State {
    let run = SplitParser.load();
    let timer = Timer::new(run).expect("Run with at least one segment provided");
    let general_layout_settings: GeneralLayoutSettings = Config::default_config();

    State {
      timer,
      general_layout_settings,
    }
  }
}