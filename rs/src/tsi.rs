use std::process::Command;

use crate::{
    config::CONFIG,
    executor::{ScratchpadExecutor, TestSramExecutor},
    MemoryIntf, PY_DIR,
};

pub const FPGA_BAUDRATE: u64 = 115200;
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

pub fn tsi_write(addr: u64, data: u64) {
    let status = Command::new("uv")
        .args([
            "run",
            "python3",
            "-m",
            "pyuartsi",
            "--port",
            &CONFIG.fpga_com_port,
            "--baudrate",
            &FPGA_BAUDRATE.to_string(),
            "--init_write",
            &format!("0x{addr:X}=0x{data:X}"),
        ])
        .current_dir(PY_DIR)
        .status()
        .expect("failed to run pyuartsi");
    if !status.success() {
        panic!("pyuartsi exited with non-zero exit code")
    }
}

pub fn tsi_read(addr: u64) -> u64 {
    let output = Command::new("uv")
        .args([
            "run",
            "python3",
            "-m",
            "pyuartsi",
            "--port",
            &CONFIG.fpga_com_port,
            "--baudrate",
            &FPGA_BAUDRATE.to_string(),
            "--init_read",
            &format!("0x{addr:X}"),
        ])
        .current_dir(PY_DIR)
        .output()
        .expect("failed to run pyuartsi");
    let output = String::from_utf8(output.stdout).expect("failed to parse pyuartsi output");
    println!("{}", output);
    let rhs = output.split_once("=>").unwrap().1.trim();
    let num = rhs.strip_prefix("0x").unwrap_or(rhs);
    u64::from_str_radix(num, 16).unwrap()
}

pub struct TsiIntf;

impl TsiIntf {
    pub fn scratchpad_executor() -> ScratchpadExecutor<Self> {
        ScratchpadExecutor::new(TsiIntf)
    }

    pub fn test_sram_executor(sram_id: u64) -> TestSramExecutor<Self> {
        TestSramExecutor::new(TsiIntf, sram_id)
    }
}

impl MemoryIntf for TsiIntf {
    fn read(&mut self, addr: u64) -> u64 {
        tsi_read(addr)
    }

    fn write(&mut self, addr: u64, data: u64) {
        tsi_write(addr, data);
    }
}
