use crate::MemoryIntf;
use crate::memory::*;
use crate::tests::SRAM_SIZES;

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
        data_width: size.width(),
        mask_granularity: size.width() / size.mask_width(),
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
        data_width: size.width(),
        mask_granularity: size.width() / size.mask_width(),
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
        data_width: size.width(),
        mask_granularity: size.width() / size.mask_width(),
    }
}

pub fn rand_bist<I>(intf: I, id: u64) -> BistController<I> {
    let size = SRAM_SIZES[id as usize];
    let mut elts = Vec::with_capacity(8);
    elts.push(Element::Op(OpElement {
        ops: vec![Op {
            typ: OperationType::Write,
            rand_data: false,
            rand_mask: false,
            data_pattern_idx: 0,
            mask_pattern_idx: 1,
            flip_data: false,
        }],
        seq: OpElementSeq::Up,
    }));
    for _ in 0..7 {
        elts.push(Element::Op(OpElement {
            ops: vec![
                Op {
                    typ: OperationType::Rand,
                    rand_data: true,
                    rand_mask: true,
                    data_pattern_idx: 0,
                    mask_pattern_idx: 0,
                    flip_data: false,
                };
                8
            ],
            seq: OpElementSeq::Rand(16384),
        }));
    }
    BistController {
        intf,
        sram_id: id,
        rows: size.rows() as u64,
        mux_ratio: size.mux_ratio() as u64,
        inner_dim: InnerDim::Col,
        rand_seed: 0x22,
        sig_seed: 0x12345678,
        patterns: vec![0, 0xffffffffffffffffffffffffffffffff],
        elts,
        cycle_limit: u64::MAX,
        stop_on_failure: true,
        data_width: size.width(),
        mask_granularity: size.width() / size.mask_width(),
    }
}

#[derive(Debug, Clone)]
pub enum OperationType {
    Read,
    Write,
    Rand,
}

#[derive(Debug, Clone)]
pub enum OpElementSeq {
    Up,
    Down,
    Rand(u64),
}

#[derive(Debug, Clone)]
pub enum InnerDim {
    Row,
    Col,
}

#[derive(Debug, Clone)]
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
    /// Data width of the target SRAM in bits (e.g. 32 for a 32-bit SRAM).
    pub data_width: u32,
    /// Number of data bits controlled by each write-mask bit (wmask granularity).
    pub mask_granularity: u32,
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

    /// Simulate the BIST on a defect-free SRAM and return the expected MISR signature.
    ///
    /// Models `BistTop.scala` exactly: runs the `ProgrammableBist` address/data sequence,
    /// applies writes and reads to a software SRAM, and accumulates all read results through
    /// the 128-bit Fibonacci MISR (taps {128,127,126,121}, initial state = `sig_seed`).
    ///
    /// Panics if any element or operation uses randomised addresses/data (not yet supported).
    pub fn expected_signature(&self) -> u128 {
        let sram_mask = bitmask_u128(self.data_width);
        let mask_width = self.data_width / self.mask_granularity;

        let total_words = (self.rows * self.mux_ratio) as usize;
        let mut sram = vec![0u128; total_words];
        let mut misr = self.sig_seed;

        let max_row = self.rows - 1;
        let max_col = self.mux_ratio - 1;
        let max_elt = self.elts.len() - 1;

        let mut elt_idx = 0usize;
        let (mut row, mut col) = elt_start_addr(&self.elts[0], max_row, max_col);

        'done: loop {
            match &self.elts[elt_idx] {
                Element::Wait(_) => {
                    // Wait elements produce no SRAM ops; skip straight to the next element.
                    elt_idx += 1;
                    if elt_idx > max_elt {
                        break 'done;
                    }
                    (row, col) = elt_start_addr(&self.elts[elt_idx], max_row, max_col);
                }

                Element::Op(op_elt) => {
                    assert!(
                        !matches!(op_elt.seq, OpElementSeq::Rand(_)),
                        "random address order is not supported in expected_signature"
                    );

                    let up = matches!(op_elt.seq, OpElementSeq::Up);
                    let row_end: u64 = if up { max_row } else { 0 };
                    let col_end: u64 = if up { max_col } else { 0 };
                    let row_start: u64 = if up { 0 } else { max_row };
                    let col_start: u64 = if up { 0 } else { max_col };

                    loop {
                        let addr = (row * self.mux_ratio + col) as usize;

                        for op in op_elt.ops.iter() {
                            assert!(
                                !op.rand_data && !op.rand_mask,
                                "random data/mask is not supported in expected_signature"
                            );
                            assert!(
                                !matches!(op.typ, OperationType::Rand),
                                "random op type is not supported in expected_signature"
                            );

                            let raw = self.patterns[op.data_pattern_idx];
                            let data = if op.flip_data {
                                !raw & sram_mask
                            } else {
                                raw & sram_mask
                            };
                            let mask = self.patterns[op.mask_pattern_idx];

                            match op.typ {
                                OperationType::Write => {
                                    sram[addr] = bist_masked_write(
                                        sram[addr],
                                        data,
                                        mask,
                                        mask_width,
                                        self.mask_granularity,
                                    );
                                }
                                OperationType::Read => {
                                    misr = misr_step_128(misr, sram[addr] & sram_mask);
                                }
                                OperationType::Rand => unreachable!(),
                            }
                        }

                        // Advance address (mirrors the Chisel state-update logic).
                        let rows_done = row == row_end;
                        let cols_done = col == col_end;

                        if rows_done && cols_done {
                            // Last address for this element – move to the next one.
                            elt_idx += 1;
                            if elt_idx > max_elt {
                                break 'done;
                            }
                            (row, col) = elt_start_addr(&self.elts[elt_idx], max_row, max_col);
                            break;
                        }

                        match self.inner_dim {
                            InnerDim::Col => {
                                if cols_done {
                                    col = col_start;
                                    row = if up { row + 1 } else { row - 1 };
                                } else {
                                    col = if up { col + 1 } else { col - 1 };
                                }
                            }
                            InnerDim::Row => {
                                if rows_done {
                                    row = row_start;
                                    col = if up { col + 1 } else { col - 1 };
                                } else {
                                    row = if up { row + 1 } else { row - 1 };
                                }
                            }
                        }
                    }
                }
            }
        }

        misr
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

