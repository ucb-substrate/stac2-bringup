use std::{
    ffi::CString,
    io::{Read, Write},
};

use visa_rs::{
    AsResourceManager, DefaultRM, Instrument, TIMEOUT_IMMEDIATE,
    enums::attribute::{AttrTmoValue, HasAttribute},
    flags::AccessMode,
};

use crate::{CLKGEN_VISA_ADDR, PSU_VISA_ADDR};

pub struct Lab {
    rm: DefaultRM,
    psu: Option<Instrument>,
    clkgen: Option<Instrument>,
}

pub fn scpi(instr: &mut Instrument, cmd: &str) {
    let msg = format!("{cmd}\n");
    instr.write_all(msg.as_bytes()).expect("SCPI write failed");
}

pub fn query(instr: &mut Instrument, cmd: &str) -> String {
    scpi(instr, cmd);
    let mut buf = vec![0u8; 4096];
    let n = instr.read(&mut buf).expect("SCPI read failed");
    String::from_utf8_lossy(&buf[..n])
        .trim_end_matches(['\r', '\n'])
        .to_string()
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
        // 10 second I/O timeout (TIMEOUT_IMMEDIATE = 0 causes reads to time out instantly)
        instr
            .set_attr(unsafe { AttrTmoValue::new_unchecked(10_000) })
            .expect("failed to set I/O timeout");
        instr
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
