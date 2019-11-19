use std::time::Instant;
use crate::{is_sorted};

pub fn measure_runtime_inplace <F, T> (sorting_foo: F, arg: &mut [T]) -> u128
where F: Fn(&mut [T]), T: std::cmp::PartialOrd{

    let start = Instant::now();
    sorting_foo(arg);
    let elapsed = start.elapsed().as_millis();
    assert!(is_sorted(arg));

    elapsed
}


pub fn measure_runtime <F, T> (sorting_foo: F, arg: &[T]) -> u128
where F: Fn(&[T]) -> Vec<T>, T: std::cmp::PartialOrd{

    let start = Instant::now();
    let sorted = sorting_foo(arg);
    let elapsed = start.elapsed().as_millis();
    assert!(is_sorted(&sorted));

    elapsed
}


