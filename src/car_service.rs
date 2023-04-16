use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

use crate::json::read_json_file;

// create a struct from
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Car {
    pub id: i64,
    pub price: i64,
    pub brand: String,
    pub model: String,
    pub year: i64,
    pub title_status: String,
    pub mileage: i64,
    pub color: String,
    pub vin: String,
    pub lot: i64,
    pub state: String,
    pub country: String,
    pub condition: String,
}

pub fn get_all_cars() -> Result<Vec<Car>> {
    let response = read_json_file(include_str!("../assets/cars.json"))?;
    let cars: Vec<Car> = serde_json::from_value(response)?;
    Ok(cars)
}