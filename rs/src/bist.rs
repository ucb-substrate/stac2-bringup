use crate::MemoryIntf;
use crate::memory::*;

const ELEMENT_TABLE_LENGTH: usize = 8;
const OPERATIONS_PER_ELEMENT: usize = 8;
const PATTERN_TABLE_LENGTH: usize = 8;
const MAX_ROW_ADDR_WIDTH: usize = 11;
const MAX_COL_ADDR_WIDTH: usize = 3;
const DATA_WIDTH: usize = 128;
const RAND_ADDR_WIDTH: usize = 14;
const ELEMENT_WIDTH: usize = 122;

const SRAM_SEL_BIST: u64 = 1;

pub fn basic_bist<I>(intf: I) -> BistController<I> {
    BistController {
        intf,
        sram_id: id,
        rows: size.rows() as u64,
        cols: size.cols() as u64,
        inner_dim: InnerDim::Col,
        rand_seed: 0x22,
        sig_seed: 0x12345678,
        patterns: vec![
            0,
            0xffffffffffffffffffffffffffffffff,
            0x123456789abcdefdeadbeef123456789,
            0xfa70ec3c686eff304ab421a404f650ee,
            0xaf899304f192ffb2e75aa2036786a6e3,
            0x5472e4c65ef7294ca10efb8dd3975e50,
        ],
        elts: vec![
            Element::Op(OpElement {
                ops: vec![Op {
                    typ: OperationType::Write,
                    rand_data: false,
                    rand_mask: false,
                    data_pattern_idx: 0,
                    mask_pattern_idx: 1,
                    flip_data: false,
                }],
                seq: OpElementSeq::Up,
            }),
            Element::Op(OpElement {
                ops: vec![
                    Op {
                        typ: OperationType::Read,
                        rand_data: false,
                        rand_mask: false,
                        data_pattern_idx: 0,
                        mask_pattern_idx: 1,
                        flip_data: false,
                    },
                    Op {
                        typ: OperationType::Write,
                        rand_data: false,
                        rand_mask: false,
                        data_pattern_idx: 0,
                        mask_pattern_idx: 1,
                        flip_data: true,
                    },
                ],
                seq: OpElementSeq::Up,
            }),
            Element::Op(OpElement {
                ops: vec![
                    Op {
                        typ: OperationType::Read,
                        rand_data: false,
                        rand_mask: false,
                        data_pattern_idx: 0,
                        mask_pattern_idx: 1,
                        flip_data: true,
                    },
                    Op {
                        typ: OperationType::Write,
                        rand_data: false,
                        rand_mask: false,
                        data_pattern_idx: 0,
                        mask_pattern_idx: 1,
                        flip_data: false,
                    },
                ],
                seq: OpElementSeq::Down,
            }),
            Element::Op(OpElement {
                ops: vec![Op {
                    typ: OperationType::Read,
                    rand_data: false,
                    rand_mask: false,
                    data_pattern_idx: 0,
                    mask_pattern_idx: 1,
                    flip_data: false,
                }],
                seq: OpElementSeq::Up,
            }),
        ],
        cycle_limit: u64::MAX,
        stop_on_failure: true,
    }
}

pub enum OperationType {
    Read,
    Write,
    Rand,
}

pub enum OpElementSeq {
    Up,
    Down,
    Rand(u64),
}

pub enum InnerDim {
    Row,
    Col,
}

pub struct Op {
    pub typ: OperationType,
    pub rand_data: bool,
    pub rand_mask: bool,
    pub data_pattern_idx: usize,
    pub mask_pattern_idx: usize,
    pub flip_data: bool,
}

pub struct OpElement {
    pub ops: Vec<Op>,
    pub seq: OpElementSeq,
}

pub struct WaitElement {
    pub cycles: u64,
}

pub enum Element {
    Op(OpElement),
    Wait(WaitElement),
}

pub struct BistController<I> {
    pub intf: I,
    pub sram_id: u64,
    pub rows: u64,
    pub cols: u64,
    pub inner_dim: InnerDim,
    pub rand_seed: u64,
    pub sig_seed: u128,
    pub patterns: Vec<u128>,
    pub elts: Vec<Element>,
    pub cycle_limit: u64,
    pub stop_on_failure: bool,
}

#[derive(Debug, Clone)]
pub struct BistResult {
    pub fail: bool,
    pub fail_cycle: u64,
    pub expected: u128,
    pub received: u128,
    pub signature: u128,
}

