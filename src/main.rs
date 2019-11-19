use sorts::{is_sorted, random_array, bubble_sort, merge_sort, measure_runtime_inplace, measure_runtime};


fn main() {

//    let mut a = random_array(5);
//    println!("before sort: {:?}, sorted = {}", a, is_sorted(&a));
//
////    bubble_sort(&mut a);
//    let a = merge_sort(&a);
//
//    println!("after sort: {:?}, sorted = {}", a, is_sorted(&a));

//    for size in [1e3, 3e3, 6e3, 1e4, 2e4].iter(){
//        let size = *size as usize;
//        let mut a = random_array(size);
//        let time = measure_runtime_inplace(bubble_sort, &mut a);
//        println!("Bubble sort took {}ms to sort {} integers.", time, size);
//    }


    for size in [1e3, 1e4, 1e5, 1e6, 1e7].iter(){
        let size = *size as usize;
        let a = random_array(size);
        let time = measure_runtime(merge_sort, &a);
        println!("Merge sort took {}ms to sort {} integers.", time, size);
    }

//    for size in [1e3, 1e4, 1e5, 1e6, 1e7].iter(){
//        let size = *size as usize;
//        let mut a = random_array(size);
//        let time = measure_runtime_inplace(merge_sort2, &mut a);
//        println!("Bubble sort took {}ms to sort {} integers.", time, size);
//    }

}
