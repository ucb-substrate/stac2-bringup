use crate::executor::{ScratchpadExecutor, TestSramExecutor};
use crate::{BEBE_HOST, BringupState, FPGA_FREQ_MHZ, HALF_CLK_DIV_RATIO, MemoryIntf, PY_DIR};
use std::process::{Child, Command};

pub const CHIP_INTENDED_BAUDRATE: u64 = 115200;
pub const CHIP_INTENDED_FREQ_MHZ: u64 = 100;

impl BringupState {
    pub fn bebe_baudrate(&mut self) -> u64 {
        let half_clk_div_ratio = self.tsi_intf().read(HALF_CLK_DIV_RATIO);
        CHIP_INTENDED_BAUDRATE * FPGA_FREQ_MHZ / CHIP_INTENDED_FREQ_MHZ / half_clk_div_ratio / 2
    }

    pub fn bebe_intf(&mut self) -> BebeIntf<'_> {
        BebeIntf::new(self)
    }

    pub fn bebe_wait(&mut self) -> std::io::Result<Child> {
        let baudrate = self.bebe_baudrate();
        Command::new("uv")
            .args([
                "run",
                BEBE_HOST,
                "--port",
                &self.config.stac_com_port,
                "--baudrate",
                &baudrate.to_string(),
                "--wait",
            ])
            .current_dir(PY_DIR)
            .spawn()
    }

    pub fn bebe_write(&mut self, addr: u64, data: u64, len: u64) {
        let addr = format!("{addr:X}");
        let data = format!("{data:X}");
        let len = format!("{len}");
        let baudrate = self.bebe_baudrate();
        let status = Command::new("uv")
            .args([
                "run",
                BEBE_HOST,
                "--port",
                &self.config.stac_com_port,
                "--baudrate",
                &baudrate.to_string(),
                "--quiet",
                "--addr",
                &addr,
                "--wdata",
                &data,
                "--wlen",
                &len,
            ])
            .current_dir(PY_DIR)
            .status()
            .expect("failed to run bebe");
        if !status.success() {
            panic!("bebe exited with non-zero exit code")
        }
    }

    pub fn bebe_read(&mut self, addr: u64, len: u64) -> u64 {
        let addr = format!("{addr:X}");
        let len = format!("{len}");
        let baudrate = self.bebe_baudrate();
        let output = Command::new("uv")
            .args([
                "run",
                BEBE_HOST,
                "--port",
                &self.config.stac_com_port,
                "--baudrate",
                &baudrate.to_string(),
                "--quiet",
                "--addr",
                &addr,
                "--rlen",
                &len,
            ])
            .current_dir(PY_DIR)
            .output()
            .expect("failed to run bebe");
        let output = String::from_utf8(output.stdout).expect("failed to parse bebe output");
        output
            .trim()
            .parse()
            .expect("failed to convert bebe output to u64")
    }
}

pub struct BebeIntf<'a>(&'a mut BringupState);

impl<'a> BebeIntf<'a> {
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

impl<'a> MemoryIntf for BebeIntf<'a> {
    fn read(&mut self, addr: u64) -> u64 {
        self.0.bebe_read(addr, 8)
    }

    fn write(&mut self, addr: u64, data: u64) {
        self.0.bebe_write(addr, data, 8);
    }
}
