use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub APP_URL: String,
    pub APP_NAME: String,
    pub APP_ENV: String,
}
