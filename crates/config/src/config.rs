use envy::{prefixed, Result};
use serde::Deserialize;

/// Contains bot configurations
///
/// # Example
/// ```ignore
/// # use config::Configuration;
/// # use std::env::set_var;
/// # fn main() {
/// // set env vars; env vars should be prefixed with `BOT`
/// set_var("BOT_TOKEN", "123");
/// let conf = Configuration::load_config().unwrap();
/// assert_eq!(conf.token, "123".to_string());
/// # }
/// ```
#[derive(Deserialize, Debug)]
pub struct Configuration {
    /// Telegram bot API token
    pub token: String,
    /// Bot username
    pub username: String,
}

impl Configuration {
    /// Loads the environmental variables into struct.
    pub fn load_config() -> Result<Self> {
        let config = prefixed("BOT_").from_env::<Configuration>()?;
        Ok(config)
    }
}
