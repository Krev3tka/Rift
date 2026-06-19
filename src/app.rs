pub struct App {
    pub config: Config,
    pub username_input: String,
    pub masterkey_input: String,
}

impl App {
    pub fn new() -> App {
        App {
            config: Config::default(),
            username_input: String::new(),
            masterkey_input: String::new(),
        }
    }
}

pub struct Config {
    pub url: String,
    pub timeout_seconds: u64
}

impl Default for Config {
    fn default() -> Self {
        Self {
            url: "https://2.26.28.51:8080/api/v1/".to_string(),
            timeout_seconds: 15
        }
    }
}