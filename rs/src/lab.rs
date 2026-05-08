use std::{
    ffi::CString,
    io::{BufRead, BufReader, Read, Write},
};

use visa_rs::{
    AsResourceManager, DefaultRM, TIMEOUT_IMMEDIATE,
    enums::attribute::{AttrTmoValue, HasAttribute},
    flags::AccessMode,
};

use crate::{CLKGEN_VISA_ADDR, PSU_VISA_ADDR};

pub struct Lab {
    rm: DefaultRM,
    psu: Option<Instrument>,
    clkgen: Option<Instrument>,
}

struct Instrument {
    inner: visa_rs::Instrument,
}

impl Instrument {
    pub fn write(&mut self, buf: &[u8]) {
        self.inner.write_all(buf).unwrap();
    }

    pub fn read(&mut self) -> String {
        let mut reader = BufReader::new(&self.inner);
        let mut buf = String::new();
        reader.read_line(&mut buf).unwrap();
        buf.trim_end().to_string()
    }
}

pub fn scpi(instr: &mut Instrument, cmd: &str) {
    let msg = format!("{cmd}\n");
    instr.write(msg.as_bytes());
}

pub fn query(instr: &mut Instrument, cmd: &str) -> String {
    scpi(instr, cmd);
    instr.read()
}

impl Lab {
    pub fn new() -> Self {
        Self {
            rm: DefaultRM::new().expect("failed to create VISA resource manager"),
            psu: None,
            clkgen: None,
        }
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
        if self.psu.is_none() {
            self.psu = Some(self.open_instr(CLKGEN_VISA_ADDR));
        }
        self.psu.as_mut().unwrap()
    }
}
