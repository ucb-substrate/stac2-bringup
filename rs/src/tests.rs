use std::path::Path;

use crate::BringupState;
use crate::executor::execute;
use crate::pattern::{FixedPattern, Pattern, SramSize};

/// The size of the scratchpad on the STAC-V2 test chip.
const SCRATCHPAD_SIZE: SramSize = SramSize {
    depth: 512,
    width: 64,
    mask_width: 8,
};

pub(crate) const SRAM_SIZES: [SramSize; 22] = [
    SramSize::new(64, 24, 8),
    SramSize::new(64, 32, 8),
    SramSize::new(128, 16, 8),
    SramSize::new(128, 24, 8),
    SramSize::new(128, 32, 8),
    SramSize::new(256, 8, 1),
    SramSize::new(256, 16, 8),
    SramSize::new(256, 32, 8),
    SramSize::new(256, 64, 8),
    SramSize::new(256, 128, 8),
    SramSize::new(512, 8, 1),
    SramSize::new(512, 32, 8),
    SramSize::new(512, 64, 8),
    SramSize::new(512, 128, 8),
    SramSize::new(1024, 8, 1),
    SramSize::new(1024, 32, 8),
    SramSize::new(1024, 64, 8),
    SramSize::new(2048, 8, 1),
    SramSize::new(2048, 32, 8),
    SramSize::new(4096, 8, 1),
    SramSize::new(4096, 32, 8),
    SramSize::new(8192, 32, 8),
];

impl BringupState {
    pub fn mats_plus_tsi_test_sram(&mut self, id: u64) {
        let size = SRAM_SIZES[id as usize];
        let pat = FixedPattern::new(Pattern::mats_plus(), size, 1);
        execute(pat, self.tsi_intf().test_sram_executor(id)).expect("failed to run MATS+ pattern");
    }

    pub fn march_cm_tsi_test_sram(&mut self, id: u64) {
        let size = SRAM_SIZES[id as usize];
        let pat = FixedPattern::new(Pattern::march_cm(), size, 1);
        execute(pat, self.tsi_intf().test_sram_executor(id))
            .expect("failed to run March C- pattern");
    }

    pub fn rand_tsi_test_sram(&mut self, id: u64) {
        let size = SRAM_SIZES[id as usize];
        let pat = FixedPattern::new(Pattern::rand(size.depth as u64 * 8), size, 151);
        execute(pat, self.tsi_intf().test_sram_executor(id)).expect("failed to run random pattern");
    }

    pub fn march_cm_rand_tsi_test_sram_all(&mut self, outdir: impl AsRef<Path>) {
        let outdir = outdir.as_ref();
        std::fs::create_dir_all(outdir).expect("failed to create dir");
        for id in 0..22 {
            let size = SRAM_SIZES[id as usize];
            let pat = FixedPattern::new(Pattern::march_cm(), size, 1);
            let res = execute(pat, self.tsi_intf().test_sram_executor(id));
            std::fs::write(
                outdir.join(format!("sram{id}_march_cm.json")),
                serde_json::to_string_pretty(&res).expect("failed to serialize"),
            )
            .expect("failed to write out file");
            let pat = FixedPattern::new(Pattern::rand(size.depth as u64 * 8), size, 151);
            let res = execute(pat, self.tsi_intf().test_sram_executor(id));
            std::fs::write(
                outdir.join(format!("sram{id}_random.json")),
                serde_json::to_string_pretty(&res).expect("failed to serialize"),
            )
            .expect("failed to write out file");
        }
    }
}

#[cfg(test)]
mod software {
    use crate::{
        IdealExecutor, execute,
        pattern::{FixedPattern, Pattern, SramSize},
    };

    #[test]
    fn mats_plus_ideal_executor() {
        let size = SramSize::new(32, 256, 4);
        let ex = IdealExecutor::new(size);
        let pat = FixedPattern::new(Pattern::mats_plus(), size, 1);
        execute(pat, ex).expect("MATS+ pattern should execute correctly with an ideal executor");
    }

    #[test]
    fn march_cm_ideal_executor() {
        let size = SramSize::new(32, 256, 4);
        let ex = IdealExecutor::new(size);
        let pat = FixedPattern::new(Pattern::march_cm(), size, 1);
        execute(pat, ex).expect("March C- pattern should execute correctly with an ideal executor");
    }

    #[test]
    fn rand4096_ideal_executor() {
        let size = SramSize::new(32, 256, 4);
        let ex = IdealExecutor::new(size);
        let pat = FixedPattern::new(Pattern::rand(4096), size, 1);
        execute(pat, ex)
            .expect("Rand 4096 pattern should execute correctly with an ideal executor");
    }
}
