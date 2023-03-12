use anyhow::{Result};
use crate::header::{ParsedStatement, StatementType};
use crate::schema_store::is_schema_ok;
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
    if parsed_st.db.trim().is_empty() {
        return Err(StatementError::NoDatabaseSelected);
    }
    let cmd = parsed_st.cmd.clone().unwrap();
    let is_err: Result<(),StatementError> = match cmd {
        StatementType::Create => {
            if parsed_st.pbs_data.is_empty() {
                return Err(StatementError::NoStoreSchemaProvided(parsed_st.store.to_string()));
            }
            let chk = is_schema_ok(parsed_st.pbs_data.as_str());
            if chk.is_err() {
                return Err(StatementError::SchemaError(chk.err().unwrap().to_string()));
            }
            Ok(())
        }
        StatementType::CreateDb => {
            if parsed_st.store.is_empty() {
                return Err(StatementError::UnknownCommand(parsed_st.store.to_string()));
            }
            if parsed_st.pbs_data.is_empty() {
                return Err(StatementError::UnknownCommand(parsed_st.pbs_data.to_string()));
            }
            Ok(())
        }
        StatementType::Get => {
            if parsed_st.store.is_empty() {
                return Err(StatementError::NoStoreSelected);
            }
            if parsed_st.pbs_data.is_empty() {
                return Err(StatementError::NoArgumentWhileGet);
            }
            Ok(())
        }
        StatementType::Upsert => {
            if parsed_st.store.is_empty() {
                return Err(StatementError::NoStoreSelected);
            }
            if parsed_st.pbs_data.is_empty() {
                todo!()
                // validate provided data with schema. check if strict enabled
            }
            Ok(())
        }
        StatementType::Delete => {
            if parsed_st.store.is_empty() {
                return Err(StatementError::NoStoreSelected);
            }
            if parsed_st.pbs_data.is_empty() {
                return Err(StatementError::NoArgumentWhileGet);
            }
            Ok(())
        }
        StatementType::Drop => {
            if parsed_st.store.is_empty() {
                return Err(StatementError::NoStoreSelected);
            }
            if !parsed_st.pbs_data.is_empty() {
                return Err(StatementError::UnknownCommand(parsed_st.pbs_data.to_string()));
            }
            Ok(())
        }
        StatementType::DropDb => {
            if !parsed_st.store.is_empty() {
                return Err(StatementError::UnknownCommand(parsed_st.store.to_string()));
            }
            if !parsed_st.pbs_data.is_empty() {
                return Err(StatementError::UnknownCommand(parsed_st.pbs_data.to_string()));
            }
            Ok(())
        }
        StatementType::DbList => {
            if !parsed_st.store.is_empty() {
                return Err(StatementError::UnknownCommand(parsed_st.store.to_string()));
            }
            if !parsed_st.pbs_data.is_empty() {
                return Err(StatementError::UnknownCommand(parsed_st.pbs_data.to_string()));
            }
            Ok(())
        },
        StatementType::Truncate => {
            if parsed_st.store.is_empty() {
                return Err(StatementError::NoStoreSelected);
            }
            if !parsed_st.pbs_data.is_empty() {
                return Err(StatementError::UnknownCommand(parsed_st.pbs_data.to_string()));
            }
            Ok(())
        }
    };
    Ok(parsed_st)
}
///```
/// SHOW :*
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
/// TRUNCATE :moss:product
/// ```
fn parse_tokens(tokens: Vec<&str>) -> ParsedStatement {
    let mut i = 0;
    let mut command= "";
    let mut dbstore = "";
    let mut psb_data= "";
    let mut rest = Vec::new();
    for tk in tokens {
        if i == 0 {
            command = tk.trim();
        }
        if i == 1 {
            dbstore = tk.trim();
        }
        if i > 1 {
            rest.push(tk.trim());
        }
        i += 1;
    }
    let binding = rest.join(" ");
    psb_data = binding.as_str();

    let mut db = "";
    let mut store = "";
    if !dbstore.is_empty() {
        let mut dbs = dbstore.split(":");
        dbs.next();
        i = 0;
        for spt in dbs {
            if i == 0 {
                db = spt.trim();
            }
            if i == 1 {
                store = spt.trim();
            }
            i += 1;
        }
    }
    if command.eq("CREATE") &&
        !db.is_empty() &&
        psb_data.is_empty() &&
        store.is_empty() {
        command = "CREATE-DB";
    }
    if command.eq("DROP") &&
        !db.is_empty() &&
        store.is_empty() {
        command = "DROP-DB";
    }
    ParsedStatement {
        cmd: match command {
            "SHOW" => Some(StatementType::DbList),
            "GET" => Some(StatementType::Get),
            "UPSERT" => Some(StatementType::Upsert),
            "DELETE" => Some(StatementType::Delete),
            "CREATE" => Some(StatementType::Create),
            "CREATE-DB" => Some(StatementType::CreateDb),
            "DROP" => Some(StatementType::Drop),
            "DROP-DB" => Some(StatementType::DropDb),
            "TRUNCATE" => Some(StatementType::Truncate),
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
        // Test-1
        let t1 = parse_stmt("DROP :moss").unwrap();
        assert_eq!(t1.cmd.unwrap(),StatementType::DropDb);
        // Test-2
        let t2 = parse_stmt("DROP :moss {}");
        assert_eq!(t2.is_err(),true);
        println!("Test-2 :: {}",t2.err().unwrap());
        // Test-3
        let schema = r#"{
                "name":"Person",
                "primaryKey":"id",
                "properties":{
                    "id":"string",
                    "name":"string",
                    "age":"number",
                    "dob":"date",
                    "children":{
                        "type":"list",
                        "linkingObject":"Child"
                    },
                    "employment":{
                        "type":"object",
                        "linkingObject":"Job"
                    },
                    "profilePic":"image",
                    "bio":"blob"
                }
            }"#;
        let create_store = format!("CREATE :moss {}",schema);
        let t3 = parse_stmt(create_store.as_str());
        assert_eq!(t3.is_err(),true);
        println!("Test-3 :: {}",t3.err().unwrap());

        println!("{}",format!("run_statement_ok:: {:?}",perf.elapsed()));
    }
}
