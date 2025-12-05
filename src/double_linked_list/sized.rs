//! Fixed-size double-linked list implementation.
//!
//! This module provides a generic double-linked list with a compile-time fixed capacity constraint.
//! The list is backed by an array of uninitialized slots, allowing stack allocation without
//! runtime allocation overhead. Valid capacities range from 0 to 63, enforced via the `ValidK` trait.
//!
//! # Overview
//!
//! The `SizedDoubleLinkedList<T, K>` type stores nodes in a fixed-size array and tracks which
//! slots are in use through a bitmask (`used`). This approach combines the performance benefits
//! of array-backed storage with the flexibility of a linked structure.
//!
//! # Features
//!
//! - **Fixed capacity**: Compile-time size bounds via const generics
//! - **No heap allocation**: All data stored on the stack
//! - **Bidirectional traversal**: Full support for forward and backward iteration
//! - **Efficient indexing**: O(n) access with optimized midpoint selection
//!
//! # Types
//!
//! - [`SizedDoubleLinkedList`]: The main list data structure
//! - [`Node`]: Individual node in the list
//! - [`ValidK`]: Trait constraining valid capacity values
//!
//! # Example
//!
//! ```ignore
//! use datastructures::DoubleLinkedList::SizedDoubleLinkedList;
//! use datastructures::Const;
//!
//! let mut list: SizedDoubleLinkedList<i32, 10> = Default::default();
//! list.insert_tail(42);
//! ```

use crate::{Const, LinkedListError};

use core::mem::MaybeUninit;

/// Trait for validating capacity constants at compile time.
/// Valid capacities range from 0 to 63.
pub trait ValidK {}

macro_rules! impl_valid_k {
    ($($k:literal),*) => { $( impl ValidK for Const<$k> {} )* };
}

impl_valid_k!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63
);

/// A fixed-size, double-linked list with compile-time capacity constraints.
///
/// The list maintains nodes in a fixed array of size `K`, with a bitmask tracking
/// which slots are in use. Supports insertion, removal, and random access operations.
/// All nodes remain on the stack with no heap allocation.
///
/// # Type Parameters
///
/// - `T`: The type of values stored in the list (must be `Sized`)
/// - `K`: Compile-time capacity (0-63), enforced via the `ValidK` trait
///
/// # Fields
///
/// - `nodes`: Array of uninitialized node slots
/// - `used`: Bitmask indicating which slots contain valid nodes
/// - `len`: Current number of elements in the list
/// - `tail`: Index of the last node (if non-empty)
/// - `head`: Index of the first node (if non-empty)
pub struct SizedDoubleLinkedList<T: Sized, const K: usize>
where
    Const<K>: ValidK,
{
    pub nodes: [MaybeUninit<Node<T>>; K],
    pub used: u64,
    pub len: usize,
    pub tail: Option<usize>,
    pub head: Option<usize>,
}

/// A single node in the double-linked list.
///
/// Each node stores a value and pointers to the previous and next nodes.
/// The `index` field tracks the node's position in the backing array.
pub struct Node<T> {
    pub value: T,
    pub index: usize,
    pub prev: Option<usize>,
    pub next: Option<usize>,
}

impl<T: Sized, const K: usize> Default for SizedDoubleLinkedList<T, K>
where
    Const<K>: ValidK,
{
    fn default() -> Self {
        Self {
            nodes: unsafe { MaybeUninit::<[MaybeUninit<Node<T>>; K]>::uninit().assume_init() },
            used: 0,
            len: 0,
            tail: None,
            head: None,
        }
    }
}

