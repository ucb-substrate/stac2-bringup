use const_format::concatcp;

pub mod bebe;
pub mod config;
pub mod executor;
pub mod memory;
pub mod pattern;
pub mod state;
mod tests;
pub mod tsi;

use ::tsi::Tsi;
pub use bebe::*;
pub use executor::*;
pub use memory::*;
pub use tsi::*;

use crate::config::{Config, load_config};

const PY_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../py");
const BEBE_HOST: &str = concatcp!(PY_DIR, "/bebe_host.py");
const DEFAULT_HALF_CLK_DIV_RATIO: u32 = 1;

pub struct BringupState {
    config: Config,
    pub(crate) tsi: Tsi,
    half_clk_div_ratio: u32,
}

impl Default for BringupState {
    fn default() -> Self {
        let config = load_config();
        let mut tsi = Tsi::new(&config.fpga_com_port, FPGA_BAUD_RATE);
        tsi.write_word(HALF_CLK_DIV_RATIO, DEFAULT_HALF_CLK_DIV_RATIO as u64)
            .expect("failed to write");
        BringupState {
            config,
            tsi,
            half_clk_div_ratio: DEFAULT_HALF_CLK_DIV_RATIO,
        }
    }
}

impl BringupState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_div_ratio(&mut self, half_clk_div_ratio: u32) {
        self.tsi
            .write_word(HALF_CLK_DIV_RATIO, half_clk_div_ratio as u64)
            .expect("failed to write");
        self.half_clk_div_ratio = half_clk_div_ratio;
    }

    pub fn enable_clk(&mut self) {
        self.tsi.write_word(CLK_EN, 1).expect("failed to write");
    }

    pub fn disable_clk(&mut self) {
        self.tsi.write_word(CLK_EN, 0).expect("failed to write");
    }

    pub fn reset_chip(&mut self) {
        self.tsi.write_word(RESET_REG, 1).expect("failed to write");
        self.tsi.write_word(RESET_REG, 0).expect("failed to write");
    }

    pub fn init_chip(&mut self) {
        let mut handle = self.bebe_wait().unwrap();
        self.enable_clk();
        self.reset_chip();
        handle.wait().unwrap();
    }
}
