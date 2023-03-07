use anyhow::{Error, Result, anyhow};
use std::{fs,thread};
use crate::SYS_CFG;

pub fn cp_cfg_toml(path: &str) -> Result<bool,Error>{
    let op = fs::copy(path, SYS_CFG);
    if op.is_err() {
        return Err(Error::from(op.err().unwrap()))
    }
    return Ok(true);
}

pub fn use_available_threads() -> usize {
    (thread::available_parallelism().map_or(1, usize::from) * 4).next_power_of_two()
}