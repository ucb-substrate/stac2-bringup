use std::ffi::CString;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::thread;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use visa_rs::flags::AccessMode;
use visa_rs::{AsResourceManager, DefaultRM, Instrument, TIMEOUT_IMMEDIATE};

use crate::BringupState;
use crate::executor::execute;
use crate::pattern::{FixedPattern, Pattern};
use crate::tests::SRAM_SIZES;

const PSU_VISA_ADDR: &str = "USB0::0x2A8D::0x8F01::CN63420426::INSTR";
const PSU_CHANNEL: u32 = 1;
const SCOPE_VISA_ADDR: &str = "USB0::0x2A8D::0x7B01::MY59280119::INSTR";
const SCOPE_VDD_CHANNEL: u32 = 1;
const CLOCK_GEN_VISA_ADDR: &str = "USB0::0x0957::0x4008::MY428EX302::INSTR";

const VDD_VOLTS: &[f64] = &[
    1.20, 1.25, 1.30, 1.35, 1.40, 1.45, 1.50, 1.55, 1.60,
    1.65, 1.70, 1.75, 1.80, 1.85, 1.90, 1.95, 2.00,
];

const CLOCK_FREQS_HZ: &[f64] = &[
    15e6, 20e6, 25e6, 30e6, 35e6, 40e6, 45e6, 50e6,
    55e6, 60e6, 65e6, 70e6, 75e6, 80e6, 85e6, 90e6, 95e6, 100e6,
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShmooPoint {
    pub vdd_set_v: f64,
    pub vdd_meas_psu_v: f64,
    pub vdd_meas_scope_v: f64,
    pub clock_freq_hz: f64,
    pub pass: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SramShmoo {
    pub sram_id: usize,
    pub points: Vec<ShmooPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShmooResult {
    pub srams: Vec<SramShmoo>,
}

fn scpi(instr: &mut Instrument, cmd: &str) {
    write!(instr, "{cmd}\n").expect("SCPI write failed");
}

fn query(instr: &mut Instrument, cmd: &str) -> String {
    scpi(instr, cmd);
    let mut buf = String::new();
    BufReader::new(instr).read_line(&mut buf).expect("SCPI read failed");
    buf.trim_end_matches(['\r', '\n']).to_string()
}

fn open_instr(rm: &DefaultRM, addr: &str) -> Instrument {
    let expr: visa_rs::ResID = CString::new(addr).unwrap().into();
    let rsc = rm.find_res(&expr).unwrap_or_else(|e| panic!("instrument not found at {addr}: {e}"));
    rm.open(&rsc, AccessMode::NO_LOCK, TIMEOUT_IMMEDIATE)
        .unwrap_or_else(|e| panic!("failed to open {addr}: {e}"))
}

impl BringupState {
    pub fn shmoo_test_all_srams(&mut self, outdir: impl AsRef<Path>) -> ShmooResult {
        let outdir = outdir.as_ref();
        std::fs::create_dir_all(outdir).expect("failed to create output dir");

        let rm = DefaultRM::new().expect("failed to create VISA resource manager");
        let mut psu = open_instr(&rm, PSU_VISA_ADDR);
        let mut scope = open_instr(&rm, SCOPE_VISA_ADDR);
        let mut clock_gen = open_instr(&rm, CLOCK_GEN_VISA_ADDR);

        println!("PSU:       {}", query(&mut psu, "*IDN?"));
        println!("Scope:     {}", query(&mut scope, "*IDN?"));
        println!("Clock gen: {}", query(&mut clock_gen, "*IDN?"));

        scpi(&mut psu, &format!("OUTP ON,(@{PSU_CHANNEL})"));
        scpi(&mut clock_gen, ":OUTP1 ON");

        let mut per_sram: Vec<Vec<ShmooPoint>> =
            (0..SRAM_SIZES.len()).map(|_| Vec::new()).collect();

        for &vdd in VDD_VOLTS {
            scpi(&mut psu, &format!("VOLT {vdd:.4},(@{PSU_CHANNEL})"));
            thread::sleep(Duration::from_millis(500));

            let vdd_meas_psu: f64 = query(&mut psu, &format!("MEAS:VOLT? (@{PSU_CHANNEL})"))
                .parse().expect("unexpected PSU voltage response");
            let vdd_meas_scope: f64 = query(&mut scope, &format!(":MEAS:VAVG? CHAN{SCOPE_VDD_CHANNEL}"))
                .parse().expect("unexpected scope voltage response");
            println!("VDD set={vdd:.3}V  psu={vdd_meas_psu:.3}V  scope={vdd_meas_scope:.3}V");

            for &freq in CLOCK_FREQS_HZ {
                scpi(&mut clock_gen, &format!(":FREQ {freq:.6E}"));

                let mut pass_count = 0usize;
                for (id, &size) in SRAM_SIZES.iter().enumerate() {
                    let pat = FixedPattern::new(Pattern::march_cm(), size, 1);
                    let pass =
                        execute(pat, self.tsi_intf().test_sram_executor(id as u64)).is_ok();
                    if pass { pass_count += 1; }
                    per_sram[id].push(ShmooPoint {
                        vdd_set_v: vdd,
                        vdd_meas_psu_v: vdd_meas_psu,
                        vdd_meas_scope_v: vdd_meas_scope,
                        clock_freq_hz: freq,
                        pass,
                    });
                }

                println!("  {:.1} MHz  {pass_count:2}/{} SRAMs pass", freq / 1e6, SRAM_SIZES.len());
            }
        }

        scpi(&mut clock_gen, ":OUTP1 OFF");
        scpi(&mut psu, &format!("OUTP OFF,(@{PSU_CHANNEL})"));

        let srams: Vec<SramShmoo> = per_sram
            .into_iter()
            .enumerate()
            .map(|(id, points)| {
                let shmoo = SramShmoo { sram_id: id, points };
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
