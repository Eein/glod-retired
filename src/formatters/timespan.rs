use livesplit_core::{TimeSpan};
use super::duration::DurationFormatter;

pub struct TimeSpanFormatter;
impl TimeSpanFormatter {
  pub fn to_short(timespan: Option<TimeSpan>) -> String {
    match timespan {
      Some(timespan) => {
        let duration = timespan.to_duration();
        DurationFormatter::to_short(duration)
      },
      None => String::from(""),
    }
  }
  pub fn to_string(timespan: Option<TimeSpan>) -> String {
    match timespan {
      Some(timespan) => {
        let duration = timespan.to_duration();
        DurationFormatter::to_string(duration)
      },
      None => String::from(""),
    }
  }
  pub fn to_delta(timespan: Option<TimeSpan>) -> String {
    match timespan {
      Some(timespan) => {
        let duration = timespan.to_duration();
        let mut formatted = DurationFormatter::to_string(duration);
        let len = formatted.len();
        formatted.truncate(len - 2);
        formatted
      },
      None => String::from(""),
    }
  }
}