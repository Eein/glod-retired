use livesplit_core::component::title::{Component, Settings, State}
use livesplit_core::settings::{Alignment::Auto, Gradient::Plain};
use livesplit_core::palette::LinSrgba;

struct Title {
  component: Component,
  settings: Settings,
}

impl Title {
  fn new() -> Title {
    let settings = Title::default_settings();
    let component = Component.with_settings();
    Title {
      settings,
      component,
    }
  }

  fn default_settings() -> Settings {
    let background = Plain(Color { rgba: LinSrgba::new(1.0, 0.5, 0.5, 0.8) });
    let text_color = None;
    let show_game_name = true;
    let show_category_name = true;
    let show_finished_runs_count = true;
    let show_attempt_count = true;
    let text_alignment = Auto;
    let display_as_single_line = true;
    let display_game_icon = true;
    let show_region = true;
    let show_platform = true;
    let show_variables = false;

    Settings {
      background,
      text_color,
      show_game_name,
      show_category_name,
      show_finished_runs_count,
      show_attempt_count,
      text_alignment,
      display_as_single_line,
      display_game_icon,
      show_region,
      show_platform,
      show_variables,
    }
  }
}