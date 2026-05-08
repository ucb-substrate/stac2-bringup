use std::path::PathBuf;

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

use crate::config::{CONFIG_PATH, ClkSel, Config, load_config};

pub struct BringupState {
    config_path: Option<PathBuf>,
    config: Config,
    pub(crate) tsi: Option<Tsi>,
    pub(crate) lab: Option<Lab>,
    pub(crate) bebe: Option<bebe::BebeHost>,
}

impl Default for BringupState {
    fn default() -> Self {
        BringupState {
            config_path: Some(PathBuf::from(CONFIG_PATH)),
            config: load_config(CONFIG_PATH),
            tsi: None,
            lab: None,
            bebe: None,
        }
    }
}

impl BringupState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_config(config: Config) -> Self {
        BringupState {
            config_path: None,
            config,
            tsi: None,
            lab: None,
            bebe: None,
        }
    }

    pub fn from_config_path(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let config = load_config(&path);
        BringupState {
            config_path: Some(path),
            config,
            tsi: None,
            lab: None,
            bebe: None,
        }
    }

    pub fn set_config_path(&mut self, path: impl Into<PathBuf>) {
        self.config_path = Some(path.into());
        self.config = load_config(self.config_path.as_ref().unwrap());
        self.reset_state();
    }

    pub fn reload_config(&mut self) {
        if let Some(path) = &self.config_path {
            self.config = load_config(path);
            self.reset_state();
        }
    }

    pub fn set_config(&mut self, config: Config) {
        self.config_path = None;
        self.config = config;
        self.reset_state();
    }

    pub fn reset_state(&mut self) {
        self.bebe = None;
        self.tsi = None;
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

    pub fn reset_fpga(&mut self) {
        self.tsi = None;
        self.tsi();
    }

    pub fn init_chip(&mut self) -> anyhow::Result<()> {
        if let ClkSel::Fpga = self.config.clk_sel {
            self.enable_clk();
        }
        self.reset_chip();
        match self.tsi_intf().read(SCRATCHPAD_BASE) {
            Ok(_) => {
                println!("Chip initialized!");
                Ok(())
            }
            Err(e) => {
                eprintln!("{e}");
                Err(e)
            }
        }
    }
}
