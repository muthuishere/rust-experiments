use std::collections::HashMap;
use std::fmt::Error;
use std::hash::Hash;
use std::result;

use serde_json;
use crate::common::linq_extensions::*;
use crate::common::functions;
use anyhow::{Context, Result};

use crate::car_service;
use crate::car_service::Car;

use std::iter::Iterator;
use crate::product_service::*;


// When a single value is returned from Result , use  .map , .map_err , .and_then , .or_else
// When multiple values are returned from Result , use ? followed by into_iter() , and then use .map

// use .map_err to convert a Result<T, E> into a Result<T, F> , to have a different error type
pub fn get_product_by_id(id:i64) -> Result<Product> {
//ok_or_else => to convert an Option<T> into a Result<T, E>


    get_all_products()?
        .into_iter()
        .find(|product| product.product_id == id)
        .ok_or_else(|| anyhow::anyhow!("Product not found"))
}

pub fn get_product_by_id_string(s:&str) -> Result<Product> {

// Adding a question mark will propogate the error , and whatever followed will not be executed , if error
    // if there is no error then the next line will be executed
   //  let id = s.parse::<i64>().with_context(|| "not parseable")?;
   // get_product_by_id(id)


    //and_then => to convert a Result<Result<T, E>,K> into a Result<U, E>
   s.parse::<i64>()
       .with_context(|| "not parseable")
       .map(|id| get_product_by_id(id))
       .and_then(functions::identity)

}



#[cfg(test)]
mod tests {
    use speculoos::*;
    use speculoos::prelude::*;

    use super::*;

    #[test]
    fn get_product_by_id_with_invalid_id_should_throw_error() {
        let result = get_product_by_id(4545454545454);
        println!("Received {:?}", result);
        assert_that!(result).is_err();
    }
    #[test]
    fn get_product_by_id_with_valid_id_should_return_ok_for_string() {
        let s ="1";
        let result = get_product_by_id_string(s);
        println!("Received {:?}", result);
        assert_that!(result).is_ok();
    }
    #[test]
    fn get_product_by_id_with_invalid_string_id_should_throw_error_for_string() {
        let s ="dasasasasasas";
        let result = get_product_by_id_string(s);
        println!("Received {:?}", result);
        assert_that!(result).is_err();
    }

}