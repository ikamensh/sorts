use rand::prelude::*;



pub fn quicksort <T> (array: &mut[T])
where T: std::cmp::PartialOrd + Copy{

    if array.len() < 2{
        return
    } else {
        let i = pivot(array);
        quicksort(&mut array[0..i]);
        let length = array.len();
        quicksort(&mut array[i+1..length]);
//        println!("Called quicksort on array of len {}", array.len());
    }
}

/// Modify the array: pick a pivot element,
/// place all smaller elements in front of it and return its index.
fn pivot <T> (array: &mut[T]) -> usize
where T: std::cmp::PartialOrd + Copy{

//    let pivot_idx = thread_rng().gen_range(0, array.len()-1);
//    array.swap(pivot_idx, array.len()-1);
    let pivot_val = array[array.len()-1];
    let mut i = 0;
    for j in 0..array.len()-1{
        if array[j] < pivot_val {
            array.swap(i, j);
            i += 1;
        }
    }
    array.swap(i, array.len()-1);
    i
}