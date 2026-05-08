//! Utilities for accessing chip memory.

pub const SCRATCHPAD_BASE: u64 = 0x8000000;
pub const BASE: u64 = 0x1000;
#[allow(clippy::identity_op)]
pub const ADDR: u64 = 0x0 + BASE;
pub const DIN: u64 = 0x8 + BASE;
pub const MASK: u64 = 0x18 + BASE;
pub const WE: u64 = 0x28 + BASE;
pub const SRAM_ID: u64 = 0x30 + BASE;
pub const SRAM_SEL: u64 = 0x38 + BASE;
pub const DOUT: u64 = 0x40 + BASE;
pub const DONE: u64 = 0x50 + BASE;
pub const BIST_RAND_SEED: u64 = 0x58 + BASE;
pub const BIST_SIG_SEED: u64 = 0x80 + BASE;
pub const BIST_MAX_ROW_ADDR: u64 = 0x90 + BASE;
pub const BIST_MAX_COL_ADDR: u64 = 0x98 + BASE;
pub const BIST_INNER_DIM: u64 = 0xA0 + BASE;
pub const BIST_ELEMENT_SEQUENCE: u64 = 0xA8 + BASE;
pub const BIST_PATTERN_TABLE: u64 = 0x128 + BASE;
pub const BIST_MAX_ELEMENT_IDX: u64 = 0x1A8 + BASE;
pub const BIST_CYCLE_LIMIT: u64 = 0x1B0 + BASE;
pub const BIST_STOP_ON_FAILURE: u64 = 0x1B8 + BASE;
pub const BIST_FAIL: u64 = 0x1C0 + BASE;
pub const BIST_FAIL_CYCLE: u64 = 0x1C8 + BASE;
pub const BIST_EXPECTED: u64 = 0x1D0 + BASE;
pub const BIST_RECEIVED: u64 = 0x1E0 + BASE;
pub const BIST_SIGNATURE: u64 = 0x1F0 + BASE;
pub const EX: u64 = 0x200 + BASE;

pub trait MemoryIntf {
    fn read(&mut self, addr: u64) -> anyhow::Result<u64>;
    fn write(&mut self, addr: u64, data: u64) -> anyhow::Result<()>;
    fn read128(&mut self, addr: u64) -> anyhow::Result<u128> {
        let r0 = self.read(addr)?;
        let r1 = self.read(addr + 8)?;
        Ok(((r1 as u128) << 64) | r0 as u128)
    }
    fn write128(&mut self, addr: u64, data: u128) -> anyhow::Result<()> {
        self.write(addr, data as u64)?;
        self.write(addr + 8, (data >> 64) as u64)?;
        Ok(())
    }
}
