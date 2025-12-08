#[cfg(test)]
mod tests {
    use datastructures::option::core::{put_option_first, put_option_last};
    use std::cmp::Ordering;

    #[test]
    fn test_compare_both_none() {
        let a: Option<i32> = None;
        let b: Option<i32> = None;
        assert_eq!(put_option_last(&a, &b, |x, y| x.cmp(y)), Ordering::Equal);
    }

    #[test]
    fn test_compare_first_none_second_some() {
        let a: Option<i32> = None;
        let b: Option<i32> = Some(42);
        assert_eq!(put_option_last(&a, &b, |x, y| x.cmp(y)), Ordering::Greater);
    }

    #[test]
    fn test_compare_first_some_second_none() {
        let a: Option<i32> = Some(42);
        let b: Option<i32> = None;
        assert_eq!(put_option_last(&a, &b, |x, y| x.cmp(y)), Ordering::Less);
    }

    #[test]
    fn test_compare_both_some_first_less() {
        let a: Option<i32> = Some(3);
        let b: Option<i32> = Some(5);
        assert_eq!(put_option_last(&a, &b, |x, y| x.cmp(y)), Ordering::Less);
    }

    #[test]
    fn test_compare_both_some_first_greater() {
        let a: Option<i32> = Some(7);
        let b: Option<i32> = Some(2);
        assert_eq!(put_option_last(&a, &b, |x, y| x.cmp(y)), Ordering::Greater);
    }

    #[test]
    fn test_compare_both_some_equal() {
        let a: Option<i32> = Some(42);
        let b: Option<i32> = Some(42);
        assert_eq!(put_option_last(&a, &b, |x, y| x.cmp(y)), Ordering::Equal);
    }

    #[test]
    fn test_compare_with_strings() {
        let a: Option<&str> = Some("apple");
        let b: Option<&str> = Some("banana");
        assert_eq!(put_option_last(&a, &b, |x, y| x.cmp(y)), Ordering::Less);
    }

    #[test]
    fn test_compare_with_strings_first_none() {
        let a: Option<&str> = None;
        let b: Option<&str> = Some("banana");
        assert_eq!(put_option_last(&a, &b, |x, y| x.cmp(y)), Ordering::Greater);
    }

    #[test]
    fn test_compare_with_custom_comparator() {
        let a: Option<i32> = Some(5);
        let b: Option<i32> = Some(3);
        // Custom comparator that reverses the order
        assert_eq!(put_option_last(&a, &b, |x, y| y.cmp(x)), Ordering::Less);
    }

    #[test]
    fn test_sorting_with_option_last() {
        let mut values = vec![Some(3), None, Some(1), None, Some(2)];

        values.sort_by(|a, b| put_option_last(a, b, |x, y| x.cmp(y)));

        let expected = vec![Some(1), Some(2), Some(3), None, None];

        assert_eq!(values, expected);
    }

    #[test]
    fn test_put_option_first_both_none() {
        let a: Option<i32> = None;
        let b: Option<i32> = None;
        assert_eq!(put_option_first(&a, &b, |x, y| x.cmp(y)), Ordering::Equal);
    }

    #[test]
    fn test_put_option_first_none_vs_some() {
        let a: Option<i32> = None;
        let b = Some(5);
        assert_eq!(put_option_first(&a, &b, |x, y| x.cmp(y)), Ordering::Less);
    }

    #[test]
    fn test_put_option_first_some_vs_none() {
        let a = Some(5);
        let b: Option<i32> = None;
        assert_eq!(put_option_first(&a, &b, |x, y| x.cmp(y)), Ordering::Greater);
    }

    #[test]
    fn test_put_option_first_both_some_less() {
        let a = Some(3);
        let b = Some(5);
        assert_eq!(put_option_first(&a, &b, |x, y| x.cmp(y)), Ordering::Less);
    }

    #[test]
    fn test_put_option_first_both_some_greater() {
        let a = Some(5);
        let b = Some(3);
        assert_eq!(put_option_first(&a, &b, |x, y| x.cmp(y)), Ordering::Greater);
    }

    #[test]
    fn test_put_option_first_both_some_equal() {
        let a = Some(5);
        let b = Some(5);
        assert_eq!(put_option_first(&a, &b, |x, y| x.cmp(y)), Ordering::Equal);
    }

    #[test]
    fn test_put_option_first_sorting() {
        let mut values = vec![Some(3), None, Some(1), Some(2), None];

        values.sort_by(|a, b| put_option_first(a, b, |x, y| x.cmp(y)));

        let expected = vec![None, None, Some(1), Some(2), Some(3)];

        assert_eq!(values, expected);
    }
}