impl<I> BistController<I> {
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

    pub fn pack_elts(&self) -> [u64; 16] {}
}

impl<I: MemoryIntf> BistController<I> {
    pub fn execute(&mut self) -> BistResult {
        self.validate();
        self.init();
        self.execute_inner();
        self.read_result()
    }

    fn init(&mut self) {
        self.intf.write(SRAM_ID, self.sram_id);
        self.intf.write(SRAM_SEL, SRAM_SEL_BIST);
        self.intf.write(BIST_RAND_SEED, self.rand_seed);
        for i in 1u64..5 {
            self.intf.write(BIST_RAND_SEED + 8 * i, 0);
        }
        self.intf.write128(BIST_SIG_SEED, self.sig_seed);
        self.intf.write(BIST_MAX_ROW_ADDR, self.rows - 1);
        self.intf.write(BIST_MAX_COL_ADDR, self.cols - 1);
        self.intf.write(BIST_INNER_DIM, self.inner_dim.encode());
        let mut packed = [0u64; 16];
        for (i, elt) in self.elts.iter().enumerate() {
            let encoded = elt.encode();
            let bit_offset = ELEMENT_WIDTH * i;
            let word = bit_offset / 64;
            let bit = bit_offset % 64;
            let lo = encoded as u64;
            let hi = (encoded >> 64) as u64;
            if bit == 0 {
                packed[word] |= lo;
                packed[word + 1] |= hi;
            } else {
                packed[word] |= lo << bit;
                packed[word + 1] |= (lo >> (64 - bit)) | (hi << bit);
                if word + 2 < packed.len() {
                    packed[word + 2] |= hi >> (64 - bit);
                }
            }
        }
        for (i, &word) in packed.iter().enumerate() {
            self.intf.write(BIST_ELEMENT_SEQUENCE + 8 * i as u64, word);
        }
        for (i, &pat) in self.patterns.iter().enumerate() {
            self.intf.write128(BIST_PATTERN_TABLE + 16 * i as u64, pat);
        }
        self.intf
            .write(BIST_MAX_ELEMENT_IDX, (self.elts.len() - 1) as u64);
        self.intf.write(BIST_CYCLE_LIMIT, self.cycle_limit);
        self.intf
            .write(BIST_STOP_ON_FAILURE, self.stop_on_failure as u64);
    }

    fn execute_inner(&mut self) {
        self.intf.write(EX, 1);
        while self.intf.read(DONE) & 0x1 == 0 {}
    }

    fn read_result(&mut self) -> BistResult {
        BistResult {
            fail: self.intf.read(BIST_FAIL) & 0x1 != 0,
            fail_cycle: self.intf.read(BIST_FAIL_CYCLE),
            expected: self.intf.read128(BIST_EXPECTED),
            received: self.intf.read128(BIST_RECEIVED),
            signature: self.intf.read128(BIST_SIGNATURE),
        }
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
            Self::Op(e) => e.encode(),
            Self::Wait(e) => ((e.cycles as u128) << 107) | (1u128 << 121),
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
        let num_addrs: u128 = match self {
            Self::Up | Self::Down => 0,
            Self::Rand(n) => *n as u128,
        };
        self.encode() | (num_addrs << 2)
    }
}

impl OperationType {
    fn encode(&self) -> u128 {
        match self {
            Self::Read => 0,
            Self::Write => 1,
            Self::Rand => 2,
        }
    }
}

impl Op {
    fn encode(&self) -> u128 {
        let mut val = self.typ.encode();
        val |= (self.rand_data as u128) << 2;
        val |= (self.rand_mask as u128) << 3;
        val |= (self.data_pattern_idx as u128) << 4;
        val |= (self.mask_pattern_idx as u128) << 7;
        val |= (self.flip_data as u128) << 10;
        val
    }
}

impl OpElement {
    pub fn encode(&self) -> u128 {
        let mut val: u128 = 0;
        for (i, op) in self.ops.iter().enumerate() {
            val |= op.encode() << (11 * i);
        }
        val |= ((self.ops.len() - 1) as u128) << (OPERATIONS_PER_ELEMENT * 11);
        val |= self.seq.encode_full() << (OPERATIONS_PER_ELEMENT * 11 + 3);
        val
    }
}

#[cfg(test)]
mod tests {
    const BASIC_BIST_PACKED: [u64; 8] = {};

    #[test]
    fn basic_bist() {}
}