impl<T: Sized, const K: usize> SizedDoubleLinkedList<T, K>
where
    Const<K>: ValidK,
{
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

    /// Returns `true` if the list has reached its maximum capacity.
    #[inline]
    pub fn is_full(&self) -> bool {
        self.len == K
    }

    /// Marks a slot as unused in the bitmask.
    #[inline]
    fn remove_used(&mut self, index: usize) {
        self.used &= !(1 << index);
    }

    /// Marks a slot as used in the bitmask.
    #[inline]
    fn add_used(&mut self, index: usize) {
        self.used |= 1 << index;
    }

    /// Finds the index of the first unused slot using bit manipulation.
    #[inline]
    fn first_free(&self) -> usize {
        (!self.used).trailing_zeros() as usize
    }

    /// Inserts a value at the end of the list.
    ///
    /// # Errors
    ///
    /// Returns `LinkedListError::ListIsFull` if the list is at capacity.
    pub fn insert_tail(&mut self, value: T) -> Result<(), LinkedListError> {
        if self.len == 0 {
            return self.insert_before(0, value);
        }

        self.insert_after(self.len - 1, value)
    }

    /// Inserts a value at the beginning of the list.
    ///
    /// # Errors
    ///
    /// Returns `LinkedListError::ListIsFull` if the list is at capacity.
    pub fn insert_head(&mut self, value: T) -> Result<(), LinkedListError> {
        self.insert_before(0, value)
    }

    /// Inserts a value after the node at the specified index.
    ///
    /// # Errors
    ///
    /// - Returns `LinkedListError::IndexOutOfRange` if `index >= len()`
    /// - Returns `LinkedListError::ListIsFull` if the list is at capacity
    ///
    /// # Note
    ///
    /// For optimal performance, the function starts traversal from the closer end (head or tail)
    /// by comparing the index position with the midpoint of the list.
    pub fn insert_after(&mut self, index: usize, value: T) -> Result<(), LinkedListError> {
        if index >= self.len {
            return Err(LinkedListError::IndexOutOfRange);
        }

        if self.is_full() {
            return Err(LinkedListError::ListIsFull);
        }

        let (mut current, steps, forward) = if index < self.len / 2 {
            (self.head.unwrap(), index, true)
        } else {
            (self.tail.unwrap(), self.len - 1 - index, false)
        };

        for _ in 0..steps {
            let node = unsafe { self.nodes[current].assume_init_ref() };

            current = if forward {
                node.next.unwrap()
            } else {
                node.prev.unwrap()
            };
        }

        let new = self.first_free();
        let after = current;

        let after_next = {
            let next = unsafe { self.nodes[after].assume_init_ref() };
            next.next
        };

        if let Some(n) = after_next {
            unsafe { self.nodes[n].assume_init_mut() }.prev = Some(new);
        } else {
            self.tail = Some(new);
        }

        let new_node = Node {
            value,
            index: new,
            prev: Some(after),
            next: after_next,
        };

        let next = unsafe { self.nodes[after].assume_init_mut() };
        next.next = Some(new);

        self.add_used(new);
        self.nodes[new] = MaybeUninit::new(new_node);
        self.len += 1;

        Ok(())
    }

    /// Inserts a value before the node at the specified index.
    ///
    /// # Errors
    ///
    /// - Returns `LinkedListError::IndexOutOfRange` if `index > len()`
    /// - Returns `LinkedListError::ListIsFull` if the list is at capacity
    ///
    /// # Note
    ///
    /// For optimal performance, the function starts traversal from the closer end (head or tail)
    /// by comparing the index position with the midpoint of the list.
    pub fn insert_before(&mut self, index: usize, value: T) -> Result<(), LinkedListError> {
        if index > self.len {
            return Err(LinkedListError::IndexOutOfRange);
        }

        if self.is_full() {
            return Err(LinkedListError::ListIsFull);
        }

        if index == 0 {
            if self.len == 0 {
                let new = self.first_free();

                let node = Node {
                    value,
                    index: new,
                    prev: None,
                    next: None,
                };

                self.add_used(new);
                self.nodes[new] = MaybeUninit::new(node);
                self.head = Some(new);
                self.tail = Some(new);
                self.len = 1;

                return Ok(());
            }

            let old = self.head.unwrap();
            let new = self.first_free();

            let prev = unsafe { self.nodes[old].assume_init_mut() };
            prev.prev = Some(new);

            let node = Node {
                value,
                index: new,
                prev: None,
                next: Some(old),
            };

            self.add_used(new);
            self.nodes[new] = MaybeUninit::new(node);
            self.head = Some(new);
            self.len += 1;

            return Ok(());
        }

        let (mut current, steps, forward) = if index < self.len / 2 {
            (self.head.unwrap(), index, true)
        } else {
            (self.tail.unwrap(), self.len - 1 - index, false)
        };

        for _ in 0..steps {
            let n = unsafe { self.nodes[current].assume_init_ref() };

            current = if forward {
                n.next.unwrap()
            } else {
                n.prev.unwrap()
            };
        }

        let before = current;
        let old_prev = unsafe { self.nodes[before].assume_init_ref().prev };
        let new = self.first_free();

        if let Some(p) = old_prev {
            unsafe { self.nodes[p].assume_init_mut() }.next = Some(new);
        } else {
            self.head = Some(new);
        }

        let new_node = Node {
            value,
            index: new,
            prev: old_prev,
            next: Some(before),
        };

        let before = unsafe { self.nodes[before].assume_init_mut() };
        before.prev = Some(new);

        self.add_used(new);
        self.nodes[new] = MaybeUninit::new(new_node);
        self.len += 1;

        Ok(())
    }

    /// Returns a reference to the value at the specified index.
    ///
    /// # Errors
    ///
    /// Returns `LinkedListError::IndexOutOfRange` if `index >= len()`
    ///
    /// # Note
    ///
    /// Performs traversal from the closer end for efficiency.
    pub fn get(&self, index: usize) -> Result<&T, LinkedListError> {
        if index >= self.len {
            return Err(LinkedListError::IndexOutOfRange);
        }

        let (mut current, steps, forward) = if index < self.len / 2 {
            (self.head.unwrap(), index, true)
        } else {
            (self.tail.unwrap(), self.len - 1 - index, false)
        };

        for _ in 0..steps {
            let n = unsafe { self.nodes[current].assume_init_ref() };

            current = if forward {
                n.next.unwrap()
            } else {
                n.prev.unwrap()
            };
        }

        let value = unsafe { &self.nodes[current].assume_init_ref().value };

        Ok(value)
    }

    /// Removes the node at the specified index from the list.
    ///
    /// # Errors
    ///
    /// Returns `LinkedListError::IndexOutOfRange` if `index >= len()`
    ///
    /// # Note
    ///
    /// Performs traversal from the closer end for efficiency. Special-case handling for
    /// removing the head, tail, or only node in the list.
    pub fn remove(&mut self, index: usize) -> Result<(), LinkedListError> {
        if index >= self.len {
            return Err(LinkedListError::IndexOutOfRange);
        }

        if self.len == 1 {
            let only = self.head.unwrap();
            let idx = unsafe { self.nodes[only].assume_init_ref() }.index;

            self.remove_used(idx);
            self.nodes[only] = MaybeUninit::uninit();
            self.head = None;
            self.tail = None;
            self.used = 0;
            self.len = 0;

            return Ok(());
        }

        if index == 0 {
            let old = self.head.unwrap();

            let (idx, next_index) = {
                let n = unsafe { self.nodes[old].assume_init_ref() };
                (n.index, n.next.unwrap())
            };

            let next = unsafe { self.nodes[next_index].assume_init_mut() };
            next.prev = None;

            self.remove_used(idx);
            self.nodes[old] = MaybeUninit::uninit();

            self.head = Some(next_index);
            self.len -= 1;

            return Ok(());
        }

        if index == self.len - 1 {
            let old = self.tail.unwrap();

            let (idx, prev_index) = {
                let n = unsafe { self.nodes[old].assume_init_ref() };
                (n.index, n.prev.unwrap())
            };

            let prev = unsafe { self.nodes[prev_index].assume_init_mut() };
            prev.next = None;

            self.remove_used(idx);
            self.nodes[old] = MaybeUninit::uninit();

            self.tail = Some(prev_index);
            self.len -= 1;

            return Ok(());
        }

        let (mut current, steps, forward) = if index < self.len / 2 {
            (self.head.unwrap(), index, true)
        } else {
            (self.tail.unwrap(), self.len - 1 - index, false)
        };

        for _ in 0..steps {
            let node = unsafe { self.nodes[current].assume_init_ref() };

            current = if forward {
                node.next.unwrap()
            } else {
                node.prev.unwrap()
            };
        }

        let (idx, prev_index, next_index) = {
            let n = unsafe { self.nodes[current].assume_init_ref() };
            (n.index, n.prev, n.next)
        };

        if let Some(p) = prev_index {
            unsafe { self.nodes[p].assume_init_mut() }.next = next_index;
        } else {
            self.head = next_index;
        }

        if let Some(n) = next_index {
            unsafe { self.nodes[n].assume_init_mut() }.prev = prev_index;
        } else {
            self.tail = prev_index;
        }

        self.remove_used(idx);
        self.nodes[current] = MaybeUninit::uninit();
        self.len -= 1;

        Ok(())
    }
}
