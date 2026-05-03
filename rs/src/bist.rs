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
            seq: OpElementSeq::Rand(16383),
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
    /// Models `BistTop.scala` exactly: runs the `ProgrammableBist` address/data/wen sequence
    /// (including the 271-bit Fibonacci LFSR for random modes), applies writes and reads to a
    /// software SRAM, and accumulates all read results through the 128-bit Fibonacci MISR
    /// (taps {128,127,126,121}, initial state = `sig_seed`).
    pub fn expected_signature(&self) -> u128 {
        let sram_mask = bitmask_u128(self.data_width);
        let mask_width = self.data_width / self.mask_granularity;

        let total_words = (self.rows * self.mux_ratio) as usize;
        let mut sram = vec![0u128; total_words];
        let mut misr = self.sig_seed;
        let mut lfsr = Lfsr271::new(self.rand_seed);

        let row_addr_bits = log2_ceil(self.rows);
        let col_addr_bits = log2_ceil(self.mux_ratio);
        let row_mask = bitmask_u64(row_addr_bits);
        let col_mask = bitmask_u64(col_addr_bits);

        let max_row = self.rows - 1;
        let max_col = self.mux_ratio - 1;

        for elt_idx in 0..self.elts.len() {
            match &self.elts[elt_idx] {
                Element::Wait(we) => {
                    for _ in 0..we.cycles {
                        lfsr.step();
                    }
                }

                Element::Op(op_elt) => {
                    // Helper: execute one op at `addr`, advancing LFSR and MISR as needed.
                    // (inline closure avoids borrow conflicts on `sram`/`misr`/`lfsr`)
                    if let OpElementSeq::Rand(n) = op_elt.seq {
                        // Random-address element: io.row/col are wired combinatorially from
                        // the live LFSR (ProgrammableBist.scala: `io.row := Mux(randAddrOrder,
                        // randRow, rowCounter)`), so the address changes on every op cycle.
                        for _ in 0..n {
                            for op in op_elt.ops.iter() {
                                // Re-derive address from the current LFSR state each op.
                                let row = lfsr.rand_row() & row_mask;
                                let col = lfsr.rand_col() & col_mask;
                                let addr = (row * self.mux_ratio + col) as usize;

                                let is_write = match op.typ {
                                    OperationType::Write => true,
                                    OperationType::Read => false,
                                    OperationType::Rand => lfsr.rand_wen(),
                                };
                                let raw_data = if op.rand_data {
                                    lfsr.rand_data() & sram_mask
                                } else {
                                    let raw = self.patterns[op.data_pattern_idx];
                                    if op.flip_data { !raw & sram_mask } else { raw & sram_mask }
                                };
                                let raw_mask = if op.rand_mask {
                                    lfsr.rand_mask()
                                } else {
                                    self.patterns[op.mask_pattern_idx]
                                };

                                if is_write {
                                    sram[addr] = bist_masked_write(
                                        sram[addr], raw_data, raw_mask,
                                        mask_width, self.mask_granularity,
                                    );
                                } else {
                                    misr = misr_step_128(misr, sram[addr] & sram_mask);
                                }
                                lfsr.step();
                            }
                        }
                    } else {
                        // Sequential (Up/Down) element.
                        let up = matches!(op_elt.seq, OpElementSeq::Up);
                        let row_end: u64 = if up { max_row } else { 0 };
                        let col_end: u64 = if up { max_col } else { 0 };
                        let row_start: u64 = if up { 0 } else { max_row };
                        let col_start: u64 = if up { 0 } else { max_col };
                        let (mut row, mut col) =
                            elt_start_addr(&self.elts[elt_idx], max_row, max_col);

                        loop {
                            let addr = (row * self.mux_ratio + col) as usize;

                            for op in op_elt.ops.iter() {
                                let is_write = match op.typ {
                                    OperationType::Write => true,
                                    OperationType::Read => false,
                                    OperationType::Rand => lfsr.rand_wen(),
                                };
                                let raw_data = if op.rand_data {
                                    lfsr.rand_data() & sram_mask
                                } else {
                                    let raw = self.patterns[op.data_pattern_idx];
                                    if op.flip_data { !raw & sram_mask } else { raw & sram_mask }
                                };
                                let raw_mask = if op.rand_mask {
                                    lfsr.rand_mask()
                                } else {
                                    self.patterns[op.mask_pattern_idx]
                                };

                                if is_write {
                                    sram[addr] = bist_masked_write(
                                        sram[addr], raw_data, raw_mask,
                                        mask_width, self.mask_granularity,
                                    );
                                } else {
                                    misr = misr_step_128(misr, sram[addr] & sram_mask);
                                }
                                lfsr.step();
                            }

                            let rows_done = row == row_end;
                            let cols_done = col == col_end;
                            if rows_done && cols_done {
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

fn bitmask_u64(n: u32) -> u64 {
    if n >= 64 { u64::MAX } else { (1u64 << n) - 1 }
}

/// Ceiling log base 2. Returns 0 for n ≤ 1.
fn log2_ceil(n: u64) -> u32 {
    if n <= 1 { 0 } else { 64 - (n - 1).leading_zeros() }
}

// ---------------------------------------------------------------------------
// 271-bit Fibonacci LFSR (models MaxPeriodFibonacciLFSR from ProgrammableBist.scala)
// ---------------------------------------------------------------------------
//
// Taps: {271, 213} → feedback = s[270] ^ s[212], new[i] = s[i-1].
// State layout across five u64 words (word k holds bits 64k..64k+63):
//   word 0: bits   0..63   → randData[63:0]
//   word 1: bits  64..127  → randData[127:64]
//   word 2: bits 128..191  → randMask[63:0]
//   word 3: bits 192..255  → randMask[127:64]
//   word 4: bits 256..270  → randRow[10:0] | randCol[2:0] | randWen (15 bits)
struct Lfsr271 {
    state: [u64; 5],
}

impl Lfsr271 {
    fn new(seed: u64) -> Self {
        let mut state = [0u64; 5];
        state[0] = seed;
        Self { state }
    }

    fn step(&mut self) {
        let s = &mut self.state;
        // feedback = s[270] ^ s[212]
        let feedback = ((s[4] >> 14) ^ (s[3] >> 20)) & 1;
        // left-shift the whole 271-bit register (new[i] = old[i-1])
        s[4] = ((s[4] << 1) | (s[3] >> 63)) & 0x7FFF;
        s[3] = (s[3] << 1) | (s[2] >> 63);
        s[2] = (s[2] << 1) | (s[1] >> 63);
        s[1] = (s[1] << 1) | (s[0] >> 63);
        s[0] = (s[0] << 1) | feedback;
    }

    fn rand_data(&self) -> u128 {
        (self.state[0] as u128) | ((self.state[1] as u128) << 64)
    }

    fn rand_mask(&self) -> u128 {
        (self.state[2] as u128) | ((self.state[3] as u128) << 64)
    }

    fn rand_row(&self) -> u64 {
        self.state[4] & 0x7FF
    }

    fn rand_col(&self) -> u64 {
        (self.state[4] >> 11) & 0x7
    }

    fn rand_wen(&self) -> bool {
        (self.state[4] >> 14) & 1 == 1
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

    /// Verify that `expected_signature` handles all random BIST modes without panicking,
    /// produces a deterministic result, and changes when the random seed changes.
    #[test]
    fn rand_bist_expected_signature_is_deterministic() {
        use crate::bist::rand_bist;
        let sig = rand_bist((), 0).expected_signature();
        assert_eq!(sig, rand_bist((), 0).expected_signature(), "not deterministic");
        assert_ne!(sig, 0);
        assert_ne!(sig, 0x12345678u128, "signature should not equal sig_seed");

        // Changing rand_seed must change the signature.
        let mut b = rand_bist((), 0);
        b.rand_seed = 0xdeadbeef;
        assert_ne!(sig, b.expected_signature());
    }

    /// Check the software model against signatures collected from the chip for all 21 SRAMs.
    #[test]
    fn rand_bist_matches_chip_signatures() {
        use crate::bist::rand_bist;
        const CHIP_SIGS: [u128; 21] = [
            203876398793974784392049286733865567426, // SRAM  0
            143816083980602734529858358810098685540, // SRAM  1
            108471526183210361328190071862838307380, // SRAM  2
            181718990368149921933182788912415723492, // SRAM  3
            271625274696836046971172272235168328567, // SRAM  4
            173780344703431218152445207057245610815, // SRAM  5
             33932211709387350024616149407249051816, // SRAM  6
            236181482163255396159126307420785725794, // SRAM  7
            253796419754034000090550023669733434008, // SRAM  8
             10045536061912160697293438420962870323, // SRAM  9
            311794336798262176063187235549129554642, // SRAM 10
            279648850837578680480069666787687318554, // SRAM 11
            188180803326311202792691840632558602861, // SRAM 12
            220770591037702866231525693588572601924, // SRAM 13
             84971561158651698205690231688503025491, // SRAM 14
            233710015981246038562622600083746819379, // SRAM 15
            305026504763097746684158955825412086443, // SRAM 16
            219610186456536458702369254403997161043, // SRAM 17
             14651963960470542285819444396208728108, // SRAM 18
             70264154285946766405024608020502514943, // SRAM 19
            285076337513478781228445413731479395446, // SRAM 20
        ];
        for (id, &expected) in CHIP_SIGS.iter().enumerate() {
            let got = rand_bist((), id as u64).expected_signature();
            assert_eq!(got, expected, "SRAM {id}: model={got} chip={expected}");
        }
    }
}
