use std::{
    ffi::CString,
    io::{BufRead, BufReader, Write},
};

use visa_rs::{flags::AccessMode, AsResourceManager, DefaultRM, TIMEOUT_IMMEDIATE};

use crate::{CLKGEN_VISA_ADDR, PSU_VISA_ADDR};

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
}
