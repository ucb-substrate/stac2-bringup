use crate::memory::*;
use crate::tests::SRAM_SIZES;
use crate::MemoryIntf;

const ELEMENT_TABLE_LENGTH: usize = 8;
const OPERATIONS_PER_ELEMENT: usize = 8;
const PATTERN_TABLE_LENGTH: usize = 8;
const MAX_ROW_ADDR_WIDTH: usize = 11;
const MAX_COL_ADDR_WIDTH: usize = 3;
const DATA_WIDTH: usize = 128;
const RAND_ADDR_WIDTH: usize = 14;
const ELEMENT_WIDTH: usize = 122;

const SRAM_SEL_BIST: u64 = 1;

pub fn basic_bist<I>(intf: I, id: u64) -> BistController<I> {
    let size = SRAM_SIZES[id as usize];
    BistController {
        intf,
        sram_id: id,
        rows: size.rows() as u64,
        mux_ratio: size.mux_ratio() as u64,
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

pub fn march_cm_bist<I>(intf: I, id: u64) -> BistController<I> {
    let size = SRAM_SIZES[id as usize];
    BistController {
        intf,
        sram_id: id,
        rows: size.rows() as u64,
        mux_ratio: size.mux_ratio() as u64,
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
                seq: OpElementSeq::Down,
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

pub fn march_b_bist<I>(intf: I, id: u64) -> BistController<I> {
    let size = SRAM_SIZES[id as usize];
    BistController {
        intf,
        sram_id: id,
        rows: size.rows() as u64,
        mux_ratio: size.mux_ratio() as u64,
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
                    Op {
                        typ: OperationType::Write,
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
    pub mux_ratio: u64,
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
        assert!(self.mux_ratio > 0);
        assert!(self.rows <= 2u64.strict_pow(MAX_ROW_ADDR_WIDTH as u32));
        assert!(self.mux_ratio <= 2u64.strict_pow(MAX_COL_ADDR_WIDTH as u32));
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

    fn encode_elts(&self) -> [u64; 16] {
        let mut packed = [0u64; 16];
        let zero_elt = Element::Wait(WaitElement { cycles: 0 });
        for i in 0..ELEMENT_TABLE_LENGTH {
            let elt = self.elts.get(i).unwrap_or(&zero_elt);
            let encoded = elt.encode();
            let bit_offset = ELEMENT_WIDTH * i;
            let word = bit_offset / 64;
            let bit = bit_offset % 64;
            let lo = encoded as u64;
            let hi = (encoded >> 64) as u64;
            if bit == 0 {
                packed[word] |= lo;
                if word + 1 < packed.len() {
                    packed[word + 1] |= hi;
                }
            } else {
                packed[word] |= lo << bit;
                if word + 1 < packed.len() {
                    packed[word + 1] |= (lo >> (64 - bit)) | (hi << bit);
                }
                if word + 2 < packed.len() {
                    packed[word + 2] |= hi >> (64 - bit);
                }
            }
        }
        packed
    }
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
        self.intf.write(BIST_MAX_COL_ADDR, self.mux_ratio - 1);
        self.intf.write(BIST_INNER_DIM, self.inner_dim.encode());
        let elts = self.encode_elts();
        for (i, &word) in elts.iter().enumerate() {
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

const ZERO_OP_ENCODED: u128 = 0x001;

fn zero_op_element_encoded() -> u128 {
    let mut val: u128 = 0;
    for i in 0..OPERATIONS_PER_ELEMENT {
        val |= ZERO_OP_ENCODED << (19 + 11 * i);
    }
    val
}

impl Element {
    pub fn encode(&self) -> u128 {
        match self {
            Self::Op(e) => 1u128 | (e.encode() << 15),
            Self::Wait(e) => ((e.cycles as u128) << 1) | (zero_op_element_encoded() << 15),
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
    pub fn num_addrs(&self) -> u128 {
        match self {
            Self::Up | Self::Down => 0,
            Self::Rand(n) => *n as u128,
        }
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
        let mut val: u128 = 0;
        val |= self.typ.encode() << 9;
        val |= (self.rand_data as u128) << 8;
        val |= (self.rand_mask as u128) << 7;
        val |= (self.data_pattern_idx as u128) << 4;
        val |= (self.mask_pattern_idx as u128) << 1;
        val |= !self.flip_data as u128;
        val
    }
}

impl OpElement {
    pub fn encode(&self) -> u128 {
        let mut val: u128 = 0;
        for i in 0..OPERATIONS_PER_ELEMENT {
            let op_encoded = if i < self.ops.len() {
                self.ops[i].encode()
            } else {
                ZERO_OP_ENCODED
            };
            val |= op_encoded << (19 + 11 * i);
        }
        val |= ((self.ops.len() - 1) as u128) << 16;
        val |= self.seq.encode() << 14;
        val |= self.seq.num_addrs();
        val
    }
}

#[cfg(test)]
mod tests {
    use crate::bist::{
        basic_bist, BistController, Element, InnerDim, Op, OpElement, OpElementSeq, OperationType,
    };

    const BASIC_BIST_PACKED: [u64; 16] = [
        0x100280c00000001,
        0x400801002004008,
        0x2005010032000000,
        0x10020040080100,
        0x801406008a0000,
        0x400801002004,
        0x1002004008030000,
        0x20040080,
        0x40080100200400,
        0x801002,
        0x801002004008010,
        0x4000000000020040,
        0x20040080100200,
        0x100000000000801,
        0x400801002004008,
        0x20,
    ];

    const COPIED_MARCH_BIST_PACKED: [u64; 16] = [
        0x320628cf80000001,
        0x4188310620c4182,
        0x8c818a33e000000,
        0x10620c41883106,
        0x182320628cf80000,
        0x4188310620c4,
        0x10608c818a33e000,
        0x620c41883,
        0x40080100200400,
        0x801002,
        0x801002004008010,
        0x4000000000020040,
        0x20040080100200,
        0x100000000000801,
        0x400801002004008,
        0x20,
    ];

    fn copied_march_bist<I>(intf: I) -> BistController<I> {
        let march_element = || {
            Element::Op(OpElement {
                ops: vec![
                    Op {
                        typ: OperationType::Write,
                        rand_data: false,
                        rand_mask: false,
                        data_pattern_idx: 3,
                        mask_pattern_idx: 1,
                        flip_data: false,
                    },
                    Op {
                        typ: OperationType::Read,
                        rand_data: false,
                        rand_mask: false,
                        data_pattern_idx: 3,
                        mask_pattern_idx: 0,
                        flip_data: false,
                    },
                    Op {
                        typ: OperationType::Write,
                        rand_data: false,
                        rand_mask: false,
                        data_pattern_idx: 3,
                        mask_pattern_idx: 1,
                        flip_data: true,
                    },
                    Op {
                        typ: OperationType::Read,
                        rand_data: false,
                        rand_mask: false,
                        data_pattern_idx: 3,
                        mask_pattern_idx: 0,
                        flip_data: true,
                    },
                    Op {
                        typ: OperationType::Read,
                        rand_data: false,
                        rand_mask: false,
                        data_pattern_idx: 3,
                        mask_pattern_idx: 0,
                        flip_data: false,
                    },
                    Op {
                        typ: OperationType::Read,
                        rand_data: false,
                        rand_mask: false,
                        data_pattern_idx: 3,
                        mask_pattern_idx: 0,
                        flip_data: false,
                    },
                    Op {
                        typ: OperationType::Read,
                        rand_data: false,
                        rand_mask: false,
                        data_pattern_idx: 3,
                        mask_pattern_idx: 0,
                        flip_data: false,
                    },
                    Op {
                        typ: OperationType::Read,
                        rand_data: false,
                        rand_mask: false,
                        data_pattern_idx: 3,
                        mask_pattern_idx: 0,
                        flip_data: false,
                    },
                ],
                seq: OpElementSeq::Up,
            })
        };

        BistController {
            intf,
            sram_id: 0,
            rows: 1,
            mux_ratio: 1,
            inner_dim: InnerDim::Row,
            rand_seed: 1,
            sig_seed: 1,
            patterns: vec![
                0,
                0xffffffff,
                0x5f1a950d9af236fcc76148fef684be4e,
                0xa0e56af2650dc903389eb701097b41b1,
            ],
            elts: vec![
                march_element(),
                march_element(),
                march_element(),
                march_element(),
            ],
            cycle_limit: 0,
            stop_on_failure: true,
        }
    }

    #[test]
    fn basic_bist_packs_correctly() {
        assert_eq!(basic_bist((), 0).encode_elts(), BASIC_BIST_PACKED);
    }

    #[test]
    fn copied_march_bist_packs_correctly() {
        assert_eq!(
            copied_march_bist(()).encode_elts(),
            COPIED_MARCH_BIST_PACKED
        );
    }
}
