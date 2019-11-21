pub fn merge_sort <T> (array: &[T]) -> Vec<T>
    where T: std::cmp::PartialOrd + Copy{

    if array.len() < 2 {
        return Vec::from(array);
    } else {
        let left_sorted = merge_sort(&array[0..array.len()/2]);
        let right_sorted = merge_sort(&array[array.len()/2..array.len()]);
        return merge(&left_sorted, &right_sorted);
    }
}

/// create a new sorted array out of two sorted arrays left and right.
/// new array will contain all elements from the two subarrays given.
fn merge <T> (left: &[T], right: &[T]) -> Vec<T>
    where T: std::cmp::PartialOrd + Copy{


    let mut result = Vec::<T>::with_capacity(left.len() + right.len());
    let (mut ptr_left, mut ptr_right) = (0, 0);

    while ptr_left < left.len() && ptr_right < right.len(){
        if left[ptr_left] < right[ptr_right] {
            result.push(left[ptr_left]);
            ptr_left += 1;
        } else {
            result.push(right[ptr_right]);
            ptr_right += 1;
        }
    }

    while ptr_left < left.len(){
        result.push(left[ptr_left]);
        ptr_left += 1;
    }

    while ptr_right < right.len(){
        result.push(right[ptr_right]);
        ptr_right += 1;
    }

    result
}