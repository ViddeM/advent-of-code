use std::env;

pub struct Config {
    pub url: String,
    pub session: String,
    pub base_download_path: String,
}

impl Config {
    pub fn new() -> Config {
        dotenv::dotenv().ok();
        Config {
            url: env::var("url").expect("No url specified in env variables"),
            session: env::var("session").expect("No session specified in env variables"),
            base_download_path: env::var("base_download_path").expect("No base download path specified in env variables"),
        }
    }
}