use anyhow::{Result,anyhow,Error};
use std::{fs, io};
use toml;
use crate::toml_schema::BaseConfig;

pub fn is_cfg_ok(path: &str) -> Result<BaseConfig,Error> {
    if let Ok(file) = fs::metadata(path) {
        if file.is_file() {
            let content = match fs::read_to_string(path) {
                Ok(c)=>c,
                Err(err)=>err.to_string()
            };
            let config = toml::from_str(&content);
            if config.is_ok() {
                Ok(config.unwrap())
            } else {
                Err(anyhow!("Config parse failed"))
            }
        } else {
            Err(anyhow!(format!("`{}` is not a file",path)))
        }
    } else {
        Err(anyhow!(format!("`{}` does not exists",path)))
    }
}

pub fn try_read(r: io::Result<usize>) -> io::Result<Option<usize>> {
    match r {
        Ok(len) => Ok(Some(len)),
        Err(e) => {
            if e.kind() == io::ErrorKind::WouldBlock {
                Ok(None)
            } else {
                Err(e)
            }
        }
    }
}
