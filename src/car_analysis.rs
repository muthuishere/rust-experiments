use std::collections::HashMap;
use std::fmt::Error;
use std::hash::Hash;
use std::result;
use itertools::Itertools;
use serde_json;
use crate::common::linq_extensions::*;
use anyhow::{Context, Result};

use crate::car_service;
use crate::car_service::Car;

use std::iter::Iterator;
use tokio::count;
use crate::common::functions;


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



fn increment_color_countold(mut colors_with_count :HashMap<String,i32>, car:Car) -> HashMap<String, i32> {
    let count = colors_with_count.entry(car.color).or_default();
    *count += 1;
    colors_with_count
}





pub fn get_car_color_and_count_old() -> Result<HashMap<String,i32>> {
   car_service::get_all_cars()?
      .into_iter()
        .fold_and_wrap(HashMap::new(), increment_color_countold)

}



//
//
// fn increment_color_count(mut colors_with_count :HashMap<String,i32>, car:Car) -> HashMap<String, i32> {
//     // let count = colors_with_count.entry(car.color).or_default();
//     // *count += 1;
//     // colors_with_count
//     do_operation(colors_with_count,car,|car| car.color,|count| count+1)
// }

pub fn get_car_color_and_count_with_custom_function() -> Result<HashMap<String,i32>> {
   car_service::get_all_cars()?
      .into_iter()
       .groupby_and_wrap(|car| car.color, |_, count| count+1)

}

pub fn get_car_color_and_count_with_counting() -> Result<HashMap<String,i32>> {
   car_service::get_all_cars()?
      .into_iter()
       .groupby_and_wrap(|car| car.color, functions::counting)


}

pub fn get_car_color_and_collect_all() -> Result<HashMap<String,Vec<Car>>> {
   car_service::get_all_cars()?
      .into_iter()
       .groupby_and_wrap(|car| car.color, |car, mut count| {
              count.push(car);
              count
       })


}



pub fn get_car_brand_and_sum_of_price() -> Result<HashMap<String,i32>> {
   car_service::get_all_cars()?
      .into_iter()
       .groupby_and_wrap(|car| car.brand, |car, count| (car.price as i32) + count )

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
    fn get_car_color_and_count_with_computable_should_be_of_count_48() {
        let result = get_car_color_and_count_with_custom_function();
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
    #[test]
    fn gget_car_color_and_count_with_counting_should_be_of_count_48() {
        let result = get_car_color_and_count_with_counting();
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
    #[test]
    fn get_car_color_and_collect_all_should_return_alls() {
        let result = get_car_color_and_collect_all();
        println!("Received {:?}", result);
        assert_that!(result).is_ok();
        let result = result.unwrap();
        //Received Ok({"cayenne red": 2, "competition orange": 1, "oxford white": 4, "charcoal": 18, "light blue": 1, "ingot silver": 1, "tan": 1, "beige": 5, "morningsky blue": 1, "ruby red": 1, "bright white clearcoat": 2, "triple yellow tri-coat": 3, "white": 707, "billet silver metallic clearcoat": 3, "kona blue metallic": 1, "lightning blue": 1, "green": 24, "tuxedo black metallic": 2, "blue": 151, "black clearcoat": 2, "orange": 20, "royal crimson metallic tinted clearcoat": 1, "ingot silver metallic": 4, "no_color": 61, "ruby red metallic tinted clearcoat": 2, "off-white": 2, "yellow": 9, "shadow black": 5, "silver": 300, "dark blue": 1, "toreador red": 1, "magnetic metallic": 6, "super black": 3, "red": 192, "gray": 395, "maroon": 1, "white platinum tri-coat metallic": 2, "pearl white": 1, "glacier white": 1, "turquoise": 1, "jazz blue pearlcoat": 1, "burgundy": 1, "brown": 20, "guard": 1, "gold": 19, "black": 516, "purple": 1, "phantom black": 1})

        assert_that!(result.len()).is_equal_to(48);

        //
        // for (color, count) in result.iter() {
        //     println!("{}: {}", color, count.toString()  );
        // }

    }
    #[test]
    fn get_car_brand_and_sum_of_price_should_be_of_count_48() {
        let result = get_car_brand_and_sum_of_price();
        println!("Received {:?}", result);
        assert_that!(result).is_ok();
        let result = result.unwrap();
        //Received Ok({"lexus": 66440, "land": 115600, "mazda": 16000, "buick": 256305, "ram": 11050, "honda": 73530, "infiniti": 157180, "jaguar": 2800, "ford": 26758607, "harley-davidson": 54680, "cadillac": 249410, "heartland": 14830, "maserati": 30300, "dodge": 7681819, "peterbilt": 1600, "nissan": 3764536, "chevrolet": 5544976, "bmw": 448750, "lincoln": 36300, "toyota": 6300, "gmc": 447610, "jeep": 328015, "hyundai": 78048, "chrysler": 246350, "kia": 144610, "mercedes-benz": 287040, "acura": 21800, "audi": 55925})


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