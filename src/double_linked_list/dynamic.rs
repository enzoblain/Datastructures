//! Double-linked list implementation with heap allocation using raw pointers.
//!
//! This module provides a generic double-linked list with unlimited capacity using heap allocation.
//! Unlike the sized variant, this list can grow indefinitely and is designed for std environments
//! where the capacity is not known at compile time.
//!
//! **Note**: This module is only available when the `no-std` feature is **not** enabled.
//! For scenarios where the maximum capacity is known and less than 64, prefer using
//! [`SizedDoubleLinkedList`](super::sized::SizedDoubleLinkedList) which offers better
//! performance through stack allocation.
//!
//! # Overview
//!
//! The `DoubleLinkedList<T>` type stores nodes individually on the heap using `Box`,
//! with head and tail pointers for O(1) insertion at both ends. This is a classic
//! pointer-based linked list implementation optimized for std environments.
//!
//! # When to use
//!
//! Use `DoubleLinkedList` when:
//! - The list size is unknown at compile time
//! - The list may need to grow beyond 63 elements
//! - You're working in a std environment
//! - You need O(1) insertions at head/tail
//!
//! Use `SizedDoubleLinkedList` when:
//! - The maximum capacity is known and ≤ 63
//! - You need no_std compatibility
//! - You want better performance through stack allocation
//!
//! # Features
//!
//! - **Unlimited capacity**: No compile-time or runtime size constraints
//! - **Heap allocation**: Each node individually allocated with Box
//! - **Bidirectional traversal**: Full support for forward and backward iteration
//! - **O(1) head/tail operations**: Efficient insertions at both ends
//!
//! # Types
//!
//! - [`DoubleLinkedList`]: The main list data structure
//!
//! # Example
//!
//! ```ignore
//! use datastructures::DoubleLinkedList::DoubleLinkedList;
//!
//! let mut list: DoubleLinkedList<i32> = Default::default();
//! list.insert_tail(42);
//! ```

use crate::LinkedListError;

use core::cmp::Ordering;
use core::ptr::NonNull;
use std::vec::Vec;

/// A double-linked list with heap-allocated nodes using raw pointers.
///
/// The list maintains head and tail pointers for O(1) insertion at both ends.
/// Each node is individually allocated on the heap using Box.
///
/// # Type Parameters
///
/// - `T`: The type of values stored in the list (must be `Sized`)
///
/// # Fields
///
/// - `head`: Pointer to the first node (if non-empty)
/// - `tail`: Pointer to the last node (if non-empty)
/// - `len`: Current number of elements in the list
pub struct DoubleLinkedList<T: Sized> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,

    len: usize,
}

/// A single node in the double-linked list.
///
/// Each node stores a value and raw pointers to the previous and next nodes.
struct Node<T> {
    value: T,

    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Box<Self> {
        Box::new(Node {
            value,
            prev: None,
            next: None,
        })
    }
}

impl<T: Clone> Clone for DoubleLinkedList<T> {
    fn clone(&self) -> Self {
        let mut new_list: Self = Default::default();

        let mut current = self.head;
        while let Some(n) = current {
            unsafe {
                let node = n.as_ref();
                new_list.insert_tail(node.value.clone()).unwrap();
                current = node.next;
            }
        }

        new_list
    }
}

impl<T: Sized> Default for DoubleLinkedList<T> {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }
}

impl<T: Sized> Drop for DoubleLinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head;
        while let Some(n) = current {
            unsafe {
                let node = Box::from_raw(n.as_ptr());
                current = node.next;
            }
        }
    }
}

impl<T: Sized> DoubleLinkedList<T> {
    /// Returns the number of elements currently in the list.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the list contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns a cloned copy of the list, preserving element order.
    pub fn copy(&self) -> Self
    where
        T: Clone,
    {
        Clone::clone(self)
    }

    /// Gets a mutable node pointer by index, traversing from the optimal end.
    fn get_node_mut(&mut self, idx: usize) -> Result<NonNull<Node<T>>, LinkedListError> {
        if idx >= self.len {
            return Err(LinkedListError::IndexOutOfRange);
        }

        unsafe {
            if idx < self.len / 2 {
                // Traverse from head
                let mut current = self.head.unwrap();
                for _ in 0..idx {
                    current = current.as_ref().next.unwrap();
                }
                Ok(current)
            } else {
                // Traverse from tail
                let mut current = self.tail.unwrap();
                for _ in 0..(self.len - 1 - idx) {
                    current = current.as_ref().prev.unwrap();
                }
                Ok(current)
            }
        }
    }

    /// Inserts a value at the end of the list.
    pub fn insert_tail(&mut self, value: T) -> Result<(), LinkedListError> {
        let n = Node::new(value);
        let new = NonNull::new(Box::into_raw(n)).unwrap();

        unsafe {
            if let Some(tail_ptr) = self.tail {
                (*new.as_ptr()).prev = Some(tail_ptr);
                (*tail_ptr.as_ptr()).next = Some(new);
                self.tail = Some(new);
            } else {
                // Empty list
                self.head = Some(new);
                self.tail = Some(new);
            }
        }

        self.len += 1;
        Ok(())
    }

