use std::fmt::Debug;

/// This module defines the heap data structure
///
#[derive(Debug)]
struct MaxHeap<T> {
    array: Vec<T>,
    heapsize: usize,
}

impl<T> MaxHeap<T>
    where T: std::cmp::Ord + Copy + Debug {
    fn left(&self, idx: usize) -> Option<usize> {
        let result = idx * 2 + 1;
        if result < self.heapsize {
            Some(result)
        } else {
            None
        }
    }

    fn right(&self, idx: usize) -> Option<usize> {
        let result = idx * 2 + 2;
        if result < self.heapsize {
            Some(result)
        } else {
            None
        }
    }

    fn parent(idx: usize) -> Option<usize> {
        if idx > 0 {
            Some((idx - 1) / 2)
        } else {
            None
        }
    }


    pub fn max_heapify(&mut self, idx: usize) {
        let mut options = vec![];

        let mut maybe_option = |maybe_child: Option<usize>| {
            match maybe_child {
                Some(child_idx) => {
                    options.push((&self.array[child_idx], child_idx))
                }
                None => {}
            }
        };

        maybe_option(self.left(idx));
        maybe_option(self.right(idx));

        let swap_with = options.iter().fold((&self.array[idx], idx), |a, &b| a.max(b)).1;

        if swap_with == idx {
            return;
        } else {
            self.array.swap(idx, swap_with);
            self.max_heapify(swap_with);
        }
    }

    pub fn build_heap(array: Vec<T>) -> MaxHeap<T> {
        let mut mh = MaxHeap { heapsize: array.len(), array };
        for idx in (0..mh.heapsize / 2).rev() {
            mh.max_heapify(idx);
        }
        mh
    }

    pub fn has_heap_property(&self) -> bool {
        for i in 0..self.heapsize {
            match self.left(i) {
                Some(child) => {
                    if self.array[i] < self.array[child] {
                        return false;
                    }
                }
                None => {}
            }
            match self.right(i) {
                Some(child) => {
                    if self.array[i] < self.array[child] {
                        return false;
                    }
                }
                None => {}
            }
        }
        true
    }

    fn heapsort(&mut self) {
        while self.heapsize > 0 {
            self.array.swap(0, self.heapsize - 1);
            self.heapsize -= 1;
            self.max_heapify(0)
        }
    }

    fn return_array(self) -> Vec<T> {
        self.array
    }
}

pub fn heapsort<T>(array: &[T]) -> Vec<T>
    where T: std::cmp::Ord + Copy + Debug {

    let mut mh = MaxHeap::build_heap(Vec::from(array));
    mh.heapsort();
    mh.return_array()
}