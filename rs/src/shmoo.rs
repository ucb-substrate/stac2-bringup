use std::path::Path;
use std::thread;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{
    BebeIntf, BistController, BringupState, config::ClkSel, march_b_bist, march_cm_bist, rand_bist,
};

fn stepped_range(start: f64, end: f64, step: f64) -> impl Iterator<Item = f64> {
    let n = ((end - start) / step).round() as usize + 1;
    (0..n).map(move |i| start + i as f64 * step)
}

pub fn vdd_volts() -> impl Iterator<Item = f64> {
    stepped_range(1.0, 2.0, 0.05)
}

pub fn clock_freqs_hz() -> impl Iterator<Item = f64> {
    stepped_range(15e6, 115e6, 5e6)
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ShmooResult {
    IntfFail,
    BistFail,
    SramFail,
    Pass,
}

pub struct ShmooTest<I> {
    pub tag: String,
    pub constructor: Box<dyn Fn(I, u64) -> BistController<I>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShmooPoint {
    pub test: String,
    pub vdd_set_v: f64,
    pub vdd_meas_psu_v: f64,
    pub idd_meas_psu_a: f64,
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

        for freq in clock_freqs_hz() {
            self.lab().clkgen_freq(freq);
            thread::sleep(Duration::from_millis(3000));
            self.lab().clkgen_on();
            println!("{:.1} MHz", freq / 1e6);

            for vdd in vdd_volts() {
                self.lab().psu_vdd(vdd);
                thread::sleep(Duration::from_millis(1000));

                let vdd_meas_psu: f64 = self.lab().psu_vdd_meas();
                let idd_meas_psu: f64 = self.lab().psu_idd_meas();
                println!("  VDD set={vdd:.3}V  psu={vdd_meas_psu:.3}V");
                println!("  IDD psu={idd_meas_psu:.3}V");

                let init_success = match self.bebe_init() {
                    Ok(_) => true,
                    Err(e) => {
                        eprintln!("failed to initialize: {e}");
                        false
                    }
                };
                for test in [
                    ShmooTest::<BebeIntf> {
                        tag: "rand".to_string(),
                        constructor: Box::new(rand_bist),
                    },
                    ShmooTest {
                        tag: "march_b".to_string(),
                        constructor: Box::new(march_b_bist),
                    },
                    ShmooTest {
                        tag: "march_cm".to_string(),
                        constructor: Box::new(march_cm_bist),
                    },
                ] {
                    let mut bist_initialized = false;

                    for shmoo in sram_shmoos.iter_mut() {
                        let id = shmoo.sram_id;
                        let result = if init_success {
                            let intf = self.bebe_intf();
                            let mut bist = march_cm_bist(intf, id as u64);
                            // If pattern/element registers are already set correctly,
                            // don't waste time setting them again. Just set SRAM-specific
                            // registers.
                            let bist_init_success = if bist_initialized {
                                bist.skip_init = true;
                                bist.init_sram().is_ok()
                            } else {
                                bist_initialized = true;
                                true
                            };
                            if bist_init_success {
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
                            }
                        } else {
                            ShmooResult::IntfFail
                        };
                        let pt = ShmooPoint {
                            test: test.tag.clone(),
                            vdd_set_v: vdd,
                            vdd_meas_psu_v: vdd_meas_psu,
                            idd_meas_psu_a: idd_meas_psu,
                            clock_freq_hz: freq,
                            result,
                        };
                        println!("SRAM {id}: {pt:?}");
                        shmoo.points.push(pt);
                        let json =
                            serde_json::to_string_pretty(&shmoo).expect("serialization failed");
                        std::fs::write(outdir.join(format!("sram{id}_shmoo.json")), json)
                            .expect("failed to write shmoo json");
                    }
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
