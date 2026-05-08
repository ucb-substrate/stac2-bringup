use serde::{Deserialize, Serialize};

use crate::pattern::{FixedPattern, FixedSramOp, SramAddr, SramSize, SramWord};
use crate::state::SramState;
use crate::{ADDR, DIN, DOUT, EX, MASK, MemoryIntf, SCRATCHPAD_BASE, SRAM_ID, SRAM_SEL, WE};

pub trait Executor {
    fn init(&mut self);
    fn read(&mut self, addr: SramAddr) -> SramWord;
    fn write(&mut self, addr: SramAddr, data: SramWord, mask: SramWord);

    fn finish(&mut self);
}

pub struct IdealExecutor {
    state: SramState,
}

impl IdealExecutor {
    pub fn new(size: SramSize) -> Self {
        let state = SramState::new(size);
        Self { state }
    }
}

impl Executor for IdealExecutor {
    fn init(&mut self) {}

    fn read(&mut self, addr: SramAddr) -> SramWord {
        self.state
            .read(addr)
            .expect("tried to read from an uninitialized address")
    }

    fn write(&mut self, addr: SramAddr, data: SramWord, mask: SramWord) {
        self.state.write(addr, data, mask);
    }

    fn finish(&mut self) {}
}

pub struct TestSramExecutor<I> {
    intf: I,
    sram_id: u64,
}

pub struct ScratchpadExecutor<I>(I);

impl<I> TestSramExecutor<I> {
    pub fn new(intf: I, sram_id: u64) -> Self {
        Self { intf, sram_id }
    }
}

impl<I: MemoryIntf> Executor for TestSramExecutor<I> {
    fn init(&mut self) {}
    fn read(&mut self, addr: SramAddr) -> SramWord {
        self.intf.write(ADDR, addr as u64).expect("TSI write failed");
        // no need to set the din/mask
        self.intf.write(WE, 0).expect("TSI write failed");
        self.intf.write(SRAM_ID, self.sram_id).expect("TSI write failed");
        self.intf.write(SRAM_SEL, 0).expect("TSI write failed");
        self.intf.write(EX, u64::MAX).expect("TSI write failed");
        self.intf.read128(DOUT).expect("TSI read failed")
    }

    fn write(&mut self, addr: SramAddr, data: SramWord, mask: SramWord) {
        self.intf.write(ADDR, addr as u64).expect("TSI write failed");
        self.intf.write128(DIN, data).expect("TSI write failed");
        self.intf.write128(MASK, mask).expect("TSI write failed");
        self.intf.write(WE, 1).expect("TSI write failed");
        self.intf.write(SRAM_ID, self.sram_id).expect("TSI write failed");
        self.intf.write(SRAM_SEL, 0).expect("TSI write failed");
        self.intf.write(EX, u64::MAX).expect("TSI write failed");
    }

    fn finish(&mut self) {}
}

impl<I> ScratchpadExecutor<I> {
    pub fn new(intf: I) -> Self {
        Self(intf)
    }
}

impl<I: MemoryIntf> Executor for ScratchpadExecutor<I> {
    fn init(&mut self) {}
    fn read(&mut self, addr: SramAddr) -> SramWord {
        self.0
            .read(SCRATCHPAD_BASE + addr as u64 * 8)
            .expect("TSI read failed") as SramWord
    }

    fn write(&mut self, addr: SramAddr, data: SramWord, mask: SramWord) {
        assert_eq!(mask, 0xFF, "scratchpad only supports mask of all 1s");
        assert_eq!(data >> 64, 0, "scratchpad only supports 64 bit writes");
        self.0
            .write(SCRATCHPAD_BASE + addr as u64 * 8, data as u64)
            .expect("TSI write failed");
    }

    fn finish(&mut self) {}
}

/// A collection of all errors produced by executing a test.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TestPatternErrors {
    pub errors: Vec<BistError>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct BistError {
    op: usize,
    expected: SramWord,
    received: SramWord,
}

/// Executes a test sequence.
pub fn execute<E: Executor>(pattern: FixedPattern, mut ex: E) -> Result<(), TestPatternErrors> {
    println!("Beginning SRAM BIST test");

    ex.init();
    let res = execute_inner(pattern, &mut ex, 0);
    ex.finish();

    res
}

/// Executes a test sequence, skipping the first `offset` operations.
pub fn execute_starting_at<E: Executor>(
    pattern: FixedPattern,
    mut ex: E,
    offset: usize,
) -> Result<(), TestPatternErrors> {
    println!("Beginning SRAM BIST test starting at operation {offset}");

    ex.init();
    let res = execute_inner(pattern, &mut ex, offset);
    ex.finish();

    res
}

fn execute_inner<E: Executor>(
    pattern: FixedPattern,
    ex: &mut E,
    ofs: usize,
) -> Result<(), TestPatternErrors> {
    let mut errors = Vec::new();
    for (i, op) in pattern.ops().enumerate().skip(ofs) {
        match op {
            FixedSramOp::Read { data, addr } => {
                print!("Reading {addr:#x}...\t");
                let dout = ex.read(addr);
                if dout == data {
                    println!("OK (received {dout:#x})");
                } else {
                    eprintln!("ERROR: got {dout:#x}, expected {data:#x}");
                    errors.push(BistError {
                        op: i,
                        expected: data,
                        received: dout,
                    });
                }
            }
            FixedSramOp::Write { data, addr, mask } => {
                print!("Writing {addr:#x} with data = {data:#x}, mask = {mask:#x}...\t");
                ex.write(addr, data, mask);
                println!("DONE");
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(TestPatternErrors { errors })
    }
}
