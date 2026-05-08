use std::path::Path;
use std::thread;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::executor::execute;
use crate::pattern::{FixedPattern, Pattern};
use crate::tests::SRAM_SIZES;
use crate::{BringupState, march_cm_bist};

pub const VDD_VOLTS: &[f64] = &[
    1.20, 1.25, 1.30, 1.35, 1.40, 1.45, 1.50, 1.55, 1.60, 1.65, 1.70, 1.75, 1.80, 1.85, 1.90,
];

pub const CLOCK_FREQS_HZ: &[f64] = &[
    // 15e6, 20e6, 25e6, 30e6, 35e6,
    40e6, 45e6, 50e6, 55e6, 60e6, 65e6, 70e6, 75e6, 80e6, 85e6, 90e6, 95e6, 100e6,
];

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ShmooResult {
    TsiFail,
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
    pub fn shmoo_test_all_srams(&mut self, outdir: impl AsRef<Path>) -> Vec<SramShmoo> {
        let outdir = outdir.as_ref();
        std::fs::create_dir_all(outdir).expect("failed to create output dir");

        println!("PSU:       {}", self.lab().psu_idn());
        println!("Clock gen: {}", self.lab().clkgen_idn());

        self.lab().psu_on();
        self.lab().clkgen().cmd(":VOLT1:HIGH 2.0");
        self.lab().clkgen().cmd(":VOLT1:LOW 0.0");

        let mut per_sram: Vec<Vec<ShmooPoint>> =
            (0..SRAM_SIZES.len()).map(|_| Vec::new()).collect();

        for &freq in CLOCK_FREQS_HZ {
            self.lab().clkgen().cmd(&format!(":FREQ {freq:.6E}"));
            thread::sleep(Duration::from_millis(3000));
            self.lab().clkgen().cmd(":OUTP1:POS ON");
            println!("{:.1} MHz", freq / 1e6);

            for &vdd in VDD_VOLTS {
                self.lab().psu_vdd(vdd);
                thread::sleep(Duration::from_millis(1000));

                let vdd_meas_psu: f64 = self.lab().psu_vdd_meas();
                println!("  VDD set={vdd:.3}V  psu={vdd_meas_psu:.3}V");

                let init_success = self.init_chip().is_ok();

                println!("finish init chip");

                for (id, size) in SRAM_SIZES.iter().take(1).enumerate() {
                    let result = if init_success {
                        let intf = self.tsi_intf();
                        let mut bist = march_cm_bist(intf, id as u64);
                        println!("executing bist");
                        match bist.execute() {
                            Ok(res) => match bist.validate_res(res) {
                                Ok(_) => ShmooResult::Pass,
                                Err(_) => ShmooResult::SramFail,
                            },
                            Err(e) if e.downcast_ref::<std::io::Error>().is_some() => {
                                ShmooResult::TsiFail
                            }
                            Err(_) => ShmooResult::BistFail,
                        }
                    } else {
                        ShmooResult::TsiFail
                    };
                    let pt = ShmooPoint {
                        vdd_set_v: vdd,
                        vdd_meas_psu_v: vdd_meas_psu,
                        clock_freq_hz: freq,
                        result,
                    };
                    println!("finish executing bist");
                    println!("SRAM {id}: {pt:?}");
                    per_sram[id].push(pt);
                }
            }
        }

        self.lab().clkgen().cmd(":OUTP1 OFF");
        self.lab().psu_off();

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
        srams
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
