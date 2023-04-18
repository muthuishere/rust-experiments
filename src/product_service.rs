use serde::{Deserialize, Serialize};
use serde_json;
use anyhow::{Context, Result};

use crate::json::read_json_file;

// create a struct from
// #[derive(Debug, Clone, Serialize, Deserialize)]


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub gender: String,
    pub city: String,
    pub state: String,
    pub zipcode: String,
    pub country: String,
    pub dateofbirth: String,
    pub email: String,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRating {
    pub id: i64,
    pub user: User,
    pub rating: i64,
    pub comment: String,
    pub created: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "productId")]
    pub product_id: i64,
    pub name: String,
    pub category: String,
    pub price: f64,
    #[serde(rename = "dealPrice")]
    pub deal_price: f64,
    pub description: String,
    pub manufacturer: String,
    #[serde(rename = "availableItems")]
    pub available_items: i64,
    #[serde(rename = "overAllRating")]
    pub over_all_rating: f64,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    pub reviews: Vec<UserRating>,
}

pub fn get_all_products() -> Result<Vec<Product>> {
    let response = read_json_file(include_str!("../assets/all-products-with-reviews.json"))?;
    let cars: Vec<Product> = serde_json::from_value(response)?;
    Ok(cars)
}

#[cfg(test)]
mod tests {
    use speculoos::*;
    use speculoos::prelude::*;

    use super::*;

    #[test]
    fn get_all_Products_should_be_of_count_46900411() {
        let result = get_all_products();
        println!("Received {:?}", result);
        assert_that!(result).is_ok();
        let result = result.unwrap();
        assert_that!(result.len()).is_equal_to(154);
    }

}