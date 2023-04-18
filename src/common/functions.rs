pub fn identity<T>(x: T) -> T {
    x
}

pub trait Math<T> {
    fn inc(a: T) -> T;

}

impl Math<i32> for i32 {
    fn inc(current: i32) -> i32 {
        current + 1
    }
}

impl Math<i64> for i64 {
    fn inc(current: i64) -> i64 {
        current + 1
    }
}


pub fn counting<K, T>(_: K, current: T) -> T
    where T: Math<T> + Copy + Default
{
    T::inc(current)
}
