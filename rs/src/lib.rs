use const_format::concatcp;

pub mod bebe;
pub mod config;
pub mod executor;
pub mod pattern;
pub mod state;
pub mod testsite;
pub mod tsi;

#[cfg(test)]
mod tests;

pub use bebe::{bebe_read, bebe_write};
pub use tsi::{tsi_read, tsi_write};

const PY_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../py");
const BEBE_HOST: &str = concatcp!(PY_DIR, "/bebe_host.py");
