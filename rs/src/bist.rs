use crate::memory::*;
use crate::MemoryIntf;

const ELEMENT_TABLE_LENGTH: usize = 8;
const OPERATIONS_PER_ELEMENT: usize = 8;
const PATTERN_TABLE_LENGTH: usize = 8;
const MAX_ROW_ADDR_WIDTH: usize = 11;
const MAX_COL_ADDR_WIDTH: usize = 3;
const DATA_WIDTH: usize = 128;
const RAND_ADDR_WIDTH: usize = 14;

enum OperationType {
    Read,
    Write,
    Rand,
}

enum OpElementSeq {
    Up,
    Down,
    Rand(u64),
}

enum InnerDim {
    Row,
    Col,
}

struct Op {
    typ: OperationType,
    rand_data: bool,
    rand_mask: bool,
    data_pattern_idx: usize,
    mask_pattern_idx: usize,
    flip_data: bool,
}

struct OpElement {
    ops: Vec<Op>,
    seq: OpElementSeq,
}

struct WaitElement {
    cycles: u64,
}

enum Element {
    Op(OpElement),
    Wait(WaitElement),
}

pub struct BistExecutor<I> {
    intf: I,
    rows: u64,
    cols: u64,
    inner_dim: InnerDim,
    // TODO: seed
    patterns: Vec<u128>,
    elts: Vec<Element>,
    cycle_limit: u64,
}

impl<I> BistExecutor<I> {
    pub fn validate(&self) {
        assert!(self.rows > 0);
        assert!(self.cols > 0);
        assert!(self.rows <= 2u64.strict_pow(MAX_ROW_ADDR_WIDTH as u32));
        assert!(self.cols <= 2u64.strict_pow(MAX_COL_ADDR_WIDTH as u32));
        assert!(self.patterns.len() <= PATTERN_TABLE_LENGTH);
        assert!(self.elts.len() <= ELEMENT_TABLE_LENGTH);
        for elt in self.elts.iter() {
            match elt {
                Element::Op(e) => {
                    assert!(e.ops.len() <= OPERATIONS_PER_ELEMENT);
                    for op in e.ops.iter() {
                        assert!(op.data_pattern_idx < self.patterns.len());
                        assert!(op.mask_pattern_idx < self.patterns.len());
                    }
                    if let OpElementSeq::Rand(n) = e.seq {
                        assert!(n < 2u64.strict_pow(RAND_ADDR_WIDTH as u32));
                    }
                }
                Element::Wait(e) => {
                    assert!(e.cycles < 2u64.strict_pow(RAND_ADDR_WIDTH as u32));
                }
            }
        }
    }
}

impl<I: MemoryIntf> BistExecutor<I> {
    fn init(&self) {
        self.intf.write(BIST_MAX_ROW_ADDR, self.rows - 1);
        self.intf.write(BIST_MAX_COL_ADDR, self.cols - 1);
        self.intf.write(BIST_INNER_DIM, self.inner_dim.encode());
        todo!("Claude: continue filling this in.");
    }
}

impl InnerDim {
    pub fn encode(&self) -> u64 {
        match self {
            Self::Row => 0,
            Self::Col => 1,
        }
    }
}

impl Element {
    pub fn encode(&self) -> u128 {
        match self {
            Self::Op(e) => e.encode() << (RAND_ADDR_WIDTH + 1) | 1,
            Self::Wait(e) => (e.cycles as u128) << 1,
        }
    }
}

impl OpElementSeq {
    pub fn encode(&self) -> u128 {
        match self {
            Self::Up => 0,
            Self::Down => 1,
            Self::Rand(_) => 2,
        }
    }
    pub fn encode_full(&self) -> u128 {
        match self {
            Self::Up => 0,
            Self::Down => 1 << RAND_ADDR_WIDTH,
            Self::Rand(n) => 2 << RAND_ADDR_WIDTH | *n as u128,
        }
    }
}
