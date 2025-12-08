#[cfg(test)]
mod tests {
    use core::mem::MaybeUninit;
    use datastructures::array::core::swap_maybeuninit_to_option_array;

    #[test]
    fn test_swap_all_initialized() {
        let arr: [MaybeUninit<i32>; 5] = [
            MaybeUninit::new(1),
            MaybeUninit::new(2),
            MaybeUninit::new(3),
            MaybeUninit::new(4),
            MaybeUninit::new(5),
        ];
        let result = swap_maybeuninit_to_option_array(arr, 5);
        assert_eq!(result[0], Some(1));
        assert_eq!(result[1], Some(2));
        assert_eq!(result[2], Some(3));
        assert_eq!(result[3], Some(4));
        assert_eq!(result[4], Some(5));
    }

    #[test]
    fn test_swap_partial_initialized() {
        let arr: [MaybeUninit<i32>; 5] = [
            MaybeUninit::new(10),
            MaybeUninit::new(20),
            MaybeUninit::new(30),
            MaybeUninit::uninit(),
            MaybeUninit::uninit(),
        ];
        let result = swap_maybeuninit_to_option_array(arr, 3);
        assert_eq!(result[0], Some(10));
        assert_eq!(result[1], Some(20));
        assert_eq!(result[2], Some(30));
        assert_eq!(result[3], None);
        assert_eq!(result[4], None);
    }

    #[test]
    fn test_swap_single_element() {
        let arr: [MaybeUninit<i32>; 3] = [
            MaybeUninit::new(42),
            MaybeUninit::uninit(),
            MaybeUninit::uninit(),
        ];
        let result = swap_maybeuninit_to_option_array(arr, 1);
        assert_eq!(result[0], Some(42));
        assert_eq!(result[1], None);
        assert_eq!(result[2], None);
    }

    #[test]
    fn test_swap_no_elements() {
        let arr: [MaybeUninit<i32>; 5] = [
            MaybeUninit::uninit(),
            MaybeUninit::uninit(),
            MaybeUninit::uninit(),
            MaybeUninit::uninit(),
            MaybeUninit::uninit(),
        ];
        let result = swap_maybeuninit_to_option_array(arr, 0);
        assert_eq!(result[0], None);
        assert_eq!(result[1], None);
        assert_eq!(result[2], None);
        assert_eq!(result[3], None);
        assert_eq!(result[4], None);
    }

    #[test]
    fn test_swap_with_strings() {
        let arr: [MaybeUninit<&str>; 3] = [
            MaybeUninit::new("hello"),
            MaybeUninit::new("world"),
            MaybeUninit::uninit(),
        ];
        let result = swap_maybeuninit_to_option_array(arr, 2);
        assert_eq!(result[0], Some("hello"));
        assert_eq!(result[1], Some("world"));
        assert_eq!(result[2], None);
    }

    #[test]
    fn test_swap_with_chars() {
        let arr: [MaybeUninit<char>; 4] = [
            MaybeUninit::new('a'),
            MaybeUninit::new('b'),
            MaybeUninit::new('c'),
            MaybeUninit::uninit(),
        ];
        let result = swap_maybeuninit_to_option_array(arr, 3);
        assert_eq!(result[0], Some('a'));
        assert_eq!(result[1], Some('b'));
        assert_eq!(result[2], Some('c'));
        assert_eq!(result[3], None);
    }
}
