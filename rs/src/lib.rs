use std::time::Duration;

use const_format::concatcp;

pub mod bebe;
pub mod bist;
pub mod config;
pub mod executor;
pub mod lab;
pub mod memory;
pub mod pattern;
pub mod shmoo;
pub mod state;
pub mod tests;
pub mod tsi;

use ::tsi::Tsi;
pub use bebe::*;
pub use bist::*;
pub use executor::*;
pub use lab::*;
pub use memory::*;
pub use shmoo::*;
pub use tests::*;
pub use tsi::*;

use crate::config::{Config, load_config};

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

    pub(crate) fn tsi(&mut self) -> &mut Tsi {
        self.tsi.get_or_insert_with(|| {
            Tsi::new(
                serialport::new(&self.config.fpga_com_port, FPGA_BAUD_RATE)
                    .timeout(Duration::from_millis(500))
                    .open()
                    .expect("failed to open TTY"),
            )
        })
    }

    pub fn lab(&mut self) -> &mut Lab {
        self.lab.get_or_insert_with(Lab::new)
    }

    pub fn set_div_ratio(&mut self, half_clk_div_ratio: u32) {
        self.tsi_intf()
            .write(HALF_CLK_DIV_RATIO, half_clk_div_ratio as u64)
            .expect("failed to write");
    }

    pub fn enable_clk(&mut self) {
        self.tsi_intf().write(CLK_EN, 1).expect("failed to write");
    }

    pub fn disable_clk(&mut self) {
        self.tsi_intf().write(CLK_EN, 0).expect("failed to write");
    }

    pub fn reset_chip(&mut self) {
        self.tsi_intf()
            .write(RESET_REG, 1)
            .expect("failed to write");
        self.tsi_intf()
            .write(RESET_REG, 0)
            .expect("failed to write");
    }

    pub fn init_chip(&mut self) -> anyhow::Result<()> {
        self.enable_clk();
        self.reset_chip();
        match self.tsi_intf().read(SCRATCHPAD_BASE) {
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
