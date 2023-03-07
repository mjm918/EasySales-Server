use crate::header::{CfData, CfWithInfo};
use serde_json::{from_str, Value};

fn is_json(input_string: &str, key: Vec<String>) -> bool {
    match from_str::<Value>(input_string) {
        Ok(data) => {
            for i in key.iter() {
                let is_ok = data.get(i).is_some();
                if !is_ok {
                    return false;
                }
            }
            true
        },
        Err(_) => false,
    }
}

fn is_arr_json(input_string: &str, key: Vec<String>) -> Option<Value> {
    match from_str::<Vec<Value>>(input_string) {
        Ok(data) => {
            let mut wrong: Value = Default::default();
            for v in data.iter() {
                for i in key.iter() {
                    let is_ok = v.get(i).is_some();
                    if !is_ok {
                        wrong = v.clone();
                    }
                }
            }
            Some(wrong)
        },
        Err(_) => None
    }
}