    /// Inserts a value at the beginning of the list.
    pub fn insert_head(&mut self, value: T) -> Result<(), LinkedListError> {
        let n = Node::new(value);
        let new = NonNull::new(Box::into_raw(n)).unwrap();

        unsafe {
            if let Some(head_ptr) = self.head {
                (*new.as_ptr()).next = Some(head_ptr);
                (*head_ptr.as_ptr()).prev = Some(new);
                self.head = Some(new);
            } else {
                // Empty list
                self.head = Some(new);
                self.tail = Some(new);
            }
        }

        self.len += 1;
        Ok(())
    }

    /// Inserts a value after the node at the specified index.
    ///
    /// # Errors
    ///
    /// - Returns `LinkedListError::IndexOutOfRange` if `idx >= len()`
    pub fn insert_after(&mut self, idx: usize, value: T) -> Result<(), LinkedListError> {
        if idx >= self.len {
            return Err(LinkedListError::IndexOutOfRange);
        }

        let current = self.get_node_mut(idx)?;

        let n = Node::new(value);
        let new = NonNull::new(Box::into_raw(n)).unwrap();

        unsafe {
            let current_ref = current.as_ref();
            let next = current_ref.next;

            (*new.as_ptr()).prev = Some(current);
            (*new.as_ptr()).next = next;
            (*current.as_ptr()).next = Some(new);

            if let Some(nxt) = next {
                (*nxt.as_ptr()).prev = Some(new);
            } else {
                // Inserted after tail
                self.tail = Some(new);
            }
        }

        self.len += 1;
        Ok(())
    }

    /// Inserts a value before the node at the specified index.
    ///
    /// # Errors
    ///
    /// - Returns `LinkedListError::IndexOutOfRange` if `idx >= len()`
    pub fn insert_before(&mut self, idx: usize, value: T) -> Result<(), LinkedListError> {
        if idx >= self.len {
            if self.len == 0 && idx == 0 {
                return self.insert_tail(value);
            }
            return Err(LinkedListError::IndexOutOfRange);
        }

        if idx == 0 {
            return self.insert_head(value);
        }

        let current = self.get_node_mut(idx)?;

        let n = Node::new(value);
        let new = NonNull::new(Box::into_raw(n)).unwrap();

        unsafe {
            let current_ref = current.as_ref();
            let prev = current_ref.prev;

            (*new.as_ptr()).next = Some(current);
            (*new.as_ptr()).prev = prev;
            (*current.as_ptr()).prev = Some(new);

            if let Some(prv) = prev {
                (*prv.as_ptr()).next = Some(new);
            } else {
                // Inserted before head
                self.head = Some(new);
            }
        }

        self.len += 1;
        Ok(())
    }

    /// Gets a reference to the value at the specified index.
    ///
    /// # Errors
    ///
    /// - Returns `LinkedListError::IndexOutOfRange` if `idx >= len()`
    pub fn get(&self, idx: usize) -> Result<&T, LinkedListError> {
        if idx >= self.len {
            return Err(LinkedListError::IndexOutOfRange);
        }

        unsafe {
            let n = if idx < self.len / 2 {
                // Traverse from head
                let mut current = self.head.unwrap();
                for _ in 0..idx {
                    current = current.as_ref().next.unwrap();
                }
                current
            } else {
                // Traverse from tail
                let mut current = self.tail.unwrap();
                for _ in 0..(self.len - 1 - idx) {
                    current = current.as_ref().prev.unwrap();
                }
                current
            };

            Ok(&n.as_ref().value)
        }
    }

    /// Removes the node at the specified index.
    ///
    /// # Errors
    ///
    /// - Returns `LinkedListError::IndexOutOfRange` if `idx >= len()`
    pub fn remove(&mut self, idx: usize) -> Result<(), LinkedListError> {
        if idx >= self.len {
            return Err(LinkedListError::IndexOutOfRange);
        }

        let n = if idx < self.len / 2 {
            // Traverse from head
            let mut current = self.head.unwrap();
            for _ in 0..idx {
                unsafe {
                    current = current.as_ref().next.unwrap();
                }
            }
            current
        } else {
            // Traverse from tail
            let mut current = self.tail.unwrap();
            for _ in 0..(self.len - 1 - idx) {
                unsafe {
                    current = current.as_ref().prev.unwrap();
                }
            }
            current
        };

        unsafe {
            let node = n.as_ref();
            let prev = node.prev;
            let next = node.next;

            match (prev, next) {
                (Some(prv), Some(nxt)) => {
                    // Middle node
                    (*prv.as_ptr()).next = Some(nxt);
                    (*nxt.as_ptr()).prev = Some(prv);
                }
                (Some(prv), None) => {
                    // Tail node
                    (*prv.as_ptr()).next = None;
                    self.tail = Some(prv);
                }
                (None, Some(nxt)) => {
                    // Head node
                    (*nxt.as_ptr()).prev = None;
                    self.head = Some(nxt);
                }
                (None, None) => {
                    // Only node
                    self.head = None;
                    self.tail = None;
                }
            }

            // Deallocate the node
            let _ = Box::from_raw(n.as_ptr());
        }

        self.len -= 1;
        Ok(())
    }

