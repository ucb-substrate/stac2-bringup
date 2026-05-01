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
    sram_id: u64,
    rows: u64,
    cols: u64,
    inner_dim: InnerDim,
    rand_seed: u64,
    sig_seed: u128,
    patterns: Vec<u128>,
    elts: Vec<Element>,
    cycle_limit: u64,
    stop_on_failure: bool,
}

pub struct BistResult {
    pub fail: bool,
    pub fail_cycle: u64,
    pub expected: u128,
    pub received: u128,
    pub signature: u128,
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
    pub fn execute(&mut self) -> BistResult {
        self.init();
        self.execute();
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
    use super::*;

    #[derive(Default)]
    struct RecordingIntf {
        writes: Vec<(u64, u64)>,
    }

    impl MemoryIntf for RecordingIntf {
        fn read(&mut self, _addr: u64) -> u64 {
            0
        }
        fn write(&mut self, addr: u64, data: u64) {
            self.writes.push((addr, data));
        }
    }

    /// Reference packing routine matching `pack_element_vec` in srambist.c:
    /// the i-th element's 122-bit encoding is laid down at bit offset
    /// `ELEMENT_WIDTH * i` of a 1024-bit (16 × u64) buffer. Done bit-by-bit
    /// so the reference is obviously correct.
    fn pack_elements_reference(elts: &[Element]) -> [u64; 16] {
        let mut out = [0u64; 16];
        for (i, elt) in elts.iter().enumerate() {
            let encoded = elt.encode();
            let bit_offset = ELEMENT_WIDTH * i;
            for b in 0..ELEMENT_WIDTH {
                let bit = ((encoded >> b) & 1) as u64;
                let pos = bit_offset + b;
                out[pos / 64] |= bit << (pos % 64);
            }
        }
        out
    }

    #[test]
    fn init_writes_match_c_run_bist() {
        let elts = vec![
            Element::Op(OpElement {
                ops: vec![
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
                        flip_data: true,
                    },
                ],
                seq: OpElementSeq::Up,
            }),
            Element::Wait(WaitElement { cycles: 42 }),
            Element::Op(OpElement {
                ops: vec![Op {
                    typ: OperationType::Rand,
                    rand_data: true,
                    rand_mask: true,
                    data_pattern_idx: 2,
                    mask_pattern_idx: 0,
                    flip_data: false,
                }],
                seq: OpElementSeq::Rand(1234),
            }),
        ];

        // Use a full pattern table (8 slots): the C `srambist_run_bist`
        // unconditionally writes all 16 u64 words from `pattern_table`,
        // whereas the Rust port only writes `self.patterns.len()` slots.
        // Filling all 8 makes the two memory traces match exactly.
        let patterns: Vec<u128> = (0..PATTERN_TABLE_LENGTH as u128)
            .map(|i| {
                0x1111_2222_3333_4444_5555_6666_7777_8888u128.wrapping_mul(i + 1)
            })
            .collect();

        let sram_id: u64 = 7;
        let rows: u64 = 64;
        let cols: u64 = 4;
        let inner_dim = InnerDim::Col;
        let inner_dim_enc = inner_dim.encode();
        let rand_seed: u64 = 0x1122_3344_5566_7788;
        let sig_seed: u128 = 0xaaaa_bbbb_cccc_dddd_eeee_ffff_0000_1111;
        let cycle_limit: u64 = 1_000_000;
        let stop_on_failure = true;

        let packed = pack_elements_reference(&elts);
        let max_elem_idx = (elts.len() - 1) as u64;

        let mut bist = BistExecutor {
            intf: RecordingIntf::default(),
            sram_id,
            rows,
            cols,
            inner_dim,
            rand_seed,
            sig_seed,
            patterns: patterns.clone(),
            elts,
            cycle_limit,
            stop_on_failure,
        };
        bist.init();

        // Replay the steps in srambist_run_bist (srambist.c). The Rust
        // `MemoryIntf` only exposes u64 writes, so each `reg_write128`
        // becomes a (lo, hi) pair at +0 / +8.
        let mut expected: Vec<(u64, u64)> = Vec::new();
        expected.push((SRAM_ID, sram_id));
        expected.push((SRAM_SEL, SRAM_SEL_BIST));
        expected.push((BIST_RAND_SEED, rand_seed));
        for i in 1u64..5 {
            expected.push((BIST_RAND_SEED + 8 * i, 0));
        }
        expected.push((BIST_SIG_SEED, sig_seed as u64));
        expected.push((BIST_SIG_SEED + 8, (sig_seed >> 64) as u64));
        expected.push((BIST_MAX_ROW_ADDR, rows - 1));
        expected.push((BIST_MAX_COL_ADDR, cols - 1));
        expected.push((BIST_INNER_DIM, inner_dim_enc));
        for i in 0..16u64 {
            expected.push((BIST_ELEMENT_SEQUENCE + 8 * i, packed[i as usize]));
        }
        for (i, p) in patterns.iter().enumerate() {
            let base = BIST_PATTERN_TABLE + 16 * i as u64;
            expected.push((base, *p as u64));
            expected.push((base + 8, (p >> 64) as u64));
        }
        expected.push((BIST_MAX_ELEMENT_IDX, max_elem_idx));
        expected.push((BIST_CYCLE_LIMIT, cycle_limit));
        expected.push((BIST_STOP_ON_FAILURE, stop_on_failure as u64));

        assert_eq!(bist.intf.writes, expected);
    }
}
