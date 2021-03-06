use sorts::{is_sorted, random_array, bubble_sort, heapsort};
//use sorts::{bubble_sort};


fn main() {

    let mut a = random_array(5);
    println!("before sort: {:?}, sorted = {}", a, is_sorted(&a));

//    bubble_sort(&mut a);
    let a = heapsort(a); // use this line for sorts that are not inplace

    println!("after sort: {:?}, sorted = {}", a, is_sorted(&a));

}
