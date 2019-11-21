use livesplit_core::{SharedTimer, Timer, GeneralLayoutSettings};
use crate::util::parser::SplitParser;

use livesplit_core::parking_lot::RwLock;
use std::sync::{Arc};
use crate::config::Config;
use livesplit_core::TimerPhase::{Running, NotRunning, Paused, Ended};

pub struct State {
  pub timer: SharedTimer,
  pub split_adjust: gtk::Adjustment,
  pub general_layout_settings: GeneralLayoutSettings,
}

impl State {
  pub fn set_split_adjustment(&mut self, adjustment: gtk::Adjustment) {
    self.split_adjust = adjustment;
  }

  pub fn new() -> State {
    let run = SplitParser.load();
    let timer = Arc::new(RwLock::new(Timer::new(run).expect("Run with at least one segment provided")));
    let general_layout_settings: GeneralLayoutSettings = Config::default_config();
    let split_adjust = gtk::Adjustment::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    {
      let shared_timer = timer.clone();
      std::thread::spawn(||{
        let hook = livesplit_hotkey::linux::Hook::new().unwrap();
        hook.register(livesplit_hotkey::KeyCode::AltR, move || {
          let phase = shared_timer.write().current_phase();
          match phase {
            Running => shared_timer.write().split(),
            NotRunning => shared_timer.write().toggle_pause_or_start(),
            Paused => shared_timer.write().toggle_pause_or_start(),
            Ended => shared_timer.write().reset(true),
          }
        }).unwrap();
        std::thread::park();
      });
    }

    State {
      timer,
      split_adjust,
      general_layout_settings,
    }
  }
}
