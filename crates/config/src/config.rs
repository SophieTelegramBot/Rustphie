use envy::{prefixed, Result};
use serde::Deserialize;

/// Contains bot configurations
///
/// # Example
/// ```
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

#[cfg(test)]
mod tests {
    use envy::Result;
    use crate::Configuration;

    #[test]
    fn load_config_test() -> Result<()> {
        let token = "123";
        std::env::set_var("BOT_TOKEN", token.clone());
        let config = Configuration::load_config()?;
        Ok(assert_eq!(config.token, token.to_string()))
    }
}
