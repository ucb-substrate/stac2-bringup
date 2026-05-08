use const_format::concatcp;

pub mod bebe;
pub mod bist;
pub mod config;
pub mod executor;
pub mod lab;
pub mod memory;
pub mod pattern;
pub mod shmootest;
pub mod state;
pub(crate) mod tests;
pub mod tsi;

use ::tsi::Tsi;
pub use bebe::*;
pub use bist::*;
pub use executor::*;
pub use memory::*;
pub use shmootest::*;
pub use tsi::*;

use crate::{
    config::{Config, load_config},
    lab::Lab,
};

const PY_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../py");
const BEBE_HOST: &str = concatcp!(PY_DIR, "/bebe_host.py");

pub struct BringupState {
    config: Config,
    pub(crate) tsi: Option<Tsi>,
    pub(crate) lab: Option<Lab>,
}

impl Default for BringupState {
    fn default() -> Self {
        BringupState {
            config: load_config(),
            tsi: None,
            lab: None,
        }
    }
}

impl BringupState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reload_config(&mut self) {
        self.config = load_config();
        self.tsi = None;
    }

    pub fn tsi(&mut self) -> &mut Tsi {
        self.tsi
            .get_or_insert_with(|| Tsi::new(&self.config.fpga_com_port, FPGA_BAUD_RATE))
    }

    pub fn lab(&mut self) -> &mut Lab {
        self.lab.get_or_insert_with(|| Lab::new())
    }

    pub fn set_div_ratio(&mut self, half_clk_div_ratio: u32) {
        self.tsi()
            .write_word(HALF_CLK_DIV_RATIO, half_clk_div_ratio as u64)
            .expect("failed to write");
    }

    pub fn enable_clk(&mut self) {
        self.tsi().write_word(CLK_EN, 1).expect("failed to write");
    }

    pub fn disable_clk(&mut self) {
        self.tsi().write_word(CLK_EN, 0).expect("failed to write");
    }

    pub fn reset_chip(&mut self) {
        self.tsi()
            .write_word(RESET_REG, 1)
            .expect("failed to write");
        self.tsi()
            .write_word(RESET_REG, 0)
            .expect("failed to write");
    }

    /// Re-opens the FPGA COM port on failure.
    pub fn read_word(&mut self, addr: u64) -> std::io::Result<u64> {
        match self.tsi().read_word(addr) {
            Ok(data) => Ok(data),
            Err(e) => {
                self.tsi = None;
                Err(e)
            }
        }
    }
    pub fn init_chip(&mut self) -> std::io::Result<()> {
        self.enable_clk();
        self.reset_chip();
        match self.read_word(SCRATCHPAD_BASE) {
            Ok(_) => {
                println!("Chip initialized!");
                Ok(())
            }
            Err(e) => {
                println!("{e}");
                Err(e)
            }
        }
    }
}
