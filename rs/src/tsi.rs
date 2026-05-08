use std::time::Duration;

use tsi::Tsi;

use crate::{
    BringupState, MemoryIntf,
    executor::{ScratchpadExecutor, TestSramExecutor},
};

pub const FPGA_BAUD_RATE: u32 = 115200;
pub const FPGA_FREQ_MHZ: u64 = 50;

pub const CONTROLLER_BASE: u64 = 0x90000000;
#[allow(clippy::identity_op)]
pub const SRAM_EXT_EN: u64 = 0x0 + CONTROLLER_BASE;
pub const SRAM_SCAN_MODE: u64 = 0x8 + CONTROLLER_BASE;
pub const SRAM_EN: u64 = 0x10 + CONTROLLER_BASE;
pub const SRAM_BIST_EN: u64 = 0x18 + CONTROLLER_BASE;
pub const SRAM_BIST_START: u64 = 0x20 + CONTROLLER_BASE;
pub const PLL_SEL: u64 = 0x28 + CONTROLLER_BASE;
pub const PLL_SCAN_RSTN: u64 = 0x30 + CONTROLLER_BASE;
pub const PLL_ARSTB: u64 = 0x38 + CONTROLLER_BASE;
pub const HALF_CLK_DIV_RATIO: u64 = 0x40 + CONTROLLER_BASE;
pub const CLK_EN: u64 = 0x48 + CONTROLLER_BASE;
pub const RESET_REG: u64 = 0x50 + CONTROLLER_BASE;
pub const SRAM_BIST_DONE: u64 = 0x58 + CONTROLLER_BASE;

impl BringupState {
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

    pub fn tsi_intf(&mut self) -> TsiIntf<'_> {
        TsiIntf::new(self)
    }
}

pub struct TsiIntf<'a>(&'a mut BringupState);

impl<'a> TsiIntf<'a> {
    pub fn new(state: &'a mut BringupState) -> Self {
        Self(state)
    }

    pub fn scratchpad_executor(self) -> ScratchpadExecutor<Self> {
        ScratchpadExecutor::new(self)
    }

    pub fn test_sram_executor(self, sram_id: u64) -> TestSramExecutor<Self> {
        TestSramExecutor::new(self, sram_id)
    }
}

impl<'a> MemoryIntf for TsiIntf<'a> {
    fn read(&mut self, addr: u64) -> anyhow::Result<u64> {
        let res = self.0.tsi().read_word(addr).map_err(anyhow::Error::from);
        if res.is_err() {
            self.0.tsi = None;
        }
        res
    }

    fn write(&mut self, addr: u64, data: u64) -> anyhow::Result<()> {
        let res = self
            .0
            .tsi()
            .write_word(addr, data)
            .map_err(anyhow::Error::from);
        if res.is_err() {
            self.0.tsi = None;
        }
        res
    }
}
