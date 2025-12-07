#[cfg(test)]
mod tests {
    use datastructures::array::core::keep_lowest;

    #[test]
    fn test_keep_lowest_basic_merge() {
        let mut a = [1, 3, 5, 7, 9];
        let b = [2, 4, 6, 8, 10];
        keep_lowest(&mut a, b);
        assert_eq!(a, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_keep_lowest_all_from_first() {
        let mut a = [1, 2, 3, 4, 5];
        let b = [6, 7, 8, 9, 10];
        keep_lowest(&mut a, b);
        assert_eq!(a, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_keep_lowest_all_from_second() {
        let mut a = [6, 7, 8, 9, 10];
        let b = [1, 2, 3, 4, 5];
        keep_lowest(&mut a, b);
        assert_eq!(a, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_keep_lowest_with_duplicates() {
        let mut a = [1, 3, 5, 7, 9];
        let b = [1, 3, 5, 7, 9];
        keep_lowest(&mut a, b);
        assert_eq!(a, [1, 1, 3, 3, 5]);
    }

    #[test]
    fn test_keep_lowest_interleaved() {
        let mut a = [1, 4, 7, 10, 13];
        let b = [2, 5, 8, 11, 14];
        keep_lowest(&mut a, b);
        assert_eq!(a, [1, 2, 4, 5, 7]);
    }

    #[test]
    fn test_keep_lowest_single_element() {
        let mut a = [5];
        let b = [3];
        keep_lowest(&mut a, b);
        assert_eq!(a, [3]);
    }

    #[test]
    fn test_keep_lowest_same_values() {
        let mut a = [5, 5, 5, 5, 5];
        let b = [5, 5, 5, 5, 5];
        keep_lowest(&mut a, b);
        assert_eq!(a, [5, 5, 5, 5, 5]);
    }

    #[test]
    fn test_keep_lowest_negative_numbers() {
        let mut a = [-5, -3, -1, 1, 3];
        let b = [-4, -2, 0, 2, 4];
        keep_lowest(&mut a, b);
        assert_eq!(a, [-5, -4, -3, -2, -1]);
    }

    #[test]
    fn test_keep_lowest_mixed_negative_positive() {
        let mut a = [-10, -5, 0, 5, 10];
        let b = [-8, -3, 2, 7, 12];
        keep_lowest(&mut a, b);
        assert_eq!(a, [-10, -8, -5, -3, 0]);
    }

    #[test]
    fn test_keep_lowest_larger_array() {
        let mut a = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let b = [2, 4, 6, 8, 10, 12, 14, 16, 18, 20];
        keep_lowest(&mut a, b);
        assert_eq!(a, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_keep_lowest_with_chars() {
        let mut a = ['a', 'c', 'e', 'g', 'i'];
        let b = ['b', 'd', 'f', 'h', 'j'];
        keep_lowest(&mut a, b);
        assert_eq!(a, ['a', 'b', 'c', 'd', 'e']);
    }

    #[test]
    fn test_keep_lowest_descending_order_corrected() {
        // Arrays should be sorted ascending for correct behavior
        let mut a = [9, 7, 5, 3, 1];
        let b = [10, 8, 6, 4, 2];
        keep_lowest(&mut a, b);
        // With unsorted input, result depends on comparison order
        // This tests actual behavior rather than expected sorted output
        assert_eq!(a, [9, 7, 5, 3, 1]);
    }

    #[test]
    fn test_keep_lowest_zero_values() {
        let mut a = [0, 0, 0, 0, 0];
        let b = [0, 0, 0, 0, 0];
        keep_lowest(&mut a, b);
        assert_eq!(a, [0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_keep_lowest_partial_overlap() {
        let mut a = [1, 5, 10, 15, 20];
        let b = [3, 7, 12, 17, 22];
        keep_lowest(&mut a, b);
        assert_eq!(a, [1, 3, 5, 7, 10]);
    }

    #[test]
    fn test_keep_lowest_u8_type() {
        let mut a: [u8; 5] = [10, 20, 30, 40, 50];
        let b: [u8; 5] = [15, 25, 35, 45, 55];
        keep_lowest(&mut a, b);
        assert_eq!(a, [10, 15, 20, 25, 30]);
    }

    #[test]
    fn test_keep_lowest_extreme_values() {
        let mut a = [i32::MIN, -100, 0, 100, i32::MAX];
        let b = [i32::MIN + 1, -50, 50, 200, i32::MAX - 1];
        keep_lowest(&mut a, b);
        assert_eq!(a, [i32::MIN, i32::MIN + 1, -100, -50, 0]);
    }
}
