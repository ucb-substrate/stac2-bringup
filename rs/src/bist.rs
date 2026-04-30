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
    Rand(usize),
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
    cycles: usize,
}

enum Element {
    Op(OpElement),
    Wait(WaitElement),
}

pub struct BistExecutor<I> {
    intf: I,
    rows: usize,
    cols: usize,
    inner_dim: InnerDim,
    // TODO: seed
    patterns: Vec<u128>,
    elts: Vec<Element>,
    cycle_limit: u64,
}

impl<I> BistExecutor<I> {
    pub fn validate(&self) {
        assert!(self.rows <= 2usize.strict_pow(MAX_ROW_ADDR_WIDTH as u32));
        assert!(self.cols <= 2usize.strict_pow(MAX_COL_ADDR_WIDTH as u32));
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
                        assert!(n < 2usize.strict_pow(RAND_ADDR_WIDTH as u32));
                    }
                }
                Element::Wait(e) => {
                    assert!(e.cycles < 2usize.strict_pow(RAND_ADDR_WIDTH as u32));
                }
            }
        }
    }
}
