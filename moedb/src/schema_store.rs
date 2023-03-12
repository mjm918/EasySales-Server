use serde_json::Value;
use anyhow::{Result};
use regex::Regex;
use crate::common::get_cfs;
use crate::uerr::SchemaError;

pub fn is_schema_ok(schema: &str) -> Result<(), SchemaError> {
    let json_schema = serde_json::from_str(schema);
    if json_schema.is_err() {
        return Err(SchemaError::InvalidSchema(schema.to_string()));
    }
    let fields_ok = schema_check(&json_schema.unwrap());
    if fields_ok.is_err() {
        return Err(fields_ok.err().unwrap());
    }
    Ok(())
}

fn is_schema_properties_ok(value: &Value) -> Result<(),SchemaError> {
    if value.is_object() {
        let cfs = get_cfs();
        for ddt in value.as_object().unwrap() {
            if !ddt.1.is_string() {
                return Err(SchemaError::TypeDeclarationWrong(ddt.0.to_string()));
            }
            let data_type = ddt.1.as_str().unwrap();
            if ddt.0.as_str() == "type" && !is_data_type_ok(data_type) {
                return Err(SchemaError::TypeDeclarationWrong(ddt.0.to_string()));
            }
            if ddt.0.as_str() == "linkingObject" && !cfs.contains(&data_type.to_string()) {
                return Err(SchemaError::UnknownObject(data_type.to_string()));
            }
        }
    } else if value.is_string() && !is_data_type_ok(value.as_str().unwrap()) {
        return Err(SchemaError::TypeDeclarationWrong(value.to_string()));
    }
    if value.is_object() || value.is_string() {
        return Ok(());
    }
    Err(SchemaError::TypeDeclarationWrong(value.to_string()))
}

fn is_schema_pv_ok(name: &str) -> Result<(),SchemaError> {
    let regx = Regex::new(r"[^a-zA-Z0-9_-]").unwrap();
    if name.to_string().trim().is_empty() {
        return Err(SchemaError::InvalidPropNaming { expected: "a-Z".to_string(), found: "".to_string() });
    }
    let starts_with = (b'A'..=b'Z').chain(b'a'..=b'z')
        .map(char::from)
        .collect::<Vec<char>>();
    let first_char = name.char_indices().next().unwrap().1;
    if !starts_with.contains(&first_char) {
        return Err(SchemaError::InvalidPropNaming { expected: "a-Z".to_string(), found: format!("cannot start with {}",first_char.to_string()) });
    }
    if regx.is_match(name) {
        return Err(SchemaError::InvalidPropNaming { expected: "alphanumeric only".to_string(), found: name.to_string() });
    }
    Ok(())
}

fn schema_check(schema: &Value) -> Result<(),SchemaError> {
    let mut prop_count = 0;
    if schema.is_object() {
        let props = schema.as_object().unwrap();
        for key in props{
            if has_field(key.0.as_str()) {
                prop_count += 1;

                // TODO: check if object store exists.

                let pv_check = prop_value_check(key.0,key.1);
                if pv_check.is_err() {
                    return Err(pv_check.err().unwrap());
                }
            }
        }
    } else {
        return Err(SchemaError::MalformedSchema);
    }
    if prop_count != 3 {
        return Err(SchemaError::MalformedSchema);
    }
    Ok(())
}

fn is_data_type_ok(dt: &str) -> bool {
    match dt {
        "string"=>true,
        "number"=>true,
        "date"=>true,
        "list"=>true,
        "object"=>true,
        "image"=>true,
        "blob"=>true,
        &_ => false
    }
}

fn has_field(field: &str) -> bool {
    match field {
        "name"=>true,
        "primaryKey"=>true,
        "properties"=>true,
        &_ => false
    }
}

fn prop_value_check(prop:&String, value:&Value) -> Result<(),SchemaError> {
    match &prop.as_str() {
        &"name"=>{
            is_schema_pv_ok(value.as_str().unwrap())
        },
        &"primaryKey"=>{
            is_schema_pv_ok(value.as_str().unwrap())
        },
        &"properties"=>{
            if !value.is_object() {
                return Err(SchemaError::MalformedPropertyDeclaration);
            }
            let properties = value.as_object().unwrap();
            for dt in properties {
                let col = is_schema_pv_ok(dt.0);
                if col.is_err() {
                    return Err(col.err().unwrap());
                }
                let res = is_schema_properties_ok(dt.1);
                if res.is_err() {
                    return Err(res.err().unwrap());
                }
            }
            Ok(())
        },
        &_ => Err(SchemaError::MalformedPropertyDeclaration)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;
    #[test]
    fn run_schema_ok(){
        let perf = Instant::now();
        let schema = r#"
            {
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
            }
        "#;
        assert_eq!(is_schema_ok(schema).unwrap(),());
        assert_eq!(is_schema_ok(schema).is_err(),false);
        assert_eq!(is_schema_ok("hello").is_err(),true);
        println!("{}",format!("run_schema_ok:: {:?}",perf.elapsed()));
    }
}