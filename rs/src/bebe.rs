use std::io::{self};
use std::thread::sleep;
use std::time::{Duration, Instant};

use serialport::SerialPort;

use crate::config::ClkSel;
use crate::executor::{ScratchpadExecutor, TestSramExecutor};
use crate::{BringupState, FPGA_FREQ_MHZ, HALF_CLK_DIV_RATIO, MemoryIntf};

pub const CHIP_INTENDED_BAUDRATE: u64 = 115200;
pub const CHIP_INTENDED_FREQ_MHZ: u64 = 100;

const NOCK_MAGIC: &[u8] = b"GOBEARS!";
const CMD_READV: u8 = b'R';
const CMD_WRITEV: u8 = b'W';
const CMD_JUMP: u8 = b'J';
const CMD_ACK: u8 = b'Y';

fn read_exact(port: &mut dyn serialport::SerialPort, buf: &mut [u8]) -> io::Result<()> {
    let mut offset = 0;
    while offset < buf.len() {
        match port.read(&mut buf[offset..]) {
            Ok(0) => return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "port closed")),
            Ok(n) => offset += n,
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn read_byte(port: &mut dyn serialport::SerialPort) -> io::Result<u8> {
    let mut b = [0u8; 1];
    read_exact(port, &mut b)?;
    Ok(b[0])
}

fn read_some<'a>(
    port: &mut dyn serialport::SerialPort,
    buf: &'a mut [u8; 256],
) -> io::Result<&'a [u8]> {
    match port.read(buf) {
        Ok(0) => Err(io::Error::new(io::ErrorKind::UnexpectedEof, "port closed")),
        Ok(n) => Ok(&buf[..n]),
        Err(e) => Err(e),
    }
}

pub fn hexdump(data: &[u8]) -> String {
    const WIDTH: usize = 16;
    let mut out = String::new();
    for chunk in data.chunks(WIDTH) {
        for b in chunk {
            out.push_str(&format!("{b:02X} "));
        }
        for _ in 0..(WIDTH - chunk.len()) {
            out.push_str("   ");
        }
        out.push_str("  ");
        for &b in chunk {
            out.push(if (32..127).contains(&b) {
                b as char
            } else {
                '.'
            });
        }
        out.push('\n');
    }
    out
}

pub struct BebeHost {
    port: Box<dyn SerialPort>,
    pub verbose: bool,
}

impl BebeHost {
    pub fn new(port: Box<dyn SerialPort>, verbose: bool) -> Self {
        Self { port, verbose }
    }

    fn send(&mut self, data: &[u8]) -> io::Result<()> {
        if self.verbose {
            print!("{}", hexdump(data));
        }
        self.port.write_all(data)?;
        self.port.flush()
    }

    fn log(&self, msg: &str) {
        if self.verbose {
            println!("{msg}");
        }
    }

