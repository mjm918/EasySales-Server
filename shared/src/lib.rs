pub mod validator;
pub mod toml_schema;
pub mod util;
pub mod tls;

pub const DB_SOCKET_PATH:&str = "./moedb.sock";
pub const DB_PATH:&str = "./moedb";
pub const SYS_CFG:&str = "./server-config.toml";
pub const LOG_DIR_APP:&str = "./logs";
pub const LOG_FILE_APP:&str = "./logs/app.log";
pub const EASYSALES_ART:&str = r"
  _____                ____        _
 | ____|__ _ ___ _   _/ ___|  __ _| | ___  ___
 |  _| / _` / __| | | \___ \ / _` | |/ _ \/ __|
 | |__| (_| \__ \ |_| |___) | (_| | |  __/\__ \
 |_____\__,_|___/\__, |____/ \__,_|_|\___||___/
                 |___/
";
pub const CLI_INSTRUCTION:&str = r"
    💻Author        Mohammad Julfikar Mahmud
    ⚙️Version       0.01
    📧Email         julfikar@eztech.com.my


    --cfg           Config file path.
                    Config file must be `*.toml`

    --status-tcp    Status of TCP server
    --status-http   Status of HTTP server

    Type `cli --<OPTION>`
";


pub const MOEDB_INF:&str = "information";
pub const MOEDB_STORE_INF:&str = "store:information";