use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use anyhow::{Result, anyhow, Error};
use shared::{EASYSALES_ART,CLI_INSTRUCTION,validator,util};
use simplelog::*;
use log::{error, LevelFilter};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("{}",EASYSALES_ART);
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        println!("{}",CLI_INSTRUCTION);
        return Ok(());
    }
    if args.len() == 1 {
        match args[0].as_str().trim() {
            "--start"      =>{

            }
            "--stop"       =>{

            }
            "--restart"    =>{

            }
            "--status-tcp" =>{

            }
            "--status-http" =>{

            }
            "--connect-moedb"=>{
                let socket_path = Path::new("/Users/julfikar/Documents/Personal.nosync/EasySales-Server/target/debug/moedb.sock");
                let mut stream = UnixStream::connect(socket_path).await.unwrap();

                let message = "Hello, Unix domain socket!".as_bytes();
                stream.write_all(message).await.unwrap();

                let mut response = [0; 1024];
                let n = stream.read(&mut response).await.unwrap();
                println!("Received {} bytes: {:?}", n, String::from_utf8_lossy(&response[..n]));
            }
            _ => {

            }
        }
    }
    if args.len() == 2 && args[0].as_str().trim() == "--cfg" && args[1].as_str().trim() != "" {
        let config_path = args[1].as_str().trim();
        let config = validator::is_cfg_ok(&config_path);
        if config.is_ok(){
            let cp_cfg = util::cp_cfg_toml(&config_path);
            if cp_cfg.is_ok() {
                println!(r"
                    ✅   Config file has been moved to source directory
                    ⚠️  Consider restarting server to make changes
                ");
                return Ok(());
            }
        } else {
            return Err(config.err().unwrap());
        }
    }
    return Err(anyhow!("Unknown instructions"));
}