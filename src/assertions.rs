
use serde_json::{Result, Value};


#[cfg(test)]
pub fn read_json_file(json_str: &str) -> Value {

    use speculoos::prelude::*;

    let result: Result<Value> = serde_json::from_str(json_str);
    assert_that(&result).is_ok();
    let json_value = result.unwrap();
    json_value
}
