#![cfg(not(feature = "no-std"))]

#[cfg(test)]
mod tests {
    use datastructures::vec::core::{keep_lowest_vec, keep_lowest_vec_by};

    #[test]
    fn test_keep_lowest_vec_basic_merge() {
        let mut a = vec![1, 3, 5, 7, 9];
        let b = vec![2, 4, 6, 8, 10];
        keep_lowest_vec(&mut a, b);
        assert_eq!(a, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_keep_lowest_vec_duplicates() {
        let mut a = vec![1, 3, 5, 7, 9];
        let b = vec![1, 3, 5, 7, 9];
        keep_lowest_vec(&mut a, b);
        assert_eq!(a, vec![1, 1, 3, 3, 5]);
    }

    #[test]
    fn test_keep_lowest_vec_all_from_second() {
        let mut a = vec![6, 7, 8, 9, 10];
        let b = vec![1, 2, 3, 4, 5];
        keep_lowest_vec(&mut a, b);
        assert_eq!(a, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_keep_lowest_vec_by_custom_order() {
        let mut a = vec!["e", "c", "a", "b"]; // sorted descending for test
        let b = vec!["d", "c", "b", "a"];
        // Reverse order comparator
        keep_lowest_vec_by(&mut a, b, |x, y| y.cmp(x));
        assert_eq!(a, vec!["e", "d", "c", "c"]);
    }

    #[test]
    fn test_keep_lowest_vec_v2_longer() {
        let mut a = vec![5, 6, 7];
        let b = vec![1, 2, 3, 4];
        keep_lowest_vec(&mut a, b);
        assert_eq!(a, vec![1, 2, 3]);
    }

    #[test]
    fn test_keep_lowest_vec_v2_shorter() {
        let mut a = vec![1, 2, 3, 4];
        let b = vec![0];
        keep_lowest_vec(&mut a, b);
        assert_eq!(a, vec![0, 1, 2, 3]);
    }
}