    /// Iterates through the list and applies a function to each element's value.
    ///
    /// The function `f` receives a mutable reference to each value in order.
    pub fn iter_and_compute<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        let mut current = self.head;
        while let Some(mut n) = current {
            unsafe {
                let node = n.as_mut();
                f(&mut node.value);
                current = node.next;
            }
        }
    }

    /// Searches for a node satisfying a predicate and returns a reference to its value.
    ///
    /// Returns the first value for which `predicate` returns `true`.
    pub fn get_value_where<F>(&self, predicate: F) -> Option<&T>
    where
        F: Fn(&T) -> bool,
    {
        let mut current = self.head;
        while let Some(n) = current {
            unsafe {
                let node = n.as_ref();
                if predicate(&node.value) {
                    return Some(&node.value);
                }
                current = node.next;
            }
        }
        None
    }

    /// Searches for a node satisfying a predicate and returns its index.
    ///
    /// Returns the index of the first node for which `predicate` returns `true`.
    pub fn get_index_where<F>(&self, predicate: F) -> Option<usize>
    where
        F: Fn(&T) -> bool,
    {
        let mut current = self.head;
        let mut idx = 0;
        while let Some(n) = current {
            unsafe {
                let node = n.as_ref();
                if predicate(&node.value) {
                    return Some(idx);
                }
                current = node.next;
                idx += 1;
            }
        }
        None
    }

    /// Searches for all nodes satisfying a predicate and returns references to their values.
    ///
    /// Returns a vector of references to values for which `predicate` returns `true`.
    pub fn get_where<F>(&self, predicate: F) -> Vec<&T>
    where
        F: Fn(&T) -> bool,
    {
        let mut results = Vec::new();
        let mut current = self.head;
        while let Some(n) = current {
            unsafe {
                let node = n.as_ref();
                if predicate(&node.value) {
                    results.push(&node.value);
                }
                current = node.next;
            }
        }
        results
    }

    /// Sorts the list in place using the given comparison function.
    ///
    /// Uses merge sort (stable sort) with Vec for intermediate storage.
    pub fn sort_by<F>(&mut self, compare: F)
    where
        T: Clone,
        F: Fn(&T, &T) -> Ordering,
    {
        if self.len <= 1 {
            return;
        }

        // Collect all values into a Vec
        let mut values = Vec::with_capacity(self.len);
        let mut current = self.head;
        while let Some(n) = current {
            unsafe {
                let node = n.as_ref();
                values.push(node.value.clone());
                current = node.next;
            }
        }

        // Sort the Vec
        values.sort_by(&compare);

        // Update the list values in place
        let mut current = self.head;
        let mut idx = 0;
        while let Some(mut n) = current {
            unsafe {
                let node = n.as_mut();
                node.value = values[idx].clone();
                current = node.next;
                idx += 1;
            }
        }
    }

    /// Returns a sorted copy of the list without modifying the original.
    ///
    /// Uses the `sort_by` method on a cloned list.
    pub fn get_sorted_by<F>(&self, compare: F) -> Self
    where
        T: Clone,
        F: Fn(&T, &T) -> Ordering,
    {
        let mut cloned = self.copy();
        cloned.sort_by(compare);
        cloned
    }

    /// Returns a Vec containing cloned values of all nodes in order.
    pub fn as_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut result = Vec::with_capacity(self.len);
        let mut current = self.head;
        while let Some(n) = current {
            unsafe {
                let node = n.as_ref();
                result.push(node.value.clone());
                current = node.next;
            }
        }
        result
    }

    /// Selects the n smallest elements according to the comparison function.
    ///
    /// Returns a tuple containing:
    /// - A vector of the n smallest values (sorted)
    /// - The actual count (min(n, list length))
    ///
    /// Uses quickselect algorithm with Vec for intermediate storage.
    pub fn select_n_first_by<F>(&self, n: usize, compare: F) -> (Vec<T>, usize)
    where
        T: Clone,
        F: Fn(&T, &T) -> Ordering,
    {
        if self.len == 0 || n == 0 {
            return (Vec::new(), 0);
        }

        let actual_n = n.min(self.len);

        // Collect all values
        let mut values = Vec::with_capacity(self.len);
        let mut current = self.head;
        while let Some(n) = current {
            unsafe {
                let node = n.as_ref();
                values.push(node.value.clone());
                current = node.next;
            }
        }

        // Select n first elements
        if actual_n >= self.len {
            values.sort_by(&compare);
            return (values, self.len);
        }

        values.select_nth_unstable_by(actual_n, &compare);
        let mut result: Vec<T> = values.into_iter().take(actual_n).collect();
        result.sort_by(&compare);

        (result, actual_n)
    }
}
