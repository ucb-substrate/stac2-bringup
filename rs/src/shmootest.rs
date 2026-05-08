use std::ffi::CString;
use std::io::{Read, Write};
use std::path::Path;
use std::thread;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use visa_rs::enums::attribute::{AttrTmoValue, HasAttribute};
use visa_rs::flags::AccessMode;
use visa_rs::{AsResourceManager, DefaultRM, Instrument, TIMEOUT_IMMEDIATE};

use crate::BringupState;
use crate::executor::execute;
use crate::lab::{query, scpi};
use crate::pattern::{FixedPattern, Pattern};
use crate::tests::SRAM_SIZES;

pub const PSU_VISA_ADDR: &str = "USB0::0x2A8D::0x8F01::CN63270183::INSTR";
pub const PSU_CHANNEL: u32 = 1;
pub const CLKGEN_VISA_ADDR: &str = "USB0::0x0957::0x4008::MY428EX302::INSTR";

pub const VDD_VOLTS: &[f64] = &[
    1.20, 1.25, 1.30, 1.35, 1.40, 1.45, 1.50, 1.55, 1.60, 1.65, 1.70, 1.75, 1.80, 1.85, 1.90, 1.95,
    2.00,
];

pub const CLOCK_FREQS_HZ: &[f64] = &[
    15e6, 20e6, 25e6, 30e6, 35e6, 40e6, 45e6, 50e6, 55e6, 60e6, 65e6, 70e6, 75e6, 80e6, 85e6, 90e6,
    95e6, 100e6,
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShmooPoint {
    pub vdd_set_v: f64,
    pub vdd_meas_psu_v: f64,
    pub clock_freq_hz: f64,
    pub pass: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SramShmoo {
    pub sram_id: usize,
    pub points: Vec<Result<ShmooPoint, ()>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShmooResult {
    pub srams: Vec<SramShmoo>,
}

impl BringupState {
    pub fn shmoo_test_all_srams(&mut self, outdir: impl AsRef<Path>) -> ShmooResult {
        let outdir = outdir.as_ref();
        std::fs::create_dir_all(outdir).expect("failed to create output dir");

        println!("PSU:       {}", query(self.lab().psu(), "*IDN?"));
        println!("Clock gen: {}", query(self.lab().clkgen(), "*IDN?"));

        scpi(self.lab().psu(), &format!("OUTP ON,(@{PSU_CHANNEL})"));
        scpi(self.lab().clkgen(), ":VOLT1:HIGH 1.8");
        scpi(self.lab().clkgen(), ":VOLT1:LOW 0.0");
        scpi(self.lab().clkgen(), ":OUTP1 ON");

        let mut per_sram: Vec<Vec<Result<ShmooPoint, ()>>> =
            (0..SRAM_SIZES.len()).map(|_| Vec::new()).collect();

        for &freq in CLOCK_FREQS_HZ {
            scpi(self.lab().clkgen(), &format!(":FREQ {freq:.6E}"));
            println!("{:.1} MHz", freq / 1e6);

            for &vdd in VDD_VOLTS {
                scpi(self.lab().psu(), &format!("VOLT {vdd:.4},(@{PSU_CHANNEL})"));
                thread::sleep(Duration::from_millis(500));

                let vdd_meas_psu: f64 =
                    query(self.lab().psu(), &format!("MEAS:VOLT? (@{PSU_CHANNEL})"))
                        .parse()
                        .expect("unexpected PSU voltage response");
                println!("  VDD set={vdd:.3}V  psu={vdd_meas_psu:.3}V  (SRAM tests skipped)");

                let init_success = self.init_chip().is_ok();

                for (id, size) in SRAM_SIZES.iter().enumerate() {
                    if init_success {
                        let pat = FixedPattern::new(Pattern::march_cm(), *size, 1);
                        let pass =
                            execute(pat, self.tsi_intf().test_sram_executor(id as u64)).is_ok();
                        per_sram[id].push(Ok(ShmooPoint {
                            vdd_set_v: vdd,
                            vdd_meas_psu_v: vdd_meas_psu,
                            clock_freq_hz: freq,
                            pass,
                        }));
                    } else {
                        per_sram[id].push(Err(()));
                    }
                }
            }
        }

        scpi(self.lab().clkgen(), ":OUTP1 OFF");
        scpi(self.lab().psu(), &format!("OUTP OFF,(@{PSU_CHANNEL})"));

        let srams: Vec<SramShmoo> = per_sram
            .into_iter()
            .enumerate()
            .map(|(id, points)| {
                let shmoo = SramShmoo {
                    sram_id: id,
                    points,
                };
                let json = serde_json::to_string_pretty(&shmoo).expect("serialization failed");
                std::fs::write(outdir.join(format!("sram{id}_shmoo.json")), json)
                    .expect("failed to write shmoo json");
                shmoo
            })
            .collect();

        println!("Results written to {}", outdir.display());
        ShmooResult { srams }
    }
}

#[cfg(test)]
mod shmoo {
    #[test]
    fn test_commands() {
        use crate::*;
        let mut l = BringupState::new();
        l.shmoo_test_all_srams("out/shmoo");
    }
}
