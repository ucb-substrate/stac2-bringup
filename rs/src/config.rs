use serde::{Deserialize, Serialize};

const CONFIG_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../Stac.toml");

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub fpga_com_port: String,
    pub stac_com_port: String,
}

pub fn load_config() -> Config {
    toml::from_str(&std::fs::read_to_string(CONFIG_PATH).unwrap()).unwrap()
}
