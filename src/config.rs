use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};

const CONFIG_FILE: &str = "settings.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub potion_key: String,
    pub time_key: u8,
    pub window_pos: Option<(f32, f32)>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            potion_key: "3".to_string(),
            time_key: 15,
            window_pos: None,
        }
    }
}

impl AppConfig {
    pub fn load() -> Self {
        match File::open(CONFIG_FILE) {
            Ok(file) => {
                let reader = BufReader::new(file);
                serde_json::from_reader(reader).unwrap_or_default()
            }
            Err(_) => Self::default(),
        }
    }
    pub fn save(&self) -> std::io::Result<()> {
        let file = File::create(CONFIG_FILE)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }
}
