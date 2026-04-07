use std::process::Command;

use crate::{config::CONFIG, PY_DIR};

pub fn tsi_write(addr: u64, data: u64) {
    let status = Command::new("uv")
        .args([
            "run",
            "python3",
            "-m",
            "pyuartsi",
            "--port",
            &CONFIG.fpga_com_port,
            "--baudrate",
            "115200",
            "--init_write",
            &format!("0x{addr:X}=0x{data:X}"),
        ])
        .current_dir(PY_DIR)
        .status()
        .expect("failed to run pyuartsi");
    if !status.success() {
        panic!("pyuartsi exited with non-zero exit code")
    }
}

pub fn tsi_read(addr: u64) -> u64 {
    let output = Command::new("uv")
        .args([
            "run",
            "python3",
            "-m",
            "pyuartsi",
            "--port",
            &CONFIG.fpga_com_port,
            "--baudrate",
            "115200",
            "--init_read",
            &format!("0x{addr:X}"),
        ])
        .current_dir(PY_DIR)
        .output()
        .expect("failed to run pyuartsi");
    let output = String::from_utf8(output.stdout).expect("failed to parse pyuartsi output");
    output
        .trim()
        .parse()
        .expect("failed to convert pyuartsi output to u64")
}
