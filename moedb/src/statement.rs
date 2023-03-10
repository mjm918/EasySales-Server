use std::iter::Map;
use anyhow::{Result};
use crate::header::{ParsedStatement, StatementType};
use crate::uerr::{StatementError};

pub fn parse_stmt(st: &str) -> Result<ParsedStatement, StatementError> {
    if st.is_empty() {
        return Err(StatementError::InvalidStatement(st.to_string()));
    }
    let tokens: Vec<&str> = st.split(" ").collect();
    if tokens.len() == 0 {
        return Err(StatementError::InvalidStatement(st.to_string()));
    }
    let parsed_st = parse_tokens(tokens.clone());
    if parsed_st.cmd.is_none() {
        return Err(StatementError::UnknownCommand(tokens[0].to_string()));
    }
    Ok(parsed_st)
}
///```
/// GET :moss:product {}
/// GET :moss:product {"$limit":100,"$offset":0}
/// GET :moss:product {"product_code":"A001"}
/// GET :moss:product {"product_available_qty":{"$bt":[10,100]}}
/// UPSERT :moss {....}
/// DELETE :moss:product {"product_code":"A001"}
/// CREATE :moss
/// CREATE :moss {
///                 "name":"person",
///                 "primaryKey":"id",
///                 "properties":{
///                     "id":"string",
///                     "name":"string",
///                     "age":"number",
///                     "dob":"date",
///                     "profile-pic":"image",
///                     "bio":"blob"
///                 }
///             }
///
/// DROP :moss
/// DROP :moss:product
///
/// ```
fn parse_tokens(tokens: Vec<&str>) -> ParsedStatement {
    let mut i = 0;
    let mut command= "";
    let mut dbstore = "";
    let mut psb_data= "";
    for tk in tokens {
        if i == 0 {
            command = tk.trim();
        }
        if i == 1 {
            dbstore = tk.trim();
        }
        if i == 2 {
            psb_data = tk.trim();
        }
        i += 1;
    }
    let mut db = "";
    let mut store = "";
    if !dbstore.is_empty() {
        let mut dbs = dbstore.split(":");
        dbs.next();
        i = 0;
        for spt in dbs {
            if i == 0 {
                db = spt;
            }
            if i == 1 {
                store = spt;
            }
            i += 1;
        }
    }
    ParsedStatement {
        cmd: match command {
            "GET" => Some(StatementType::Get),
            "UPSERT" => Some(StatementType::Upsert),
            "DELETE" => Some(StatementType::Delete),
            "CREATE" => Some(StatementType::Create),
            "DROP" => Some(StatementType::Drop),
            &_ => None
        },
        db: db.to_string(),
        store: store.to_string(),
        pbs_data: psb_data.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;
    #[test]
    fn run_statement_ok() {
        let perf = Instant::now();
        let parsed_stmt = parse_stmt("DROP :moss:product").unwrap();
        assert_eq!(parsed_stmt.cmd.unwrap(),StatementType::Drop);
        assert_eq!(parsed_stmt.db,"moss");
        assert_eq!(parsed_stmt.store,"product");
        assert_eq!(parsed_stmt.pbs_data, "");
        println!("{}",format!("run_statement_ok:: {:?}",perf.elapsed()));
    }
}
