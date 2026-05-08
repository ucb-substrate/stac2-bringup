use std::path::Path;
use std::thread;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{BringupState, config::ClkSel, march_cm_bist};

pub const VDD_VOLTS: &[f64] = &[
    // 1.20, 1.25, 1.30, 1.35, 1.40, 1.45, 1.50, 1.55,
    1.60, 1.65, 1.70, 1.75, 1.80, 1.85, 1.90,
];

pub const CLOCK_FREQS_HZ: &[f64] = &[
    // 15e6, 20e6, 25e6, 30e6, 35e6,
    40e6, 45e6, 50e6, 55e6, 60e6, 65e6, 70e6, 75e6, 80e6, 85e6, 90e6, 95e6, 100e6,
];

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ShmooResult {
    IntfFail,
    BistFail,
    SramFail,
    Pass,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShmooPoint {
    pub vdd_set_v: f64,
    pub vdd_meas_psu_v: f64,
    pub clock_freq_hz: f64,
    pub result: ShmooResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SramShmoo {
    pub sram_id: usize,
    pub points: Vec<ShmooPoint>,
}

impl BringupState {
    pub fn shmoo(
        &mut self,
        srams: impl IntoIterator<Item = usize>,
        outdir: impl AsRef<Path>,
    ) -> Vec<SramShmoo> {
        if !matches!(self.config.clk_sel, ClkSel::External) {
            panic!("external clock must be selected to run Shmoo tests");
        }
        let outdir = outdir.as_ref();
        std::fs::create_dir_all(outdir).expect("failed to create output dir");

        println!("PSU:       {}", self.lab().psu_idn());
        println!("Clock gen: {}", self.lab().clkgen_idn());

        self.lab().psu_on();
        self.lab().clkgen_vdd(2.0);

        let mut sram_shmoos: Vec<SramShmoo> = srams
            .into_iter()
            .map(|sram_id| SramShmoo {
                sram_id,
                points: Vec::new(),
            })
            .collect();

        for &freq in CLOCK_FREQS_HZ {
            self.lab().clkgen_freq(freq);
            thread::sleep(Duration::from_millis(3000));
            self.lab().clkgen_on();
            println!("{:.1} MHz", freq / 1e6);

            for &vdd in VDD_VOLTS {
                self.lab().psu_vdd(vdd);
                thread::sleep(Duration::from_millis(1000));

                let vdd_meas_psu: f64 = self.lab().psu_vdd_meas();
                println!("  VDD set={vdd:.3}V  psu={vdd_meas_psu:.3}V");

                let init_success = self.bebe_init().is_ok();

                for shmoo in sram_shmoos.iter_mut() {
                    let id = shmoo.sram_id;
                    let result = if init_success {
                        let intf = self.bebe_intf();
                        let mut bist = march_cm_bist(intf, id as u64);
                        match bist.execute() {
                            Ok(res) => match bist.validate_res(res) {
                                Ok(_) => ShmooResult::Pass,
                                Err(_) => ShmooResult::SramFail,
                            },
                            Err(e) if e.downcast_ref::<std::io::Error>().is_some() => {
                                ShmooResult::IntfFail
                            }
                            Err(_) => ShmooResult::BistFail,
                        }
                    } else {
                        ShmooResult::IntfFail
                    };
                    let pt = ShmooPoint {
                        vdd_set_v: vdd,
                        vdd_meas_psu_v: vdd_meas_psu,
                        clock_freq_hz: freq,
                        result,
                    };
                    println!("SRAM {id}: {pt:?}");
                    shmoo.points.push(pt);
                    let json = serde_json::to_string_pretty(&shmoo).expect("serialization failed");
                    std::fs::write(outdir.join(format!("sram{id}_shmoo.json")), json)
                        .expect("failed to write shmoo json");
                }
            }
        }

        self.lab().clkgen_off();
        self.lab().psu_off();

        println!("Results written to {}", outdir.display());
        sram_shmoos
    }
}

#[cfg(test)]
mod shmoo {
    #[test]
    fn test_commands() {
        use crate::*;
        let mut l = BringupState::new();
        l.shmoo(0..22, "out/shmoo");
    }
}
