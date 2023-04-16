use serde_json::{Result, Value};

pub fn read_json_file(json_str: &str) -> Result<Value> {

    use speculoos::prelude::*;

    let result: Result<Value> = serde_json::from_str(json_str);
    // assert_that(&result).is_ok();
    //let json_value = result.unwrap();
    result
}
