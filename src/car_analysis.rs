use std::collections::HashMap;
use std::fmt::Error;
use std::hash::Hash;
use std::result;

use serde_json::{Result, Value};

use crate::car_service;
use crate::car_service::Car;

use std::iter::Iterator;

// Define the custom trait


pub trait LinqExtensions: Iterator {
    fn collect_and_wrap(self) -> Result<Vec<Self::Item>>
        where
            Self: Sized;


    fn fold_and_wrap<B, F>( self, init: B,  f: F) -> Result<B>
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B;

    fn reduce_and_wrap<F>(self, f: F) -> Result<Self::Item>
        where
            Self: Sized,
            F: FnMut(Self::Item, Self::Item) -> Self::Item;

}


// Implement the custom trait for all Iterator types
impl<T: Iterator> LinqExtensions for T {
    fn collect_and_wrap(self) -> Result<Vec<Self::Item>> {
        Ok(self.collect())
    }

    fn fold_and_wrap<B, F>(mut self, init: B, mut f: F) -> Result<B>
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B{
        Ok(self.fold(init, f))

    }

    fn reduce_and_wrap<F>(mut self, f: F) -> Result<Self::Item>
        where
            Self: Sized,
            F: FnMut(Self::Item, Self::Item) -> Self::Item{
        Ok(self.reduce(f).unwrap())

    }
}

// Whenver you use ? mark to unwrap and do things its your responsibility to return Ok or Err at end of function
// Always use clone() when you are using filter() or map() or any other function which returns iterator
// use into_iter instead of iter to pass ownership of the elements to the iterator and avoid the clone
pub fn get_cars_in_2017() -> Result<Vec<Car>> {
   car_service::get_all_cars()?
            .into_iter()
            .filter(|car| car.year == 2017)
            .collect_and_wrap()
}


pub fn get_chevrolet_cars_in_2016() -> Result<Vec<Car>> {

  car_service::get_all_cars()?
        .into_iter()
        .filter(|car| car.brand == "chevrolet" && car.year == 2016)
      .collect_and_wrap()
}
fn increment_color_count(mut colors_with_count :HashMap<String,i32>, car:Car) -> HashMap<String, i32> {
    let count = colors_with_count.entry(car.color).or_default();
    *count += 1;
    colors_with_count

}
pub fn get_car_color_and_count() -> Result<HashMap<String,i32>> {
   car_service::get_all_cars()?
      .into_iter()
        .fold_and_wrap(HashMap::new(), increment_color_count)

}


pub fn get_car_total_net_worth() -> Result<i64> {
   car_service::get_all_cars()?
      .into_iter()
       .map(|car| car.price as i64)
       .reduce_and_wrap(|acc, price| acc + price)

}

#[cfg(test)]
mod tests {
    use speculoos::*;
    use speculoos::prelude::*;

    use super::*;

    #[test]
    fn get_car_total_net_worth_should_be_of_count_46900411() {
        let result = get_car_total_net_worth();
        println!("Received {:?}", result);
        assert_that!(result).is_ok();
        let result = result.unwrap();
        assert_that!(result).is_equal_to(46900411);
    }
    #[test]
    fn get_cars_in_2017_should_be_of_count_377() {
        let result = get_cars_in_2017();
        println!("Received {:?}", result);
        assert_that!(result).is_ok();
        let result = result.unwrap();
        assert_that!(result.len()).is_equal_to(377);
    }
    #[test]
    fn get_chevrolet_cars_in_2016_should_be_of_count_43() {
        let result = get_chevrolet_cars_in_2016();
        println!("Received {:?}", result);
        assert_that!(result).is_ok();
        let result = result.unwrap();
        assert_that!(result.len()).is_equal_to(43);
    }
    #[test]
    fn get_car_color_and_count_should_be_of_count_48() {
        let result = get_car_color_and_count();
        println!("Received {:?}", result);
        assert_that!(result).is_ok();
        let result = result.unwrap();
        //Received Ok({"cayenne red": 2, "competition orange": 1, "oxford white": 4, "charcoal": 18, "light blue": 1, "ingot silver": 1, "tan": 1, "beige": 5, "morningsky blue": 1, "ruby red": 1, "bright white clearcoat": 2, "triple yellow tri-coat": 3, "white": 707, "billet silver metallic clearcoat": 3, "kona blue metallic": 1, "lightning blue": 1, "green": 24, "tuxedo black metallic": 2, "blue": 151, "black clearcoat": 2, "orange": 20, "royal crimson metallic tinted clearcoat": 1, "ingot silver metallic": 4, "no_color": 61, "ruby red metallic tinted clearcoat": 2, "off-white": 2, "yellow": 9, "shadow black": 5, "silver": 300, "dark blue": 1, "toreador red": 1, "magnetic metallic": 6, "super black": 3, "red": 192, "gray": 395, "maroon": 1, "white platinum tri-coat metallic": 2, "pearl white": 1, "glacier white": 1, "turquoise": 1, "jazz blue pearlcoat": 1, "burgundy": 1, "brown": 20, "guard": 1, "gold": 19, "black": 516, "purple": 1, "phantom black": 1})

        assert_that!(result.len()).is_equal_to(48);

        assert_that!(&result).contains_key(format!("charcoal"));
        assert_that!(result.get("charcoal")).is_some();
        assert_that!(result.get("charcoal").unwrap()).is_equal_to(&18);
        assert_that!(result.get("light blue")).is_some();
        assert_that!(result.get("light blue").unwrap()).is_equal_to(&1);


        for (color, count) in result.iter() {
            println!("{}: {}", color, count);
        }

    }
}