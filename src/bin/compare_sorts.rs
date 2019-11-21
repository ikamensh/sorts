use sorts::{bubble_sort, merge_sort, quicksort, heapsort};
use sorts::{compare_sort_inplace, compare_sort, TimeComplexity};



fn main() {

    compare_sort("heap", heapsort, TimeComplexity::Nlogn);

    compare_sort_inplace("bubble", bubble_sort, TimeComplexity::Nsquared);
    compare_sort("merge", merge_sort, TimeComplexity::Nlogn);
    compare_sort_inplace("quick", quicksort, TimeComplexity::Nlogn);

}
