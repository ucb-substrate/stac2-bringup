use std::{path::Path, time::Duration};

use serde::{Deserialize, Serialize};

pub const CONFIG_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../Stac.toml");

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClkSel {
    External,
    Fpga,
}

fn default_timeout() -> Duration {
    Duration::from_millis(500)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub fpga_com_port: String,
    pub stac_com_port: String,
    pub clk_sel: ClkSel,
    #[serde(with = "humantime_serde")]
    #[serde(default = "default_timeout")]
    pub timeout: Duration,
}

pub fn load_config(path: impl AsRef<Path>) -> Config {
    toml::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

impl Default for Config {
    fn default() -> Self {
        load_config(CONFIG_PATH)
    }
}
