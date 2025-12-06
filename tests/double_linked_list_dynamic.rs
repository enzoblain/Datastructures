#![cfg(not(feature = "no-std"))]

#[cfg(test)]
mod tests {
    use datastructures::DoubleLinkedList::DoubleLinkedList;
    use datastructures::LinkedListError;

    #[test]
    fn test_insert_head_empty_list() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        assert!(list.insert_head(42).is_ok());
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_insert_head_multiple() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        assert!(list.insert_head(1).is_ok());
        assert!(list.insert_head(2).is_ok());
        assert!(list.insert_head(3).is_ok());

        assert_eq!(list.len(), 3);
        assert_eq!(*list.get(0).unwrap(), 3);
        assert_eq!(*list.get(1).unwrap(), 2);
        assert_eq!(*list.get(2).unwrap(), 1);
    }

    #[test]
    fn test_insert_tail_empty_list() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        assert!(list.insert_tail(42).is_ok());
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_insert_tail_multiple() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        assert!(list.insert_tail(1).is_ok());
        assert!(list.insert_tail(2).is_ok());
        assert!(list.insert_tail(3).is_ok());

        assert_eq!(list.len(), 3);
        assert_eq!(*list.get(0).unwrap(), 1);
        assert_eq!(*list.get(1).unwrap(), 2);
        assert_eq!(*list.get(2).unwrap(), 3);
    }

    #[test]
    fn test_insert_after() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        assert!(list.insert_head(1).is_ok());
        assert!(list.insert_head(3).is_ok());
        assert!(list.insert_after(0, 2).is_ok());

        assert_eq!(list.len(), 3);
        assert_eq!(*list.get(0).unwrap(), 3);
        assert_eq!(*list.get(1).unwrap(), 2);
        assert_eq!(*list.get(2).unwrap(), 1);
    }

    #[test]
    fn test_insert_after_out_of_range() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        assert!(list.insert_head(1).is_ok());
        match list.insert_after(5, 2) {
            Err(LinkedListError::IndexOutOfRange) => (),
            _ => panic!("Expected IndexOutOfRange error"),
        }
    }

    #[test]
    fn test_insert_before() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        assert!(list.insert_tail(1).is_ok());
        assert!(list.insert_tail(3).is_ok());
        assert!(list.insert_before(1, 2).is_ok());

        assert_eq!(list.len(), 3);
        assert_eq!(*list.get(0).unwrap(), 1);
        assert_eq!(*list.get(1).unwrap(), 2);
        assert_eq!(*list.get(2).unwrap(), 3);
    }

    #[test]
    fn test_insert_before_head() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        assert!(list.insert_tail(2).is_ok());
        assert!(list.insert_before(0, 1).is_ok());

        assert_eq!(list.len(), 2);
        assert_eq!(*list.get(0).unwrap(), 1);
        assert_eq!(*list.get(1).unwrap(), 2);
    }

    #[test]
    fn test_insert_before_out_of_range() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        assert!(list.insert_head(1).is_ok());
        match list.insert_before(5, 2) {
            Err(LinkedListError::IndexOutOfRange) => (),
            _ => panic!("Expected IndexOutOfRange error"),
        }
    }

    #[test]
    fn test_get() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        assert!(list.insert_tail(10).is_ok());
        assert!(list.insert_tail(20).is_ok());
        assert!(list.insert_tail(30).is_ok());

        assert_eq!(*list.get(0).unwrap(), 10);
        assert_eq!(*list.get(1).unwrap(), 20);
        assert_eq!(*list.get(2).unwrap(), 30);
    }

    #[test]
    fn test_get_out_of_range() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        assert!(list.insert_tail(10).is_ok());

        match list.get(5) {
            Err(LinkedListError::IndexOutOfRange) => (),
            _ => panic!("Expected IndexOutOfRange error"),
        }
    }

    #[test]
    fn test_remove_head() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        assert!(list.insert_tail(1).is_ok());
        assert!(list.insert_tail(2).is_ok());
        assert!(list.insert_tail(3).is_ok());

        assert!(list.remove(0).is_ok());
        assert_eq!(list.len(), 2);
        assert_eq!(*list.get(0).unwrap(), 2);
        assert_eq!(*list.get(1).unwrap(), 3);
    }

    #[test]
    fn test_remove_tail() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        assert!(list.insert_tail(1).is_ok());
        assert!(list.insert_tail(2).is_ok());
        assert!(list.insert_tail(3).is_ok());

        assert!(list.remove(2).is_ok());
        assert_eq!(list.len(), 2);
        assert_eq!(*list.get(0).unwrap(), 1);
        assert_eq!(*list.get(1).unwrap(), 2);
    }

    #[test]
    fn test_remove_middle() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        assert!(list.insert_tail(1).is_ok());
        assert!(list.insert_tail(2).is_ok());
        assert!(list.insert_tail(3).is_ok());

        assert!(list.remove(1).is_ok());
        assert_eq!(list.len(), 2);
        assert_eq!(*list.get(0).unwrap(), 1);
        assert_eq!(*list.get(1).unwrap(), 3);
    }

    #[test]
    fn test_remove_only_element() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        assert!(list.insert_tail(42).is_ok());
        assert!(list.remove(0).is_ok());

        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_remove_out_of_range() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        assert!(list.insert_tail(1).is_ok());

        match list.remove(5) {
            Err(LinkedListError::IndexOutOfRange) => (),
            _ => panic!("Expected IndexOutOfRange error"),
        }
    }

    #[test]
    fn test_insert_remove_reuse_slots() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        for i in 0..5 {
            assert!(list.insert_tail(i).is_ok());
        }
        assert_eq!(list.len(), 5);

        assert!(list.remove(2).is_ok());
        assert_eq!(list.len(), 4);

        assert!(list.insert_tail(99).is_ok());
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn test_complex_operations() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        for i in 1..=5 {
            assert!(list.insert_tail(i).is_ok());
        }

        assert!(list.insert_head(0).is_ok());
        assert!(list.insert_after(2, 10).is_ok());
        assert!(list.remove(0).is_ok());

        let last = list.len() - 1;
        assert!(list.remove(last).is_ok());

        assert_eq!(*list.get(0).unwrap(), 1);
        assert_eq!(*list.get(1).unwrap(), 2);
        assert_eq!(*list.get(2).unwrap(), 10);
        assert_eq!(*list.get(3).unwrap(), 3);
        assert_eq!(*list.get(4).unwrap(), 4);
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn test_forward_backward_traversal() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        for i in 0..10 {
            assert!(list.insert_tail(i).is_ok());
        }

        assert_eq!(*list.get(0).unwrap(), 0);
        assert_eq!(*list.get(1).unwrap(), 1);
        assert_eq!(*list.get(4).unwrap(), 4);

        assert_eq!(*list.get(9).unwrap(), 9);
        assert_eq!(*list.get(8).unwrap(), 8);
        assert_eq!(*list.get(5).unwrap(), 5);
    }

    #[test]
    fn test_with_strings() {
        let mut list: DoubleLinkedList<String> = Default::default();

        assert!(list.insert_tail("Hello".to_string()).is_ok());
        assert!(list.insert_tail("World".to_string()).is_ok());
        assert!(list.insert_tail("!".to_string()).is_ok());

        assert_eq!(*list.get(0).unwrap(), "Hello");
        assert_eq!(*list.get(1).unwrap(), "World");
        assert_eq!(*list.get(2).unwrap(), "!");
    }

    #[test]
    fn test_remove_all_elements() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        for i in 0..5 {
            assert!(list.insert_tail(i).is_ok());
        }

        for _ in 0..5 {
            assert!(list.remove(0).is_ok());
        }

        assert_eq!(list.len(), 0);
        assert!(list.is_empty());

        assert!(list.insert_tail(100).is_ok());
        assert_eq!(*list.get(0).unwrap(), 100);
    }

    #[test]
    fn test_iter_and_compute() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        // Insert elements 1, 2, 3
        assert!(list.insert_tail(1).is_ok());
        assert!(list.insert_tail(2).is_ok());
        assert!(list.insert_tail(3).is_ok());

        // Double all elements
        list.iter_and_compute(|val| *val *= 2);

        // Verify all elements have been doubled
        assert_eq!(*list.get(0).unwrap(), 2);
        assert_eq!(*list.get(1).unwrap(), 4);
        assert_eq!(*list.get(2).unwrap(), 6);
    }

    #[test]
    fn test_iter_and_compute_empty_list() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        // This should not panic on an empty list
        list.iter_and_compute(|val| *val += 1);

        assert!(list.is_empty());
    }

    #[test]
    fn test_iter_and_compute_complex_operation() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        assert!(list.insert_tail(5).is_ok());
        assert!(list.insert_tail(10).is_ok());
        assert!(list.insert_tail(15).is_ok());

        // Add 100 to each element and then multiply by 2
        list.iter_and_compute(|val| {
            *val += 100;
            *val *= 2;
        });

        assert_eq!(*list.get(0).unwrap(), 210); // (5 + 100) * 2
        assert_eq!(*list.get(1).unwrap(), 220); // (10 + 100) * 2
        assert_eq!(*list.get(2).unwrap(), 230); // (15 + 100) * 2
    }

    #[test]
    fn test_sort_by_ascending() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        for v in [5, 1, 3, 2, 4] {
            assert!(list.insert_tail(v).is_ok());
        }

        list.sort_by(|a, b| a.cmp(b));

        for (i, expected) in (1..=5).enumerate() {
            assert_eq!(*list.get(i).unwrap(), expected);
        }
    }

    #[test]
    fn test_sort_by_stable_with_duplicates() {
        let mut list: DoubleLinkedList<(i32, usize)> = Default::default();

        let items = [(2, 0), (1, 0), (2, 1), (1, 1), (2, 2)];

        for item in items {
            assert!(list.insert_tail(item).is_ok());
        }

        // Sort only by the first field; stability should keep second field order for equals.
        list.sort_by(|a, b| a.0.cmp(&b.0));

        // Expect all (1, *) first preserving insertion order, then (2, *) preserving order.
        let expected = [(1, 0), (1, 1), (2, 0), (2, 1), (2, 2)];

        for (i, exp) in expected.iter().enumerate() {
            assert_eq!(*list.get(i).unwrap(), *exp);
        }
    }

    #[test]
    fn test_get_sorted_by_does_not_mutate_original() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        for v in [3, 1, 2] {
            assert!(list.insert_tail(v).is_ok());
        }

        let sorted = list.get_sorted_by(|a, b| a.cmp(b));

        // Original remains unchanged
        assert_eq!(*list.get(0).unwrap(), 3);
        assert_eq!(*list.get(1).unwrap(), 1);
        assert_eq!(*list.get(2).unwrap(), 2);

        // Sorted copy is ordered
        assert_eq!(*sorted.get(0).unwrap(), 1);
        assert_eq!(*sorted.get(1).unwrap(), 2);
        assert_eq!(*sorted.get(2).unwrap(), 3);
    }

    #[test]
    fn test_get_sorted_by_empty_list() {
        let list: DoubleLinkedList<i32> = Default::default();

        let sorted = list.get_sorted_by(|a, b| a.cmp(b));

        assert!(sorted.is_empty());
        assert!(list.is_empty());
    }

    #[test]
    fn test_get_index_where() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        assert!(list.insert_tail(5).is_ok());
        assert!(list.insert_tail(10).is_ok());
        assert!(list.insert_tail(15).is_ok());

        let idx = list.get_index_where(|v| *v > 7);
        assert_eq!(idx, Some(1));

        let none_idx = list.get_index_where(|v| *v == 99);
        assert!(none_idx.is_none());
    }

    #[test]
    fn test_get_value_where() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        assert!(list.insert_tail(3).is_ok());
        assert!(list.insert_tail(4).is_ok());
        assert!(list.insert_tail(5).is_ok());

        let val = list.get_value_where(|v| *v % 2 == 0);
        assert_eq!(val, Some(&4));

        let none_val = list.get_value_where(|v| *v < 0);
        assert!(none_val.is_none());
    }

    #[test]
    fn test_select_n_first_by_returns_minimals() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        for v in [5, 1, 4, 2, 3] {
            assert!(list.insert_tail(v).is_ok());
        }

        let (values, len) = list.select_n_first_by(2, |a, b| a.cmp(b));
        assert_eq!(len, 2);
        assert_eq!(values, vec![1, 2]);

        // original list untouched
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn test_select_n_first_by_handles_n_greater_than_len() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        for v in [7, 2, 9] {
            assert!(list.insert_tail(v).is_ok());
        }

        let (values, len) = list.select_n_first_by(5, |a, b| a.cmp(b));
        assert_eq!(len, 3);
        assert_eq!(values, vec![2, 7, 9]);
    }

    #[test]
    fn test_as_vec_returns_cloned_values() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        assert!(list.insert_tail(10).is_ok());
        assert!(list.insert_tail(20).is_ok());
        assert!(list.insert_tail(30).is_ok());

        let vec = list.as_vec();

        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0], 10);
        assert_eq!(vec[1], 20);
        assert_eq!(vec[2], 30);

        // Original list remains intact
        assert_eq!(list.len(), 3);
        assert_eq!(*list.get(0).unwrap(), 10);
        assert_eq!(*list.get(1).unwrap(), 20);
        assert_eq!(*list.get(2).unwrap(), 30);
    }

    #[test]
    fn test_unlimited_growth_beyond_sized_capacity() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        // Insert more than 63 elements (SizedDoubleLinkedList limit)
        for i in 0..100 {
            assert!(list.insert_tail(i).is_ok());
        }

        assert_eq!(list.len(), 100);

        // Verify all elements are accessible
        for i in 0..100 {
            assert_eq!(*list.get(i).unwrap(), i as i32);
        }
    }

    #[test]
    fn test_large_scale_insert_remove_operations() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        // Insert 1000 elements
        for i in 0..1000 {
            assert!(list.insert_tail(i).is_ok());
        }

        assert_eq!(list.len(), 1000);

        // Remove every other element
        for _ in 0..500 {
            assert!(list.remove(0).is_ok());
        }

        assert_eq!(list.len(), 500);

        // Insert 500 more elements
        for i in 1000..1500 {
            assert!(list.insert_tail(i).is_ok());
        }

        assert_eq!(list.len(), 1000);
    }

    #[test]
    fn test_free_list_reuse_after_multiple_removes() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        // Fill with 20 elements
        for i in 0..20 {
            assert!(list.insert_tail(i).is_ok());
        }

        // Remove elements at various positions
        assert!(list.remove(5).is_ok());
        assert!(list.remove(10).is_ok());
        assert!(list.remove(3).is_ok());

        assert_eq!(list.len(), 17);

        // Insert new elements should reuse freed slots
        for i in 100..103 {
            assert!(list.insert_tail(i).is_ok());
        }

        assert_eq!(list.len(), 20);

        // Verify list integrity
        let vec = list.as_vec();
        assert_eq!(vec.len(), 20);
    }

    #[test]
    fn test_sort_large_list() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        // Insert 100 elements in reverse order
        for i in (0..100).rev() {
            assert!(list.insert_tail(i).is_ok());
        }

        list.sort_by(|a, b| a.cmp(b));

        // Verify sorted order
        for i in 0..100 {
            assert_eq!(*list.get(i).unwrap(), i as i32);
        }
    }

    #[test]
    fn test_select_n_first_from_large_list() {
        let mut list: DoubleLinkedList<i32> = Default::default();

        // Insert 200 random-ish values
        for i in 0..200 {
            assert!(list.insert_tail((i * 7 + 13) % 200).is_ok());
        }

        let (values, len) = list.select_n_first_by(10, |a, b| a.cmp(b));
        assert_eq!(len, 10);

        // Verify values are sorted and are the minimals
        for i in 0..9 {
            assert!(values[i] <= values[i + 1]);
        }

        // Original list untouched
        assert_eq!(list.len(), 200);
    }
}
