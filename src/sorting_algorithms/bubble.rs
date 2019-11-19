pub fn bubble_sort <T> (array: &mut[T])
    where T: std::cmp::PartialOrd{
    for i in 0..array.len(){
        for j in 1..array.len() - i{
            if array[j-1] > array[j]{
                array.swap(j-1, j);
            }
        }
    }
}