use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    delay: u32,
    delta: u32,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            delay: 1,
            delta: 12,
        }
    }
}

fn main() {
    let config_path = config_dir().unwrap().join(".riggler");
    let config_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(config_path)
        .unwrap();

    let result = serde_json::to_writer_pretty(config_file, &Config::default());
    println!("{:?}", result);
}
