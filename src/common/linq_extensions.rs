use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;

use anyhow::{Context, Result};



// pub trait LinqFunctions<T> {
//     fn counting(input:Iterator::Item,v:T) -> T;
// }
// impl LinqFunctions<i32> for i32 {
//      fn counting(_: Iterator::Item, current: i32) -> i32 {
//         current + 1
//     }
// }
// impl LinqFunctions<i64> for i64 {
//      fn counting(_: Iterator::Item, current: i64) -> i64 {
//         current + 1
//     }
// }

/*
pub trait LinqFunctions<T, U>
where
    U: Iterator,
{
    fn counting(input: U::Item, v: T) -> T;
}

impl<T, U> LinqFunctions<i32, U> for T
where
    U: Iterator<Item = i32>,
{
    fn counting(_: U::Item, current: i32) -> i32 {
        current + 1
    }
}

impl<T

*/


pub trait LinqExtensions: Iterator {
    fn collect_and_wrap(self) -> Result<Vec<Self::Item>>
        where
            Self: Sized;


    fn fold_and_wrap<B, F>(self, init: B, f: F) -> Result<B>
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B;

    fn groupby_and_wrap<K, V>(self, property_extractor: fn(Self::Item) -> K, computable_function: fn(Self::Item, V) -> V) -> Result<HashMap<K, V>>
        where
            Self: Sized,
            K: Eq + Hash ,
            V: Default + Clone,
            Self::Item: Clone;

    fn reduce_and_wrap<F>(self, f: F) -> Result<Self::Item>
        where
            Self: Sized,
            F: FnMut(Self::Item, Self::Item) -> Self::Item;
}


// Implement the custom trait for all Iterator types
impl<T: Iterator> LinqExtensions for T
{
    fn groupby_and_wrap<K, V>(self, property_extractor: fn(Self::Item) -> K, computable_function: fn(Self::Item, V) -> V) -> Result<HashMap<K, V>>
        where
            Self: Sized,
            K: Eq + Hash ,
            V: Default + Clone,
            Self::Item: Clone

    {
        self.fold_and_wrap(HashMap::new(), |mut acc, car| {

            let key = property_extractor(car.clone());

            acc.entry(key).and_modify(|result| {
                let new_result = computable_function(car.clone(), result.clone());
                *result = new_result;
            })
                .or_insert_with(|| {
                    let initial_value = Default::default();
                    computable_function(car.clone(), initial_value)
                });

            acc


        })
    }


    fn collect_and_wrap(self) -> Result<Vec<Self::Item>> {
        Ok(self.collect())
    }

    fn fold_and_wrap<B, F>(mut self, init: B, mut f: F) -> Result<B>
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B {
        Ok(self.fold(init, f))
    }

    fn reduce_and_wrap<F>(mut self, f: F) -> Result<Self::Item>
        where
            Self: Sized,
            F: FnMut(Self::Item, Self::Item) -> Self::Item {
        Ok(self.reduce(f).unwrap())
    }
}

