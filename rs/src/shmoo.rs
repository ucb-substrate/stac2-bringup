use std::path::Path;
use std::thread;
use std::time::Duration;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    BebeIntf, BistController, BringupState, CLK_EN, HALF_CLK_DIV_RATIO, MemoryIntf, config::ClkSel,
    march_b_bist, march_cm_bist, rand_bist,
};

fn stepped_range(start: f64, end: f64, step: f64) -> impl Iterator<Item = f64> {
    let n = ((end - start) / step).round() as usize + 1;
    (0..n).map(move |i| start + i as f64 * step)
}

pub fn vdd_volts() -> impl Iterator<Item = f64> {
    stepped_range(1.8, 1.8, 0.05)
}

pub fn clock_freqs_hz() -> impl Iterator<Item = f64> {
    stepped_range(15e6, 30e6, 5e6)
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ShmooResult {
    IntfTimeout,
    BistTimeout,
    BistFail,
    Pass,
}

pub trait FrequencySweep {
    fn init(&mut self, state: &mut BringupState);
    // Sets the frequency and returns the set frequency Hz.
    fn next(&mut self, state: &mut BringupState) -> Option<f64>;
    fn shutdown(&mut self, state: &mut BringupState);
}

pub struct ClkgenSweep {
    freqs_hz: Box<dyn Iterator<Item = f64>>,
}

impl ClkgenSweep {
    fn new(freqs_hz: impl IntoIterator<Item = f64> + 'static) -> Self {
        Self {
            freqs_hz: Box::new(freqs_hz.into_iter()),
        }
    }
}

impl FrequencySweep for ClkgenSweep {
    fn init(&mut self, state: &mut BringupState) {
        assert!(
            matches!(state.config.clk_sel, ClkSel::External),
            "external clock must be selected to run clkgen sweep"
        );

        assert_eq!(
            state.tsi_intf().read(CLK_EN).unwrap(),
            0,
            "FPGA clock must be disabled"
        );

        println!("Clock gen: {}", state.lab().clkgen_idn());

        state.lab().clkgen_vdd(2.0);
    }

    fn next(&mut self, state: &mut BringupState) -> Option<f64> {
        let freq = self.freqs_hz.next()?;
        state.lab().clkgen_freq(freq);
        thread::sleep(Duration::from_millis(3000));
        state.lab().clkgen_on();
        println!("{:.1} MHz", freq / 1e6);
        Some(freq)
    }

    fn shutdown(&mut self, state: &mut BringupState) {
        state.lab().clkgen_off();
    }
}

pub struct FpgaSweep {
    half_clk_div_ratios: Box<dyn Iterator<Item = u64>>,
}

impl FpgaSweep {
    fn new(half_clk_div_ratios: impl IntoIterator<Item = u64> + 'static) -> Self {
        Self {
            half_clk_div_ratios: Box::new(half_clk_div_ratios.into_iter()),
        }
    }
}

impl FrequencySweep for FpgaSweep {
    fn init(&mut self, state: &mut BringupState) {
        assert!(
            matches!(state.config.clk_sel, ClkSel::Fpga),
            "FPGA clock must be selected to run FPGA sweep"
        );

        if let Ok(clkgen) = state.lab().try_clkgen() {
            assert_eq!(
                clkgen
                    .query(":OUTP1:POS?")
                    .parse::<u64>()
                    .expect("unexpected clkgen state"),
                0,
                "External clock must be disabled"
            );
        } else {
            eprintln!(
                "WARNING: Could not check if clkgen is off, ensure that it is off before continuing. Waiting 5 seconds..."
            );
            thread::sleep(Duration::from_secs(5));
        }

        state.enable_clk();
    }

    fn next(&mut self, state: &mut BringupState) -> Option<f64> {
        let div_ratio = self.half_clk_div_ratios.next()?;
        state
            .tsi_intf()
            .write(HALF_CLK_DIV_RATIO, div_ratio)
            .unwrap();
        let freq = 50e6 / div_ratio as f64 / 2.;
        println!("{:.1} MHz", freq / 1e6);
        Some(freq)
    }

    fn shutdown(&mut self, state: &mut BringupState) {
        state.disable_clk();
    }
}

pub struct ShmooTest {
    pub tag: String,
    pub constructor: Box<dyn for<'a> Fn(BebeIntf<'a>, u64) -> BistController<BebeIntf<'a>>>,
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
    pub fn shmoo_vmin(
        &mut self,
        srams: impl IntoIterator<Item = usize>,
        out_dir: impl AsRef<Path>,
    ) -> Vec<SramShmoo> {
        self.shmoo(
            FpgaSweep::new(8..12),
            stepped_range(0.8, 1.1, 0.05),
            vec![ShmooTest {
                tag: "rand".to_string(),
                constructor: Box::new(|intf, id| rand_bist(intf, id)),
            }],
            srams,
            out_dir,
        )
    }

    pub fn shmoo_all_tests(
        &mut self,
        srams: impl IntoIterator<Item = usize>,
        out_dir: impl AsRef<Path>,
    ) -> Vec<SramShmoo> {
        self.shmoo(
            ClkgenSweep::new(stepped_range(15e6, 115e6, 5e6)),
            stepped_range(1., 2., 0.05),
            vec![
                ShmooTest {
                    tag: "rand".to_string(),
                    constructor: Box::new(|intf, id| rand_bist(intf, id)),
                },
                ShmooTest {
                    tag: "march_b".to_string(),
                    constructor: Box::new(|intf, id| march_b_bist(intf, id)),
                },
                ShmooTest {
                    tag: "march_cm".to_string(),
                    constructor: Box::new(|intf, id| march_cm_bist(intf, id)),
                },
            ],
            srams,
            out_dir,
        )
    }
    pub fn shmoo<S: FrequencySweep>(
        &mut self,
        mut freq_sweep: S,
        vdds: impl IntoIterator<Item = f64>,
        tests: impl IntoIterator<Item = ShmooTest>,
        srams: impl IntoIterator<Item = usize>,
        outdir: impl AsRef<Path>,
    ) -> Vec<SramShmoo> {
        let outdir = outdir.as_ref();
        std::fs::create_dir_all(outdir).expect("failed to create output dir");

        println!("PSU:       {}", self.lab().psu_idn());
        self.lab().psu_on();
        freq_sweep.init(self);

        let mut sram_shmoos: Vec<SramShmoo> = srams
            .into_iter()
            .map(|sram_id| SramShmoo {
                sram_id,
                points: Vec::new(),
            })
            .collect();

        let vdds = vdds.into_iter().collect_vec();
        let tests = tests.into_iter().collect_vec();
        while let Some(freq) = freq_sweep.next(self) {
            for vdd in vdds.iter().copied() {
                self.lab().psu_vdd(vdd);
                thread::sleep(Duration::from_millis(1000));

                let vdd_meas_psu: f64 = self.lab().psu_vdd_meas();
                let idd_meas_psu: f64 = self.lab().psu_idd_meas();
                println!("  VDD set={vdd:.3}V  psu={vdd_meas_psu:.3}V");
                println!("  IDD psu={idd_meas_psu:.3}V");

                let mut init_success = match self.bebe_init() {
                    Ok(_) => true,
                    Err(e) => {
                        eprintln!("failed to initialize: {e}");
                        false
                    }
                };
                for test in &tests {
                    let mut bist_initialized = false;

                    for shmoo in sram_shmoos.iter_mut() {
                        let id = shmoo.sram_id;
                        let result = if init_success {
                            let timeout = self.config.timeout;
                            let intf = self.bebe_intf();
                            let mut bist = (test.constructor)(intf, id as u64);
                            bist.timeout = Some(timeout);
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
                                        Err(_) => ShmooResult::BistFail,
                                    },
                                    Err(e) if e.downcast_ref::<std::io::Error>().is_some() => {
                                        ShmooResult::IntfTimeout
                                    }
                                    Err(_) => ShmooResult::BistTimeout,
                                }
                            } else {
                                ShmooResult::IntfTimeout
                            }
                        } else {
                            ShmooResult::IntfTimeout
                        };
                        if let ShmooResult::IntfTimeout = result {
                            init_success = false;
                        }
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
        freq_sweep.shutdown(self);
        self.lab().psu_off();

        println!("Results written to {}", outdir.display());
        sram_shmoos
    }
}
