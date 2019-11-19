mod examine;
pub use examine::is_sorted;

mod random_array;
pub use random_array::random_array;

mod sorting_algorithms;
pub use sorting_algorithms::bubble::bubble_sort;
pub use sorting_algorithms::merge_sort::merge_sort;
//pub use sorting_algorithms::merge_from_github::sort as merge_sort2;

mod measure_runtime;
pub use measure_runtime::{measure_runtime, measure_runtime_inplace};

