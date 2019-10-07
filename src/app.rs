use crate::state::State;
use crate::components::title::Title;
use crate::components::splits::Splits;
use crate::components::timer::Timer;
use crate::components::total_playtime::TotalPlaytime;
use crate::components::current_pace::CurrentPace;
use crate::components::possible_time_save::PossibleTimeSave;
use crate::components::previous_segment::PreviousSegment;
use crate::components::sum_of_best::SumOfBest;
use crate::components::delta::Delta;
use crate::components::blank_space::BlankSpace;
use gtk::*;
use std::sync::{Arc};
use livesplit_core::parking_lot::RwLock;

const CSS: &str = include_str!("styles/app.css");

pub struct App{
  pub state: State,
  pub window: Window,
  pub title: gtk::Box,
  pub splits: Arc<RwLock<Splits>>,
  pub timer: Arc<RwLock<Timer>>,
  pub total_playtime: Arc<RwLock<TotalPlaytime>>,
  pub current_pace: Arc<RwLock<CurrentPace>>,
  pub possible_time_save: Arc<RwLock<PossibleTimeSave>>,
  pub previous_segment: Arc<RwLock<PreviousSegment>>,
  pub sum_of_best: Arc<RwLock<SumOfBest>>,
  pub delta: Arc<RwLock<Delta>>,
  pub blank_space: Arc<RwLock<BlankSpace>>,
  pub test_button: gtk::Button,
  pub split_button: gtk::Button,
}

impl App {

  pub fn new() -> App {
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_default_size(300, 500);

    let screen = window.get_screen().unwrap();
    let style = CssProvider::new();
    let _ = CssProviderExt::load_from_data(&style, CSS.as_bytes());
    StyleContext::add_provider_for_screen(&screen, &style, STYLE_PROVIDER_PRIORITY_USER);
    let state = State::new();

    let container = gtk::Box::new(Orientation::Vertical, 0);
    let title = Title::new().widget(&state);
    let splits = Arc::new(RwLock::new(Splits::new(&state)));
    let timer = Arc::new(RwLock::new(Timer::new(&state)));
    let blank_space = Arc::new(RwLock::new(BlankSpace::new(&state)));
    let total_playtime = Arc::new(RwLock::new(TotalPlaytime::new(&state)));
    let current_pace = Arc::new(RwLock::new(CurrentPace::new(&state)));
    let delta = Arc::new(RwLock::new(Delta::new(&state)));
    let possible_time_save = Arc::new(RwLock::new(PossibleTimeSave::new(&state)));
    let previous_segment = Arc::new(RwLock::new(PreviousSegment::new(&state)));
    let sum_of_best = Arc::new(RwLock::new(SumOfBest::new(&state)));
    let test_button = gtk::Button::new_with_label("START/PAUSE");
    let split_button = gtk::Button::new_with_label("SPLIT");

    container.add(&title);
    container.add(&splits.read().widget);
    container.add(&blank_space.read().widget);
    container.add(&total_playtime.read().widget);
    container.add(&current_pace.read().widget);
    container.add(&delta.read().widget);
    container.add(&possible_time_save.read().widget);
    container.add(&previous_segment.read().widget);
    container.add(&sum_of_best.read().widget);
    container.add(&timer.read().widget);
    container.add(&test_button);
    container.add(&split_button);

    window.add(&container);

    App {
      state,
      window,
      title,
      splits,
      timer,
      total_playtime,
      current_pace,
      delta,
      possible_time_save,
      previous_segment,
      blank_space,
      sum_of_best,
      test_button,
      split_button,
    }
  }
}