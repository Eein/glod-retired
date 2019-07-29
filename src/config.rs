use livesplit_core::layout::LayoutDirection;
use livesplit_core::settings::Color;
use livesplit_core::palette::LinSrgba;
use livesplit_core::settings::Gradient::Plain;
use livesplit_core::{GeneralLayoutSettings};

pub struct Config;
impl Config {
  pub fn default_config() -> GeneralLayoutSettings {
    let direction = LayoutDirection::Vertical;
    let background = Plain(Color { rgba: LinSrgba::new(1.0, 0.5, 0.5, 0.8) });
    let best_segment_color = Color { rgba: LinSrgba::new(1.0, 0.5, 0.5, 0.8) };
    let ahead_gaining_time_color = Color { rgba: LinSrgba::new(1.0, 0.5, 0.5, 0.8) };
    let ahead_losing_time_color = Color { rgba: LinSrgba::new(1.0, 0.5, 0.5, 0.8) };
    let behind_gaining_time_color = Color { rgba: LinSrgba::new(1.0, 0.5, 0.5, 0.8) };
    let behind_losing_time_color = Color { rgba: LinSrgba::new(1.0, 0.5, 0.5, 0.8) };
    let not_running_color = Color { rgba: LinSrgba::new(1.0, 0.5, 0.5, 0.8) };
    let personal_best_color = Color { rgba: LinSrgba::new(1.0, 0.5, 0.5, 0.8) };
    let paused_color = Color { rgba: LinSrgba::new(1.0, 0.5, 0.5, 0.8) };
    let thin_separators_color = Color { rgba: LinSrgba::new(1.0, 0.5, 0.5, 0.8) };
    let separators_color = Color { rgba: LinSrgba::new(1.0, 0.5, 0.5, 0.8) };
    let text_color = Color { rgba: LinSrgba::new(1.0, 0.5, 0.5, 0.8) };

    GeneralLayoutSettings {
      direction,
      background,
      best_segment_color,
      ahead_gaining_time_color,
      ahead_losing_time_color,
      behind_gaining_time_color,
      behind_losing_time_color,
      not_running_color,
      personal_best_color,
      paused_color,
      thin_separators_color,
      separators_color,
      text_color,
    }
  }
}