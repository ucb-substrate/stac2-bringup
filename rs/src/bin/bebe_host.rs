use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;
use serialport::SerialPort;

const NOCK_MAGIC: &[u8] = b"GOBEARS!";
const CMD_READV: u8 = b'R';
const CMD_WRITEV: u8 = b'W';
const CMD_JUMP: u8 = b'J';
const CMD_ACK: u8 = b'Y';

#[derive(Parser)]
#[command(about = "Host-side tool for the BEBE bootloader protocol")]
struct Args {
    /// Disable debugging print statements; will only print read output
    #[arg(long)]
    quiet: bool,

    /// Wait for DUT to wake up
    #[arg(long)]
    wait: bool,

    /// Timeout in seconds for --wait (default: wait forever)
    #[arg(long)]
    wait_timeout: Option<f64>,

    /// COM port to interact with
    #[arg(long)]
    port: String,

    /// Baud rate
    #[arg(long)]
    baudrate: u32,

    /// Address to interact with (hex)
    #[arg(long)]
    addr: Option<String>,

    /// File to write to the DUT
    #[arg(long)]
    wfile: Option<PathBuf>,

    /// Write some given data to the DUT (hex)
    #[arg(long)]
    wdata: Option<String>,

    /// The length of the data to write to the DUT
    #[arg(long)]
    wlen: Option<usize>,

    /// Read length from the DUT
    #[arg(long)]
    rlen: Option<usize>,

    /// Begin executing at the given address (DUT fence.i's)
    #[arg(long)]
    jump: bool,
}

fn hexdump(data: &[u8]) -> String {
    const WIDTH: usize = 16;
    let mut out = String::new();
    for chunk in data.chunks(WIDTH) {
        for b in chunk {
            out.push_str(&format!("{b:02X} "));
        }
        for _ in 0..(WIDTH - chunk.len()) {
            out.push_str("   ");
        }
        out.push_str("  ");
        for &b in chunk {
            out.push(if (32..127).contains(&b) {
                b as char
            } else {
                '.'
            });
        }
        out.push('\n');
    }
    out
}

/// Read exactly `buf.len()` bytes, retrying silently through timeout errors.
fn read_exact(port: &mut dyn SerialPort, buf: &mut [u8]) -> io::Result<()> {
    let mut offset = 0;
    while offset < buf.len() {
        match port.read(&mut buf[offset..]) {
            Ok(0) => return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "port closed")),
            Ok(n) => offset += n,
            Err(e) if e.kind() == io::ErrorKind::TimedOut => {}
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn read_byte(port: &mut dyn SerialPort) -> io::Result<u8> {
    let mut b = [0u8; 1];
    read_exact(port, &mut b)?;
    Ok(b[0])
}

/// Read as many bytes as are currently available (at least 1), retrying through timeouts.
fn read_some<'a>(port: &mut dyn SerialPort, buf: &'a mut [u8; 256]) -> io::Result<&'a [u8]> {
    loop {
        let n = (port.bytes_to_read().unwrap_or(0) as usize)
            .max(1)
            .min(buf.len());
        match port.read(&mut buf[..n]) {
            Ok(0) => return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "port closed")),
            Ok(n) => return Ok(&buf[..n]),
            Err(e) if e.kind() == io::ErrorKind::TimedOut => continue,
            Err(e) => return Err(e),
        }
    }
}

fn tx(port: &mut dyn SerialPort, data: &[u8], quiet: bool) {
    if !quiet {
        print!("{}", hexdump(data));
    }
    port.write_all(data).expect("serial write failed");
    port.flush().expect("serial flush failed");
}

fn log(quiet: bool, msg: &str) {
    if !quiet {
        println!("{msg}");
    }
}

fn parse_hex(s: &str) -> u64 {
    let s = s.trim_start_matches("0x").trim_start_matches("0X");
    u64::from_str_radix(s, 16).expect("invalid hex value")
}

