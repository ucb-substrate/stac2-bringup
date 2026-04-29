use crate::config::CONFIG;
use crate::executor::Executor;
use crate::pattern::{SramAddr, SramWord};
use crate::{BringupState, BEBE_HOST, FPGA_FREQ_MHZ, PY_DIR};
use std::process::{Child, Command};

pub const CHIP_INTENDED_BAUDRATE: u64 = 115200;
pub const CHIP_INTENDED_FREQ_MHZ: u64 = 100;

pub const SCRATCHPAD_BASE_ADDR: u64 = 0x8000000;
pub const BASE: u64 = 0x1000;
#[allow(clippy::identity_op)]
pub const ADDR: u64 = 0x0 + BASE;
pub const DIN: u64 = 0x8 + BASE;
pub const MASK: u64 = 0x10 + BASE;
pub const WE: u64 = 0x18 + BASE;
pub const SRAM_ID: u64 = 0x20 + BASE;
pub const SRAM_SEL: u64 = 0x28 + BASE;
pub const SAE_CTL: u64 = 0x30 + BASE;
pub const SAE_SEL: u64 = 0x38 + BASE;
pub const DOUT: u64 = 0x40 + BASE;
pub const TDC: u64 = 0x48 + BASE;
pub const DONE: u64 = 0x68 + BASE;
pub const BIST_RAND_SEED: u64 = 0x70 + BASE;
pub const BIST_SIG_SEED: u64 = 0x80 + BASE;
pub const BIST_MAX_ROW_ADDR: u64 = 0x88 + BASE;
pub const BIST_MAX_COL_ADDR: u64 = 0x90 + BASE;
pub const BIST_INNER_DIM: u64 = 0x98 + BASE;
pub const BIST_ELEMENT_SEQUENCE: u64 = 0xa0 + BASE;
pub const BIST_PATTERN_TABLE: u64 = 0x120 + BASE;
pub const BIST_MAX_ELEMENT_IDX: u64 = 0x140 + BASE;
pub const BIST_CYCLE_LIMIT: u64 = 0x148 + BASE;
pub const BIST_STOP_ON_FAILURE: u64 = 0x150 + BASE;
pub const BIST_FAIL: u64 = 0x158 + BASE;
pub const BIST_FAIL_CYCLE: u64 = 0x160 + BASE;
pub const BIST_EXPECTED: u64 = 0x168 + BASE;
pub const BIST_RECEIVED: u64 = 0x170 + BASE;
pub const BIST_SIGNATURE: u64 = 0x178 + BASE;
pub const EX: u64 = 0x180 + BASE;

impl BringupState {
    pub fn bebe_baudrate(&self) -> u64 {
        CHIP_INTENDED_BAUDRATE * FPGA_FREQ_MHZ
            / CHIP_INTENDED_FREQ_MHZ
            / self.half_clk_div_ratio as u64
            / 2
    }

    pub fn bebe_wait(&self) -> std::io::Result<Child> {
        Command::new("uv")
            .args([
                "run",
                BEBE_HOST,
                "--port",
                &CONFIG.stac_com_port,
                "--baudrate",
                &self.bebe_baudrate().to_string(),
                "--wait",
            ])
            .current_dir(PY_DIR)
            .spawn()
    }

    pub fn bebe_write(&self, addr: u64, data: u64, len: u64) {
        let addr = format!("{addr:X}");
        let data = format!("{data:X}");
        let len = format!("{len}");
        let status = Command::new("uv")
            .args([
                "run",
                BEBE_HOST,
                "--port",
                &CONFIG.stac_com_port,
                "--baudrate",
                &self.bebe_baudrate().to_string(),
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

    pub fn bebe_read(&self, addr: u64, len: u64) -> u64 {
        let addr = format!("{addr:X}");
        let len = format!("{len}");
        let output = Command::new("uv")
            .args([
                "run",
                BEBE_HOST,
                "--port",
                &CONFIG.stac_com_port,
                "--baudrate",
                &self.bebe_baudrate().to_string(),
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
        println!("{}", output);
        output
            .trim()
            .parse()
            .expect("failed to convert bebe output to u64")
    }
}

pub struct BebeExecutor<'a> {
    state: &'a BringupState,
    sram_id: u64,
}

pub struct BebeScratchpadExecutor<'a>(&'a BringupState);

impl<'a> BebeExecutor<'a> {
    pub fn new(state: &'a BringupState, sram_id: u64) -> Self {
        Self { state, sram_id }
    }
}

impl<'a> Executor for BebeExecutor<'a> {
    fn init(&mut self) {}
    fn read(&mut self, addr: SramAddr) -> SramWord {
        self.state.bebe_write(0x1000, addr as u64, 8);
        // no need to set the mask
        // bebe_write(0x1010, u64::MAX, 8);
        self.state.bebe_write(0x1018, 0, 8);
        self.state.bebe_write(0x1020, self.sram_id, 8);
        self.state.bebe_write(0x1028, 0, 8);
        self.state.bebe_write(0x1038, 0, 8);
        self.state.bebe_write(0x1180, u64::MAX, 8);
        self.state.bebe_read(0x1040, 8)
    }

    fn write(&mut self, addr: SramAddr, data: SramWord, mask: SramWord) {
        self.state.bebe_write(0x1000, addr as u64, 8);
        self.state.bebe_write(0x1008, data, 8);
        self.state.bebe_write(0x1010, mask, 8);
        self.state.bebe_write(0x1018, u64::MAX, 8);
        self.state.bebe_write(0x1020, self.sram_id, 8);
        self.state.bebe_write(0x1028, 0, 8);
        self.state.bebe_write(0x1038, 0, 8);
        self.state.bebe_write(0x1180, u64::MAX, 8);
    }

    fn finish(&mut self) {}
}

impl<'a> BebeScratchpadExecutor<'a> {
    pub fn new(state: &'a BringupState) -> Self {
        Self(state)
    }
}

impl<'a> Executor for BebeScratchpadExecutor<'a> {
    fn init(&mut self) {}
    fn read(&mut self, addr: SramAddr) -> SramWord {
        self.0.bebe_read(SCRATCHPAD_BASE_ADDR + addr as u64 * 8, 8)
    }

    fn write(&mut self, addr: SramAddr, data: SramWord, mask: SramWord) {
        assert_eq!(mask, 0xFF, "scratchpad only supports mask of all 1s");
        self.0
            .bebe_write(SCRATCHPAD_BASE_ADDR + addr as u64 * 8, data, 8);
    }

    fn finish(&mut self) {}
}