    pub fn wait(&mut self, timeout: Option<Duration>) -> io::Result<()> {
        self.log("[bebe host] Waiting for DUT...");
        let deadline = timeout.map(|d| Instant::now() + d);
        let mut buf = [0u8; 256];
        'wait: loop {
            if deadline.is_some_and(|d| Instant::now() >= d) {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "timed out waiting for DUT",
                ));
            }
            for &b in read_some(self.port.as_mut(), &mut buf)? {
                if b == b'A' {
                    break 'wait;
                }
            }
        }
        self.log("[bebe host] DUT found!");
        Ok(())
    }

    pub fn nock(&mut self, timeout: Option<Duration>) -> io::Result<()> {
        self.log("[bebe host] Trying to nock...");
        self.send(NOCK_MAGIC)?;
        let deadline = timeout.map(|d| Instant::now() + d);
        let mut buf = [0u8; 256];
        'nock: loop {
            if deadline.is_some_and(|d| Instant::now() >= d) {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "timed out during nock",
                ));
            }
            for &b in read_some(self.port.as_mut(), &mut buf)? {
                if b == b'A' {
                    continue;
                } else if b == CMD_ACK {
                    break 'nock;
                } else {
                    // return Err(io::Error::new(
                    //     io::ErrorKind::InvalidData,
                    //     format!("unexpected response during nock: {b}"),
                    // ));
                }
            }
        }
        self.log("[bebe host] Connected to DUT!");
        Ok(())
    }

    pub fn write(&mut self, addr: u64, data: &[u8]) -> io::Result<()> {
        for chunk in data.chunks(0xfffff) {
            let block_len = chunk.len();
            let mut msg = vec![CMD_WRITEV];
            msg.extend_from_slice(&(block_len as u32).to_be_bytes());
            msg.extend_from_slice(&addr.to_be_bytes());
            msg.extend_from_slice(chunk);
            self.log(&format!("[bebe host] write {addr:#x}, len {block_len}..."));
            self.send(&msg)?;
            let ack = read_byte(self.port.as_mut())?;
            if ack != CMD_ACK {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("expected ack, got {ack}"),
                ));
            }
        }
        self.log("[bebe host] OK");
        Ok(())
    }

    pub fn write_int(&mut self, addr: u64, data: u64, len: usize) -> io::Result<()> {
        if len > 8 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "len > 8, use write() with a byte slice",
            ));
        }
        let be = data.to_be_bytes();
        self.write(addr, &be[8 - len..])
    }

    pub fn read(&mut self, addr: u64, len: usize) -> io::Result<Vec<u8>> {
        let mut msg = vec![CMD_READV];
        msg.extend_from_slice(&(len as u32).to_be_bytes());
        msg.extend_from_slice(&addr.to_be_bytes());
        self.log(&format!("[bebe host] read {addr:#x}, len {len}..."));
        self.send(&msg)?;
        let mut data = vec![0u8; len];
        read_exact(self.port.as_mut(), &mut data)?;
        Ok(data)
    }

    pub fn read_int(&mut self, addr: u64, len: usize) -> io::Result<u64> {
        let data = self.read(addr, len)?;
        let mut val: u64 = 0;
        for (i, &b) in data.iter().enumerate() {
            val |= (b as u64) << (8 * i);
        }
        Ok(val)
    }

    pub fn jump(&mut self, addr: u64) -> io::Result<()> {
        let mut msg = vec![CMD_JUMP];
        msg.extend_from_slice(&addr.to_be_bytes());
        self.log(&format!("[bebe host] Jump {addr:#x}..."));
        self.send(&msg)?;
        let ack = read_byte(self.port.as_mut())?;
        if ack != CMD_ACK {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("expected ack, got {ack}"),
            ));
        }
        self.log("[bebe host] OK");
        Ok(())
    }
}

impl BringupState {
    fn bebe(&mut self) -> &mut BebeHost {
        if self.bebe.is_none() {
            let baudrate = self.bebe_baudrate() as u32;
            self.bebe = Some(BebeHost::new(
                serialport::new(&self.config.stac_com_port, baudrate)
                    .timeout(self.config.timeout)
                    .open()
                    .expect("failed to open TTY"),
                false,
            ));
            sleep(Duration::from_millis(500));
        }
        self.bebe.as_mut().unwrap()
    }

    pub fn refresh_bebe(&mut self) {
        self.bebe = None;
        self.bebe();
    }

    pub fn bebe_intf(&mut self) -> BebeIntf<'_> {
        BebeIntf::new(self)
    }

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
        (CHIP_INTENDED_BAUDRATE as f64 / CHIP_INTENDED_FREQ_MHZ as f64 * freq_mhz) as u64
    }

    pub fn bebe_init(&mut self) -> io::Result<()> {
        self.refresh_bebe();
        std::thread::sleep(Duration::from_millis(500));
        self.reset_chip();
        std::thread::sleep(Duration::from_millis(500));
        let timeout = self.config.timeout;
        let res = self.bebe().nock(Some(timeout));
        if res.is_err() {
            self.bebe = None;
        }
        res
    }

    pub fn bebe_write(&mut self, addr: u64, data: u64, len: u64) -> anyhow::Result<()> {
        let timeout = self.config.timeout;
        self.bebe().nock(Some(timeout))?;
        let res = self.bebe().write_int(addr, data, len as usize);
        if res.is_err() {
            self.bebe = None;
        }
        Ok(res?)
    }

    pub fn bebe_read(&mut self, addr: u64, len: u64) -> anyhow::Result<u64> {
        let timeout = self.config.timeout;
        self.bebe().nock(Some(timeout))?;
        let res = self.bebe.as_mut().unwrap().read_int(addr, len as usize);
        if res.is_err() {
            self.bebe = None;
        }
        Ok(res?)
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
