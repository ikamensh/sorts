pub fn is_sorted <T> (array: &[T]) -> bool
    where T: std::cmp::PartialOrd{
    for idx in 1..array.len() {
        if array[idx -1] > array[idx] {
            return false;
        }
    }

    return true;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trivial_sorted() {
        assert!(is_sorted(&Vec::<u8>::new()));
        assert!(is_sorted(&[0u8]));
        assert!(is_sorted(&[1]));
        assert!(is_sorted(&[-144.4]));
        assert!(is_sorted(&[1e9]));
    }

    #[test]
    fn correct_sorted() {
        assert!(is_sorted(&vec![1,2,3,4]));
        assert!(is_sorted(&[-1e10, -144.4, 100500., 1e200]));
    }

    #[test]
    fn correct_not_sorted() {
        assert!(!is_sorted(&vec![1,3,2,4]));
        assert!(!is_sorted(&[-1e10, -144.4, 1e200, 100500.]));
        assert!(!is_sorted(&[-16, -81, -70, -55, 72]))
    }
}