use const_format::concatcp;

use crate::config::ClkSel;
use crate::executor::{ScratchpadExecutor, TestSramExecutor};
use crate::{BringupState, FPGA_FREQ_MHZ, HALF_CLK_DIV_RATIO, MemoryIntf, PY_DIR, RS_DIR};
use std::process::{Child, Command};
use std::thread::sleep;
use std::time::Duration;

pub const CHIP_INTENDED_BAUDRATE: u64 = 115200;
pub const CHIP_INTENDED_FREQ_MHZ: u64 = 100;
pub const BEBE_HOST: &str = concatcp!(RS_DIR, "/target/debug/bebe_host");

impl BringupState {
    pub fn bebe_baudrate(&mut self) -> u64 {
        let freq_mhz = match self.config.clk_sel {
            ClkSel::Fpga => {
                let half_clk_div_ratio = self
                    .tsi_intf()
                    .read(HALF_CLK_DIV_RATIO)
                    .expect("failed to read half_clk_div_ratio");
                (FPGA_FREQ_MHZ / half_clk_div_ratio / 2) as f64
            }
            ClkSel::External => self.lab().clkgen_freq_meas() / 1e6,
        };
        println!("freq_mhz = {freq_mhz}");
        let baudrate =
            (CHIP_INTENDED_BAUDRATE as f64 / CHIP_INTENDED_FREQ_MHZ as f64 * freq_mhz) as u64;
        println!("baudrate = {baudrate}");
        baudrate
    }

    pub fn bebe_intf(&mut self) -> BebeIntf<'_> {
        BebeIntf::new(self)
    }

    pub fn bebe_wait(&mut self) -> std::io::Result<Child> {
        let baudrate = self.bebe_baudrate();
        Command::new(BEBE_HOST)
            .args([
                "--port",
                &self.config.stac_com_port,
                "--baudrate",
                &baudrate.to_string(),
                "--wait",
                "--wait-timeout",
                "1",
            ])
            .spawn()
    }

    pub fn bebe_init(&mut self) -> std::io::Result<()> {
        let mut handle = self.bebe_wait()?;
        sleep(Duration::from_millis(500));
        self.reset_chip();
        let status = handle.wait()?;
        if status.success() {
            Ok(())
        } else {
            Err(std::io::Error::other("bebe_host failed"))
        }
    }

    pub fn bebe_write(&mut self, addr: u64, data: u64, len: u64) -> anyhow::Result<()> {
        let addr = format!("{addr:X}");
        let data = format!("{data:X}");
        let len = format!("{len}");
        let baudrate = self.bebe_baudrate();
        let status = Command::new(BEBE_HOST)
            .args([
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
            .status()?;
        anyhow::ensure!(status.success(), "bebe_write exited with {status}");
        Ok(())
    }

    pub fn bebe_read(&mut self, addr: u64, len: u64) -> anyhow::Result<u64> {
        let addr = format!("{addr:X}");
        let len = format!("{len}");
        let baudrate = self.bebe_baudrate();
        let output = Command::new(BEBE_HOST)
            .args([
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
            .output()?;
        anyhow::ensure!(output.status.success(), "bebe_read exited with {}", output.status);
        let s = String::from_utf8(output.stdout)?;
        Ok(s.trim().parse()?)
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
    fn read(&mut self, addr: u64) -> anyhow::Result<u64> {
        self.0.bebe_read(addr, 8)
    }

    fn write(&mut self, addr: u64, data: u64) -> anyhow::Result<()> {
        self.0.bebe_write(addr, data, 8)
    }
}
