use const_format::concatcp;

pub mod bebe;
pub mod config;
pub mod executor;
pub mod pattern;
pub mod state;
pub mod testsite;
pub mod tsi;

#[cfg(test)]
mod tests;

pub use bebe::*;
pub use tsi::*;

const PY_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../py");
const BEBE_HOST: &str = concatcp!(PY_DIR, "/bebe_host.py");
const DEFAULT_HALF_CLK_DIV_RATIO: u32 = 1;

pub struct BringupState {
    half_clk_div_ratio: u32,
}

impl Default for BringupState {
    fn default() -> Self {
        ctl_write(HALF_CLK_DIV_RATIO, DEFAULT_HALF_CLK_DIV_RATIO as u64);
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
        ctl_write(HALF_CLK_DIV_RATIO, half_clk_div_ratio as u64);
        self.half_clk_div_ratio = half_clk_div_ratio;
    }

    pub fn enable_clk(&self) {
        ctl_write(CLK_EN, 1);
    }

    pub fn disable_clk(&self) {
        ctl_write(CLK_EN, 0);
    }

    pub fn reset_chip(&self) {
        ctl_write(RESET_REG, 1);
        ctl_write(RESET_REG, 0);
    }

    pub fn init_chip(&self) {
        self.enable_clk();
        self.reset_chip();
    }
}
