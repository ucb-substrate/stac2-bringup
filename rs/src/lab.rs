use std::{
    ffi::CString,
    io::{BufRead, BufReader, Write},
};

use visa_rs::{AsResourceManager, DefaultRM, TIMEOUT_IMMEDIATE, flags::AccessMode};

pub const PSU_VISA_ADDR: &str = "USB0::0x2A8D::0x8F01::CN63270183::INSTR";
pub const PSU_CHANNEL: u32 = 1;
pub const CLKGEN_VISA_ADDR: &str = "USB0::0x0957::0x4008::MY428EX302::INSTR";

pub struct Lab {
    rm: DefaultRM,
    psu: Option<Instrument>,
    clkgen: Option<Instrument>,
}

pub struct Instrument {
    inner: visa_rs::Instrument,
}

impl Instrument {
    fn write(&mut self, buf: &[u8]) {
        self.inner.write_all(buf).unwrap();
    }

    fn read(&mut self) -> String {
        let mut reader = BufReader::new(&self.inner);
        let mut buf = String::new();
        reader.read_line(&mut buf).unwrap();
        buf.trim_end().to_string()
    }

    pub fn cmd(&mut self, cmd: &str) {
        let msg = format!("{cmd}\n");
        self.write(msg.as_bytes());
    }

    pub fn query(&mut self, cmd: &str) -> String {
        self.write(cmd.as_bytes());
        self.read()
    }
}

impl Default for Lab {
    fn default() -> Self {
        Self {
            rm: DefaultRM::new().expect("failed to create VISA resource manager"),
            psu: None,
            clkgen: None,
        }
    }
}

impl Lab {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open_instr(&self, addr: &str) -> Instrument {
        let expr: visa_rs::ResID = CString::new(addr).unwrap().into();
        let rsc = self
            .rm
            .find_res(&expr)
            .unwrap_or_else(|e| panic!("instrument not found at {addr}: {e}"));
        let instr = self
            .rm
            .open(&rsc, AccessMode::NO_LOCK, TIMEOUT_IMMEDIATE)
            .unwrap_or_else(|e| panic!("failed to open {addr}: {e}"));
        Instrument { inner: instr }
    }

    pub fn psu(&mut self) -> &mut Instrument {
        if self.psu.is_none() {
            self.psu = Some(self.open_instr(PSU_VISA_ADDR));
        }
        self.psu.as_mut().unwrap()
    }

    pub fn clkgen(&mut self) -> &mut Instrument {
        if self.clkgen.is_none() {
            self.clkgen = Some(self.open_instr(CLKGEN_VISA_ADDR));
        }
        self.clkgen.as_mut().unwrap()
    }

    pub fn status(&mut self) {
        println!("===============");
        println!("PSU (ch{PSU_CHANNEL})\n");
        let state = self.psu().query(&format!("OUTP? (@{PSU_CHANNEL})"));
        let volt = self.psu_vdd_meas();
        let curr = self.psu_idd_meas();
        println!("  State:   {state}");
        println!("  Voltage: {volt:.4} V");
        println!("  Current: {curr:.4} A");
        println!("===============");

        println!("===============");
        println!("Clock Generator\n");
        let freq = self.clkgen_freq_meas();
        let div = self.clkgen().query(":OUTP1:DIV?");
        let pos = self.clkgen().query(":OUTP1:POS?");
        println!("  Freq:    {:.6E} Hz", freq);
        println!("  Divider: {div}");
        println!("  POS:     {pos}");
        println!("===============");
    }

    pub fn psu_idn(&mut self) -> String {
        self.psu().query("*IDN?")
    }

    pub fn psu_on(&mut self) {
        self.psu().cmd(&format!("OUTP ON,(@{PSU_CHANNEL})"));
    }

    pub fn psu_off(&mut self) {
        self.psu().cmd(&format!("OUTP OFF,(@{PSU_CHANNEL})"));
    }

    pub fn psu_vdd(&mut self, vdd: f64) {
        self.psu().cmd(&format!("VOLT {vdd:.4},(@{PSU_CHANNEL})"));
    }

    pub fn psu_vdd_meas(&mut self) -> f64 {
        self.psu()
            .query(&format!("MEAS:VOLT? (@{PSU_CHANNEL})"))
            .parse()
            .expect("unexpected PSU voltage response")
    }

    pub fn psu_idd_meas(&mut self) -> f64 {
        self.psu()
            .query(&format!("MEAS:CURR? (@{PSU_CHANNEL})"))
            .parse()
            .expect("unexpected PSU voltage response")
    }

    pub fn clkgen_idn(&mut self) -> String {
        self.clkgen().query("*IDN?")
    }

    pub fn clkgen_on(&mut self) {
        self.clkgen().cmd(":OUTP1:POS ON");
    }

    pub fn clkgen_off(&mut self) {
        self.clkgen().cmd(":OUTP1:POS OFF");
    }

    pub fn clkgen_freq(&mut self, freq: f64) {
        self.clkgen().cmd(&format!(":FREQ {freq:.6E}"));
    }

    pub fn clkgen_freq_meas(&mut self) -> f64 {
        self.clkgen()
            .query(":FREQ?")
            .parse()
            .expect("unexpected clkgen freq")
    }

    pub fn clkgen_vdd(&mut self, vdd: f64) {
        self.clkgen().cmd(&format!(":VOLT1:HIGH {vdd:.4}"));
        self.clkgen().cmd(":VOLT1:LOW 0.0");
    }
}
