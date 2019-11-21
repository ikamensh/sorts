use std::time::Instant;
use crate::{is_sorted, random_array};

fn measure_runtime_inplace <F, T> (sort_callable: F, arg: &mut [T]) -> u128
where F: Fn(&mut [T]), T: std::cmp::PartialOrd{

    let start = Instant::now();
    sort_callable(arg);
    let elapsed = start.elapsed().as_millis();
    assert!(is_sorted(arg));

    elapsed
}

pub fn compare_sort_inplace <F> (name: &str, sort_callable: F, time_complexity: TimeComplexity)
where F: Fn(&mut [i8]){
    let test_sizes = test_sizes(time_complexity);


    for size in test_sizes.iter(){
        let size = *size as usize;
        let mut a = random_array(size);
        let time = measure_runtime_inplace(&sort_callable, &mut a);
        println!("{} sort took {}ms to sort {} integers.",name, time, size);
    }
}



fn measure_runtime <F, T> (sort_callable: F, arg: &[T]) -> u128
where F: Fn(&[T]) -> Vec<T>, T: std::cmp::PartialOrd{

    let start = Instant::now();
    let sorted = sort_callable(arg);
    let elapsed = start.elapsed().as_millis();
    assert!(is_sorted(&sorted));

    elapsed
}



pub fn compare_sort <F> (name: &str, sort_callable: F, time_complexity: TimeComplexity)
where F: Fn(&[i8]) -> Vec<i8>{
    let test_sizes = test_sizes(time_complexity);


    for size in test_sizes.iter(){
        let size = *size as usize;
        let a = random_array(size);
        let time = measure_runtime(&sort_callable, &a);
        println!("{} sort took {}ms to sort {} integers.", name, time, size);
    }
}


pub enum TimeComplexity{
    Nlogn,
    Nsquared,
}

fn test_sizes(time_complexity: TimeComplexity) -> [f64; 4]{
    match time_complexity {
        TimeComplexity::Nlogn => {
            [1e3, 1e4, 1e5, 1e6]
        }
        TimeComplexity::Nsquared => {
            [5e2, 1e3, 2e3, 4e3]
        }
    }
}