fn main() {
    let args = Args::parse();
    let quiet = args.quiet;

    let mut port = serialport::new(&args.port, args.baudrate)
        .timeout(Duration::from_secs(1))
        .open()
        .expect("failed to open serial port");

    let mut performed_operation = false;

    let mut buf = [0u8; 256];

    if args.wait {
        performed_operation = true;
        log(quiet, "[bebe host] Waiting for DUT...");
        let deadline = args
            .wait_timeout
            .map(|secs| std::time::Instant::now() + Duration::from_secs_f64(secs));
        'wait: loop {
            if deadline.is_some_and(|d| std::time::Instant::now() >= d) {
                eprintln!("[bebe host] timed out waiting for DUT");
                std::process::exit(1);
            }
            for &b in read_some(port.as_mut(), &mut buf).expect("error waiting for DUT") {
                if b == b'A' {
                    break 'wait;
                }
            }
        }
        log(quiet, "[bebe host] DUT found!");
    }

    log(quiet, "[bebe host] Trying to nock...");
    tx(port.as_mut(), NOCK_MAGIC, quiet);
    // Residual wakeup signals from DUT can linger in FIFOs after the nock magic is sent.
    'nock: loop {
        println!("{buf:?}");
        for &b in read_some(port.as_mut(), &mut buf).expect("error during nock") {
            if b == b'A' {
                continue;
            } else if b == CMD_ACK {
                break 'nock;
            } else if !args.wait {
                log(
                    quiet,
                    &format!("[bebe host] Unexpected response from DUT during nock: {b}"),
                );
                std::process::exit(1);
            }
        }
    }
    log(quiet, "[bebe host] Connected to DUT!");

    let Some(addr_str) = args.addr else {
        if !performed_operation {
            Args::parse_from(["bebe_host", "--help"]);
        }
        return;
    };

    let mut addr = parse_hex(&addr_str);

    if let Some(wfile) = args.wfile {
        performed_operation = true;
        let data = fs::read(&wfile).expect("failed to read wfile");
        for chunk in data.chunks(0xfffff) {
            let block_len = chunk.len();
            let mut msg = vec![CMD_WRITEV];
            msg.extend_from_slice(&(block_len as u32).to_be_bytes()); // ">I"
            msg.extend_from_slice(&addr.to_be_bytes()); // ">Q"
            msg.extend_from_slice(chunk);
            log(
                quiet,
                &format!("[bebe host] write {addr:#x}, len {block_len}..."),
            );
            tx(port.as_mut(), &msg, quiet);
            let ack = read_byte(port.as_mut()).expect("failed to read ack");
            if ack != CMD_ACK {
                log(
                    quiet,
                    &format!("[bebe host] Unexpected response from DUT during nock: {ack}"),
                );
                std::process::exit(1);
            }
            addr += block_len as u64;
        }
        log(quiet, "[bebe host] OK");
    }

    if let (Some(wdata_str), Some(wlen)) = (args.wdata, args.wlen) {
        performed_operation = true;
        if wlen > 8 {
            log(
                quiet,
                &format!("[bebe host] wlen {wlen} > 8, which is not allowed. Use wfile."),
            );
        } else {
            let wdata = parse_hex(&wdata_str);
            // struct.pack(">Q", wdata)[-wlen:] — big-endian u64, last wlen bytes
            let wdata_be = wdata.to_be_bytes();
            let wdata_clip = &wdata_be[8 - wlen..];
            let mut msg = vec![CMD_WRITEV];
            msg.extend_from_slice(&(wlen as u32).to_be_bytes());
            msg.extend_from_slice(&addr.to_be_bytes());
            msg.extend_from_slice(wdata_clip);
            log(
                quiet,
                &format!("[bebe host] write {addr:#x}={wdata:#x}, len {wlen}..."),
            );
            tx(port.as_mut(), &msg, quiet);
            let ack = read_byte(port.as_mut()).expect("failed to read ack");
            if ack != CMD_ACK {
                log(quiet, &format!("[bebe host] Expected ack, got {ack}"));
            } else {
                log(quiet, "[bebe host] OK");
            }
        }
    }

    if let Some(rlen) = args.rlen {
        performed_operation = true;
        let mut msg = vec![CMD_READV];
        msg.extend_from_slice(&(rlen as u32).to_be_bytes());
        msg.extend_from_slice(&addr.to_be_bytes());
        log(quiet, &format!("[bebe host] read {addr:#x}, len {rlen}..."));
        tx(port.as_mut(), &msg, quiet);
        log(quiet, "[bebe host] read result:");
        let mut data = vec![0u8; rlen];
        read_exact(port.as_mut(), &mut data).expect("failed to read data");
        if quiet {
            // int.from_bytes(data, "little") — little-endian arbitrary-precision int
            let mut val: u128 = 0;
            for (i, &b) in data.iter().enumerate() {
                val |= (b as u128) << (8 * i);
            }
            println!("{val}");
        } else {
            print!("{}", hexdump(&data));
        }
    }

    if args.jump {
        performed_operation = true;
        let mut msg = vec![CMD_JUMP];
        msg.extend_from_slice(&addr.to_be_bytes());
        log(quiet, &format!("[bebe host] Jump {addr:#x}..."));
        tx(port.as_mut(), &msg, quiet);
        let ack = read_byte(port.as_mut()).expect("failed to read ack");
        if ack != CMD_ACK {
            log(quiet, &format!("[bebe host] Expected ack, got {ack}"));
        } else {
            log(quiet, "[bebe host] OK");
        }
    }

    if !performed_operation {
        Args::parse_from(["bebe_host", "--help"]);
    }
}
