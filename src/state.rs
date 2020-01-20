use livesplit_core::{SharedTimer, Timer, GeneralLayoutSettings, HotkeyConfig, HotkeySystem};
use crate::util::parser::SplitParser;

use livesplit_core::parking_lot::RwLock;
use std::sync::{Arc};
use crate::config::Config;
use livesplit_hotkey::linux::KeyCode;


pub struct State {
  pub timer: SharedTimer,
  pub hotkeys: HotkeySystem,
  pub general_layout_settings: GeneralLayoutSettings,
}

impl State {

  pub fn new() -> State {
    let run = SplitParser.load();
    let timer: SharedTimer = Arc::new(RwLock::new(Timer::new(run).expect("Run with at least one segment provided")));

    let hotkeys = HotkeySystem::with_config(timer.clone(), State::hotkey_config())
      .expect("hotkeys could not register");

    let general_layout_settings: GeneralLayoutSettings = Config::default_config();

    State {
      timer,
      hotkeys,
      general_layout_settings,
    }
  }

  pub fn hotkey_config() -> HotkeyConfig {
    HotkeyConfig {
      split: Some(KeyCode::AltR),
      reset: Some(KeyCode::BackSpace),
      undo: Some(KeyCode::Up),
      skip: Some(KeyCode::Down),
      pause: Some(KeyCode::Pause),
      undo_all_pauses: Some(KeyCode::SuperL),
      previous_comparison: Some(KeyCode::Left),
      next_comparison: Some(KeyCode::Right),
      toggle_timing_method: Some(KeyCode::Plus),
    }
  }
}
