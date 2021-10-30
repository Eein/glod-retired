use livesplit_core::GeneralLayoutSettings;

pub struct Config;
impl Config {
  pub fn default_config() -> GeneralLayoutSettings {
    GeneralLayoutSettings {
        ..Default::default()
    }
  }
}
