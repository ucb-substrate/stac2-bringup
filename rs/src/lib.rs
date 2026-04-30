use const_format::concatcp;

pub mod bebe;
pub mod bist;
pub mod config;
pub mod executor;
pub mod memory;
pub mod pattern;
pub mod state;
pub mod tsi;

#[cfg(test)]
mod tests;

pub use bebe::*;
pub use executor::*;
pub use memory::*;
pub use tsi::*;

const PY_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../py");
const BEBE_HOST: &str = concatcp!(PY_DIR, "/bebe_host.py");
const DEFAULT_HALF_CLK_DIV_RATIO: u32 = 1;

pub struct BringupState {
    half_clk_div_ratio: u32,
}

impl Default for BringupState {
    fn default() -> Self {
        tsi_write(HALF_CLK_DIV_RATIO, DEFAULT_HALF_CLK_DIV_RATIO);
        BringupState {
            half_clk_div_ratio: DEFAULT_HALF_CLK_DIV_RATIO,
        }
    }
}

impl BringupState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_div_ratio(&mut self, half_clk_div_ratio: u32) {
        tsi_write(HALF_CLK_DIV_RATIO, half_clk_div_ratio);
        self.half_clk_div_ratio = half_clk_div_ratio;
    }

    pub fn enable_clk(&self) {
        tsi_write(CLK_EN, 1);
    }

    pub fn disable_clk(&self) {
        tsi_write(CLK_EN, 0);
    }

    pub fn reset_chip(&self) {
        tsi_write(RESET_REG, 1);
        tsi_write(RESET_REG, 0);
    }

    pub fn init_chip(&self) {
        let mut handle = self.bebe_wait().unwrap();
        self.enable_clk();
        self.reset_chip();
        handle.wait().unwrap();
    }
}