// ---------------------------------------------------------------------------
// MISR helpers (models MaxPeriodFibonacciMISR from MISR.scala, width=128)
// ---------------------------------------------------------------------------

/// One step of the 128-bit Fibonacci MISR.
///
/// The hardware delta function (FibonacciMISR.delta) is:
///   new[0]  = in[0] ^ XOR(s[tap-1] for tap in taps)
///   new[i]  = s[i-1] ^ in[i]   for i = 1..127
///
/// Taps for width 128 (from MISR.tapsMaxPeriod): {128, 127, 126, 121}.
fn misr_step_128(state: u128, input: u128) -> u128 {
    // taps {128,127,126,121} → 0-indexed bit positions {127,126,125,120}
    let tap_xor = ((state >> 127) ^ (state >> 126) ^ (state >> 125) ^ (state >> 120)) & 1;
    let new_s0 = (input & 1) ^ tap_xor;
    // bits 1..127: new[i] = s[i-1] ^ in[i]  (left-shift state, then XOR input)
    ((state << 1) ^ input) & !1u128 | new_s0
}

/// Return the starting (row, col) for an element based on its direction.
fn elt_start_addr(elt: &Element, max_row: u64, max_col: u64) -> (u64, u64) {
    match elt {
        Element::Op(e) => match e.seq {
            OpElementSeq::Up | OpElementSeq::Rand(_) => (0, 0),
            OpElementSeq::Down => (max_row, max_col),
        },
        Element::Wait(_) => (0, 0),
    }
}

/// Apply a masked write to an SRAM word, mirroring the hardware mask-expansion
/// logic (each of the `mask_width` mask bits enables writing `mask_gran` data bits).
fn bist_masked_write(
    old: u128,
    data: u128,
    mask_raw: u128,
    mask_width: u32,
    mask_gran: u32,
) -> u128 {
    let actual_mask = mask_raw & bitmask_u128(mask_width);
    let mut result = old;
    for i in 0..mask_width {
        if (actual_mask >> i) & 1 == 1 {
            let lo = i * mask_gran;
            let hi = lo + mask_gran;
            // Build a mask covering bits lo..hi-1.
            let lo_mask = if lo == 0 { 0u128 } else { (1u128 << lo) - 1 };
            let hi_mask = if hi >= 128 {
                u128::MAX
            } else {
                (1u128 << hi) - 1
            };
            let group_mask = hi_mask ^ lo_mask;
            result = (result & !group_mask) | (data & group_mask);
        }
    }
    result
}

/// Return a u128 with the lower `n` bits set (handles n == 128 without overflow).
fn bitmask_u128(n: u32) -> u128 {
    if n >= 128 {
        u128::MAX
    } else {
        (1u128 << n) - 1
    }
}

// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::bist::{
        BistController, Element, InnerDim, Op, OpElement, OpElementSeq, OperationType, WaitElement,
        basic_bist, misr_step_128,
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
            data_width: 128,
            mask_granularity: 8,
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

    // -----------------------------------------------------------------------
    // expected_signature tests
    // -----------------------------------------------------------------------

    /// Single-address SRAM (rows=1, mux_ratio=1), write 0 then read 0.
    /// MISR taps for width 128: {128,127,126,121}.
    ///
    /// sig_seed = 1 = 0x00...001
    /// Read value = 0.
    ///   tap_xor = bits 127,126,125,120 of 1 = all zero → 0
    ///   new_s0  = in[0] ^ tap_xor = 0 ^ 0 = 0
    ///   upper   = (1 << 1) ^ 0 = 2  → bits 1..127 = old bit 0 = 0 apart from bit 1
    ///   result  = 2 & !1 | 0 = 2
    #[test]
    fn single_addr_write_then_read() {
        let ctrl = BistController {
            intf: (),
            sram_id: 0,
            rows: 1,
            mux_ratio: 1,
            inner_dim: InnerDim::Col,
            rand_seed: 1,
            sig_seed: 1,
            patterns: vec![0, u128::MAX],
            elts: vec![
                Element::Op(OpElement {
                    ops: vec![Op {
                        typ: OperationType::Write,
                        rand_data: false,
                        rand_mask: false,
                        data_pattern_idx: 0, // write 0
                        mask_pattern_idx: 1, // full mask
                        flip_data: false,
                    }],
                    seq: OpElementSeq::Up,
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
            stop_on_failure: false,
            data_width: 128,
            mask_granularity: 8,
        };

        let expected = misr_step_128(1, 0); // one read of 0 into seed=1
        assert_eq!(ctrl.expected_signature(), expected);
        assert_eq!(expected, 2);
    }

    /// Two reads: write all-ones then read twice.
    /// Verifies the MISR chains correctly across multiple reads.
    #[test]
    fn single_addr_write_ones_read_twice() {
        let ctrl = BistController {
            intf: (),
            sram_id: 0,
            rows: 1,
            mux_ratio: 1,
            inner_dim: InnerDim::Col,
            rand_seed: 1,
            sig_seed: 1,
            patterns: vec![0, u128::MAX],
            elts: vec![
                Element::Op(OpElement {
                    ops: vec![Op {
                        typ: OperationType::Write,
                        rand_data: false,
                        rand_mask: false,
                        data_pattern_idx: 1, // write all-ones
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
                            data_pattern_idx: 1,
                            mask_pattern_idx: 1,
                            flip_data: false,
                        },
                        Op {
                            typ: OperationType::Read,
                            rand_data: false,
                            rand_mask: false,
                            data_pattern_idx: 1,
                            mask_pattern_idx: 1,
                            flip_data: false,
                        },
                    ],
                    seq: OpElementSeq::Up,
                }),
            ],
            cycle_limit: u64::MAX,
            stop_on_failure: false,
            data_width: 128,
            mask_granularity: 8,
        };

        // Two reads of u128::MAX starting from seed=1.
        let s1 = misr_step_128(1, u128::MAX);
        let s2 = misr_step_128(s1, u128::MAX);
        assert_eq!(ctrl.expected_signature(), s2);
    }

    /// Wait element between two Op elements does not affect the signature.
    #[test]
    fn wait_element_is_transparent() {
        let base = BistController {
            intf: (),
            sram_id: 0,
            rows: 1,
            mux_ratio: 1,
            inner_dim: InnerDim::Col,
            rand_seed: 1,
            sig_seed: 1,
            patterns: vec![0, u128::MAX],
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
            stop_on_failure: false,
            data_width: 128,
            mask_granularity: 8,
        };

        // Insert a wait element between write and read – signature must be identical.
        let with_wait = BistController {
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
                Element::Wait(WaitElement { cycles: 100 }),
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
            intf: (),
            sram_id: 0,
            rows: 1,
            mux_ratio: 1,
            inner_dim: InnerDim::Col,
            rand_seed: 1,
            sig_seed: 1,
            patterns: vec![0, u128::MAX],
            cycle_limit: u64::MAX,
            stop_on_failure: false,
            data_width: 128,
            mask_granularity: 8,
        };

        assert_eq!(base.expected_signature(), with_wait.expected_signature());
    }
}
