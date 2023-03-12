use anyhow::{Result,Error};
use crate::header::{Command, Disk, Memory, MoDb, SocketResponse};
use crate::statement::parse_stmt;
use crate::uerr::StatementError;

impl Command {
    pub fn new() -> Result<Self,Error> {
        let mem = Memory::new();
        if mem.is_err() {
            return Err(mem.err().unwrap());
        }
        let disk = Disk::new();
        if disk.is_err() {
            return Err(disk.err().unwrap());
        }
        Ok(Self {
            mem: mem.unwrap(),
            disk: disk.unwrap()
        })
    }

    pub fn receive(&self, stmt: String) -> SocketResponse {
        let parsed_stmt = parse_stmt(stmt.as_str());
        if parsed_stmt.is_err() {
            return SocketResponse {
                err: true,
                message: parsed_stmt.err().unwrap(),
                data: Default::default(),
            };
        }
        SocketResponse {
            err: false,
            message: StatementError::UnknownCommand("TODO".to_string()),
            data: Default::default(),
        }
    }
}