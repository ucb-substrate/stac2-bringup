use const_format::concatcp;

pub mod bebe;
pub mod executor;
pub mod pattern;
pub mod state;
pub mod testsite;

#[cfg(test)]
mod tests;

const PY_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../py");
const BEBE_HOST: &str = concatcp!(PY_DIR, "/bebe_host.py");
