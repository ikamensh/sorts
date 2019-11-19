use rand::prelude::*;

pub fn random_array(size: usize) -> Vec<i8> {
    let mut result = Vec::<i8>::with_capacity(size);

    for _ in 0..size{
        result.push(random());
    }
    result
}