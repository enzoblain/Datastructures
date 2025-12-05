#[cfg(test)]
mod tests {
    use datastructures::DoubleLinkedList::SizedDoubleLinkedList;
    use datastructures::LinkedListError;

    #[test]
    fn test_insert_head_empty_list() {
        let mut list: SizedDoubleLinkedList<i32, 10> = Default::default();

        assert!(list.insert_head(42).is_ok());
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_insert_head_multiple() {
        let mut list: SizedDoubleLinkedList<i32, 10> = Default::default();

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
        let mut list: SizedDoubleLinkedList<i32, 10> = Default::default();

        assert!(list.insert_tail(42).is_ok());
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_insert_tail_multiple() {
        let mut list: SizedDoubleLinkedList<i32, 10> = Default::default();

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
        let mut list: SizedDoubleLinkedList<i32, 10> = Default::default();

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
        let mut list: SizedDoubleLinkedList<i32, 10> = Default::default();

        assert!(list.insert_head(1).is_ok());
        match list.insert_after(5, 2) {
            Err(LinkedListError::IndexOutOfRange) => (),
            _ => panic!("Expected IndexOutOfRange error"),
        }
    }

    #[test]
    fn test_insert_before() {
        let mut list: SizedDoubleLinkedList<i32, 10> = Default::default();

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
        let mut list: SizedDoubleLinkedList<i32, 10> = Default::default();

        assert!(list.insert_tail(2).is_ok());
        assert!(list.insert_before(0, 1).is_ok());

        assert_eq!(list.len(), 2);
        assert_eq!(*list.get(0).unwrap(), 1);
        assert_eq!(*list.get(1).unwrap(), 2);
    }

    #[test]
    fn test_insert_before_out_of_range() {
        let mut list: SizedDoubleLinkedList<i32, 10> = Default::default();

        assert!(list.insert_head(1).is_ok());
        match list.insert_before(5, 2) {
            Err(LinkedListError::IndexOutOfRange) => (),
            _ => panic!("Expected IndexOutOfRange error"),
        }
    }

    #[test]
    fn test_get() {
        let mut list: SizedDoubleLinkedList<i32, 10> = Default::default();

        assert!(list.insert_tail(10).is_ok());
        assert!(list.insert_tail(20).is_ok());
        assert!(list.insert_tail(30).is_ok());

        assert_eq!(*list.get(0).unwrap(), 10);
        assert_eq!(*list.get(1).unwrap(), 20);
        assert_eq!(*list.get(2).unwrap(), 30);
    }

    #[test]
    fn test_get_out_of_range() {
        let mut list: SizedDoubleLinkedList<i32, 10> = Default::default();

        assert!(list.insert_tail(10).is_ok());

        match list.get(5) {
            Err(LinkedListError::IndexOutOfRange) => (),
            _ => panic!("Expected IndexOutOfRange error"),
        }
    }

    #[test]
    fn test_remove_head() {
        let mut list: SizedDoubleLinkedList<i32, 10> = Default::default();

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
        let mut list: SizedDoubleLinkedList<i32, 10> = Default::default();

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
        let mut list: SizedDoubleLinkedList<i32, 10> = Default::default();

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
        let mut list: SizedDoubleLinkedList<i32, 10> = Default::default();

        assert!(list.insert_tail(42).is_ok());
        assert!(list.remove(0).is_ok());

        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_remove_out_of_range() {
        let mut list: SizedDoubleLinkedList<i32, 10> = Default::default();

        assert!(list.insert_tail(1).is_ok());

        match list.remove(5) {
            Err(LinkedListError::IndexOutOfRange) => (),
            _ => panic!("Expected IndexOutOfRange error"),
        }
    }

    #[test]
    fn test_is_full() {
        let mut list: SizedDoubleLinkedList<i32, 3> = Default::default();

        assert!(!list.is_full());
        assert!(list.insert_tail(1).is_ok());
        assert!(!list.is_full());
        assert!(list.insert_tail(2).is_ok());
        assert!(!list.is_full());
        assert!(list.insert_tail(3).is_ok());
        assert!(list.is_full());

        match list.insert_tail(4) {
            Err(LinkedListError::ListIsFull) => (),
            _ => panic!("Expected ListIsFull error"),
        }
    }

    #[test]
    fn test_insert_remove_reuse_slots() {
        let mut list: SizedDoubleLinkedList<i32, 5> = Default::default();

        for i in 0..5 {
            assert!(list.insert_tail(i).is_ok());
        }
        assert!(list.is_full());

        assert!(list.remove(2).is_ok());
        assert!(!list.is_full());

        assert!(list.insert_tail(99).is_ok());
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn test_complex_operations() {
        let mut list: SizedDoubleLinkedList<i32, 20> = Default::default();

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
        let mut list: SizedDoubleLinkedList<i32, 20> = Default::default();

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
        let mut list: SizedDoubleLinkedList<String, 10> = Default::default();

        assert!(list.insert_tail("Hello".to_string()).is_ok());
        assert!(list.insert_tail("World".to_string()).is_ok());
        assert!(list.insert_tail("!".to_string()).is_ok());

        assert_eq!(*list.get(0).unwrap(), "Hello");
        assert_eq!(*list.get(1).unwrap(), "World");
        assert_eq!(*list.get(2).unwrap(), "!");
    }

    #[test]
    fn test_edge_case_size_limits() {
        let mut list: SizedDoubleLinkedList<i32, 1> = Default::default();

        assert!(list.insert_tail(42).is_ok());
        assert!(list.is_full());

        match list.insert_tail(43) {
            Err(LinkedListError::ListIsFull) => (),
            _ => panic!("Expected ListIsFull error"),
        }

        assert_eq!(*list.get(0).unwrap(), 42);
    }

    #[test]
    fn test_remove_all_elements() {
        let mut list: SizedDoubleLinkedList<i32, 10> = Default::default();

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
}
