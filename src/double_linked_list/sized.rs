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

use crate::array::core::swap_maybeuninit_to_option_array;
use crate::{Const, LinkedListError};

use core::cmp::{Ordering, min};
use core::mem::MaybeUninit;

#[cfg(feature = "no-std")]
use core::mem::swap;

#[cfg(not(feature = "no-std"))]
extern crate std;
#[cfg(not(feature = "no-std"))]
use std::vec::Vec;

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
    nodes: [MaybeUninit<Node<T>>; K],
    used: u64,
    len: usize,
    tail: Option<usize>,
    head: Option<usize>,
}

/// A single node in the double-linked list.
///
/// Each node stores a value and pointers to the previous and next nodes.
/// The `index` field tracks the node's position in the backing array.
#[derive(Clone, Copy)]
pub struct Node<T> {
    pub value: T,
    pub index: usize,
    pub prev: Option<usize>,
    pub next: Option<usize>,
}

/// Clones the list by iterating through nodes in order and duplicating values.
impl<T: Clone, const K: usize> Clone for SizedDoubleLinkedList<T, K>
where
    Const<K>: ValidK,
{
    fn clone(&self) -> Self {
        let mut new_list: Self = Default::default();

        let mut current = match self.head {
            Some(index) => index,
            None => return new_list,
        };

        loop {
            let node = unsafe { &*self.nodes[current].as_ptr() };
            new_list.insert_tail(node.value.clone()).unwrap();

            match node.next {
                Some(next) => current = next,
                None => break,
            }
        }

        new_list
    }
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

    /// Returns a cloned copy of the list, preserving element order.
    pub fn copy(&self) -> Self
    where
        T: Clone,
    {
        Clone::clone(self)
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
            let index = unsafe { self.nodes[only].assume_init_ref() }.index;

            self.remove_used(index);
            self.nodes[only] = MaybeUninit::uninit();
            self.head = None;
            self.tail = None;
            self.used = 0;
            self.len = 0;

            return Ok(());
        }

        if index == 0 {
            let old = self.head.unwrap();

            let (index, next_index) = {
                let n = unsafe { self.nodes[old].assume_init_ref() };
                (n.index, n.next.unwrap())
            };

            let next = unsafe { self.nodes[next_index].assume_init_mut() };
            next.prev = None;

            self.remove_used(index);
            self.nodes[old] = MaybeUninit::uninit();

            self.head = Some(next_index);
            self.len -= 1;

            return Ok(());
        }

        if index == self.len - 1 {
            let old = self.tail.unwrap();

            let (index, prev_index) = {
                let n = unsafe { self.nodes[old].assume_init_ref() };
                (n.index, n.prev.unwrap())
            };

            let prev = unsafe { self.nodes[prev_index].assume_init_mut() };
            prev.next = None;

            self.remove_used(index);
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

        let (index, prev_index, next_index) = {
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

        self.remove_used(index);
        self.nodes[current] = MaybeUninit::uninit();
        self.len -= 1;

        Ok(())
    }

    /// Iterates through all nodes in the list and applies a function to each element.
    ///
    /// This function traverses the list from head to tail, calling the provided closure
    /// for each node's value. It allows mutable access to each element during iteration.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that takes a mutable reference to each element and performs some computation
    ///
    /// # Example
    ///
    /// ```ignore
    /// use datastructures::DoubleLinkedList::SizedDoubleLinkedList;
    ///
    /// let mut list: SizedDoubleLinkedList<i32, 10> = Default::default();
    /// list.insert_tail(1);
    /// list.insert_tail(2);
    /// list.insert_tail(3);
    ///
    /// list.iter_and_compute(|val| *val *= 2);
    /// // All elements are now doubled
    /// ```
    pub fn iter_and_compute(&mut self, f: impl Fn(&mut T)) {
        let mut current = match self.head {
            Some(idx) => idx,
            None => return,
        };

        loop {
            let node = unsafe { &mut *self.nodes[current].as_mut_ptr() };
            f(&mut node.value);

            match node.next {
                Some(next_idx) => current = next_idx,
                None => break,
            }
        }
    }

    /// Traverses the list and returns the first node that matches the predicate.
    ///
    /// Starts from the head and walks forward until a value satisfies the provided
    /// predicate, returning the corresponding node. Returns `None` when no element
    /// matches.
    ///
    /// # Arguments
    ///
    /// * `f` - Predicate applied to each value to determine a match
    fn get_where(&self, f: impl Fn(&T) -> bool) -> Option<&Node<T>> {
        let mut current = self.head?;

        loop {
            let node = unsafe { &*self.nodes[current].as_ptr() };
            if f(&node.value) {
                return Some(node);
            }

            match node.next {
                Some(next_idx) => current = next_idx,
                None => break,
            }
        }

        None
    }

    /// Returns the index of the first node whose value matches the predicate.
    ///
    /// Traverses the list from head to tail and evaluates the predicate on each
    /// element, returning the index of the first match. If no element satisfies
    /// the predicate, `None` is returned.
    ///
    /// # Arguments
    ///
    /// * `f` - Predicate applied to each value to find a match
    ///
    /// # Example
    ///
    /// ```ignore
    /// use datastructures::DoubleLinkedList::SizedDoubleLinkedList;
    ///
    /// let mut list: SizedDoubleLinkedList<i32, 10> = Default::default();
    /// list.insert_tail(10);
    /// list.insert_tail(20);
    /// list.insert_tail(30);
    ///
    /// let index = list.get_index_where(|v| *v == 20);
    /// assert_eq!(index, Some(1));
    /// ```
    pub fn get_index_where(&self, f: impl Fn(&T) -> bool) -> Option<usize> {
        self.get_where(f).map(|n| n.index)
    }

    /// Returns a reference to the first value that matches the predicate.
    ///
    /// Iterates from head to tail and applies the predicate to each value,
    /// returning a reference to the first matching element. If no match is
    /// found, `None` is returned.
    ///
    /// # Arguments
    ///
    /// * `f` - Predicate used to identify a matching value
    ///
    /// # Example
    ///
    /// ```ignore
    /// use datastructures::DoubleLinkedList::SizedDoubleLinkedList;
    ///
    /// let mut list: SizedDoubleLinkedList<i32, 10> = Default::default();
    /// list.insert_tail(5);
    /// list.insert_tail(15);
    ///
    /// let value = list.get_value_where(|v| *v > 10);
    /// assert_eq!(value, Some(&15));
    /// ```
    pub fn get_value_where(&self, f: impl Fn(&T) -> bool) -> Option<&T> {
        self.get_where(f).map(|n| &n.value)
    }

    /// Sorts the list in-place using a stable merge sort and the provided comparator.
    ///
    /// The comparator should return an [`Ordering`] for two values, following the same
    /// convention as `std::cmp::Ord::cmp`. The sort is **stable**, preserving the
    /// relative order of elements that compare equal.
    ///
    /// This version uses stack-allocated buffers for `no-std` compatibility.
    ///
    /// # Arguments
    ///
    /// * `compare` - Comparator function defining the ordering between two values
    #[cfg(feature = "no-std")]
    pub fn sort_by(&mut self, mut compare: impl FnMut(&T, &T) -> Ordering) {
        if self.len <= 1 {
            return;
        }

        // Collect node indices following the current linked order into a stack-allocated buffer.
        let mut indices_buf: [MaybeUninit<usize>; K] =
            unsafe { MaybeUninit::uninit().assume_init() };
        let mut current = self.head.unwrap();

        for slot in indices_buf.iter_mut().take(self.len) {
            slot.write(current);

            let node = unsafe { &*self.nodes[current].as_ptr() };
            match node.next {
                Some(next) => current = next,
                None => break,
            }
        }

        // Secondary buffer for merges.
        let mut buffer_buf: [MaybeUninit<usize>; K] =
            unsafe { MaybeUninit::uninit().assume_init() };

        let len = self.len;

        // SAFETY: the first `len` slots are initialized above. We transmute to slices of `usize`.
        let mut src: &mut [usize] =
            unsafe { &mut *(&mut indices_buf[..len] as *mut [MaybeUninit<usize>] as *mut [usize]) };
        let mut dst: &mut [usize] =
            unsafe { &mut *(&mut buffer_buf[..len] as *mut [MaybeUninit<usize>] as *mut [usize]) };

        // Comparator on indices delegating to node values.
        let mut cmp_indices = |a: usize, b: usize| {
            let va = unsafe { &*self.nodes[a].as_ptr() };
            let vb = unsafe { &*self.nodes[b].as_ptr() };

            compare(&va.value, &vb.value)
        };

        let mut width = 1;

        while width < len {
            let mut i = 0;
            while i < len {
                let mid = (i + width).min(len);
                let end = (i + 2 * width).min(len);

                // Merge [i, mid) and [mid, end) from src into dst.
                let (mut left, mut right, mut k) = (i, mid, i);

                while left < mid && right < end {
                    if cmp_indices(src[left], src[right]) != Ordering::Greater {
                        dst[k] = src[left];

                        left += 1;
                    } else {
                        dst[k] = src[right];

                        right += 1;
                    }
                    k += 1;
                }

                while left < mid {
                    dst[k] = src[left];

                    left += 1;
                    k += 1;
                }

                while right < end {
                    dst[k] = src[right];

                    right += 1;
                    k += 1;
                }

                i += 2 * width;
            }

            width *= 2;
            swap(&mut src, &mut dst);
        }

        self.head = Some(src[0]);
        self.tail = Some(*src.last().unwrap());

        for (pos, &idx) in src.iter().enumerate() {
            let prev = if pos == 0 { None } else { Some(src[pos - 1]) };

            let next = if pos + 1 == len {
                None
            } else {
                Some(src[pos + 1])
            };

            let n = unsafe { self.nodes[idx].assume_init_mut() };

            n.prev = prev;
            n.next = next;
        }
    }

    /// Returns a sorted clone of the list using the provided comparator.
    ///
    /// The original list remains unchanged; the returned list is sorted with the
    /// same stable merge sort logic as [`sort_by`]. Requires `T: Clone` to
    /// duplicate elements into the new list without heap allocation.
    #[cfg(feature = "no-std")]
    pub fn get_sorted_by(&self, compare: impl FnMut(&T, &T) -> Ordering) -> Self
    where
        T: Clone,
    {
        let mut cloned = self.clone();

        cloned.sort_by(compare);
        cloned
    }

    /// Returns the backing nodes array as an `Option` array.
    ///
    /// Returns an array where each slot corresponding to an initialized node contains `Some(Node)`,
    /// and unused slots contain `None`. This provides access to all nodes without heap allocation,
    /// suitable for `no_std` contexts.
    ///
    /// # Requirements
    ///
    /// `T` must be `Copy` to efficiently clone node values into the array.
    pub fn as_array(&self) -> [Option<Node<T>>; K]
    where
        T: Copy,
    {
        let mut nodes_copy: [MaybeUninit<Node<T>>; K] =
            unsafe { MaybeUninit::uninit().assume_init() };

        let mut current = match self.head {
            Some(idx) => idx,
            None => return swap_maybeuninit_to_option_array(nodes_copy, 0),
        };

        loop {
            let n = unsafe { &*self.nodes[current].as_ptr() };

            let cloned = Node {
                value: n.value,
                index: n.index,
                prev: n.prev,
                next: n.next,
            };

            nodes_copy[current] = MaybeUninit::new(cloned);

            match n.next {
                Some(next) => current = next,
                None => break,
            }
        }

        swap_maybeuninit_to_option_array(nodes_copy, self.len)
    }

    /// Selects up to `N` smallest values according to the comparator using quickselect,
    /// then returns them sorted by the same comparator.
    ///
    /// The function performs an in-place quickselect on stack-allocated index buffers
    /// to partition the first `N` minimal elements (by `compare`) to the front.
    /// Returns an `Option` array where the first `min(N, self.len())` entries contain `Some(value)`,
    /// and remaining entries are `None`. Elements are sorted by the provided comparator.
    #[cfg(feature = "no-std")]
    pub fn select_n_first_by<const N: usize>(
        &self,
        mut compare: impl FnMut(&T, &T) -> Ordering,
    ) -> [Option<T>; N]
    where
        T: Copy,
    {
        let mut out: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        if self.len == 0 || N == 0 {
            return swap_maybeuninit_to_option_array(out, 0);
        }

        // Gather indices in list order.
        let mut indices_buf: [MaybeUninit<usize>; K] =
            unsafe { MaybeUninit::uninit().assume_init() };
        let mut current = self.head.unwrap();

        for slot in indices_buf.iter_mut().take(self.len) {
            slot.write(current);

            let n = unsafe { &*self.nodes[current].as_ptr() };
            match n.next {
                Some(next) => current = next,
                None => break,
            }
        }

        let len = self.len;
        let target = min(N, len);

        // SAFETY: first `len` slots initialized above.
        let indices: &mut [usize] =
            unsafe { &mut *(&mut indices_buf[..len] as *mut [MaybeUninit<usize>] as *mut [usize]) };

        let mut cmp_indices = |a: usize, b: usize| {
            let va = unsafe { &*self.nodes[a].as_ptr() };
            let vb = unsafe { &*self.nodes[b].as_ptr() };

            compare(&va.value, &vb.value)
        };

        // Hoare partition for quickselect.
        fn partition(
            arr: &mut [usize],
            left: usize,
            right: usize,
            mut cmp: impl FnMut(usize, usize) -> Ordering,
        ) -> usize {
            let pivot = arr[(left + right) / 2];
            let mut i = left;
            let mut j = right;

            loop {
                while cmp(arr[i], pivot) == Ordering::Less {
                    i += 1;
                }

                while cmp(arr[j], pivot) == Ordering::Greater {
                    if j == 0 {
                        break;
                    }

                    j -= 1;
                }

                if i >= j {
                    return j;
                }

                arr.swap(i, j);

                i += 1;

                if j == 0 {
                    return 0;
                }

                j -= 1;
            }
        }

        if len > 1 {
            let mut left = 0;
            let mut right = len - 1;
            let select_pos = target - 1;

            while left < right {
                let pivot = partition(indices, left, right, &mut cmp_indices);

                if select_pos <= pivot {
                    if pivot == 0 {
                        break;
                    }

                    right = pivot;
                } else {
                    left = pivot + 1;
                }
            }
        }

        // Sort the first `target` indices to return values in order.
        if target > 1 {
            for i in 1..target {
                let mut j = i;
                while j > 0 && cmp_indices(indices[j], indices[j - 1]) == Ordering::Less {
                    indices.swap(j, j - 1);
                    j -= 1;
                }
            }
        }

        // Copy the first `target` values (ordered) into output buffer.
        for (dst, &idx) in out.iter_mut().take(target).zip(indices.iter().take(target)) {
            let n = unsafe { &*self.nodes[idx].as_ptr() };

            dst.write(n.value);
        }

        swap_maybeuninit_to_option_array(out, target)
    }

    /// Sorts the list in-place using standard library's sort (faster than no_std version).
    ///
    /// Sorts the list in-place using the provided comparator.
    ///
    /// The comparator should return an [`Ordering`] for two values, following the same
    /// convention as `std::cmp::Ord::cmp`. The sort is **stable**, preserving the
    /// relative order of elements that compare equal.
    ///
    /// This version uses `Vec` and standard library sorting for better performance
    /// when `no-std` feature is not enabled.
    ///
    /// # Arguments
    ///
    /// * `compare` - Comparator function defining the ordering between two values
    #[cfg(not(feature = "no-std"))]
    pub fn sort_by(&mut self, mut compare: impl FnMut(&T, &T) -> Ordering) {
        if self.len <= 1 {
            return;
        }

        let mut indices = Vec::with_capacity(self.len);
        let mut current = self.head.unwrap();

        loop {
            indices.push(current);
            let node = unsafe { &*self.nodes[current].as_ptr() };
            match node.next {
                Some(next) => current = next,
                None => break,
            }
        }

        indices.sort_unstable_by(|&a, &b| {
            let va = unsafe { &*self.nodes[a].as_ptr() };
            let vb = unsafe { &*self.nodes[b].as_ptr() };
            compare(&va.value, &vb.value)
        });

        self.head = Some(indices[0]);
        self.tail = Some(*indices.last().unwrap());

        for (pos, &idx) in indices.iter().enumerate() {
            let prev = if pos == 0 {
                None
            } else {
                Some(indices[pos - 1])
            };
            let next = if pos + 1 == self.len {
                None
            } else {
                Some(indices[pos + 1])
            };
            let n = unsafe { self.nodes[idx].assume_init_mut() };
            n.prev = prev;
            n.next = next;
        }
    }

    /// Returns a sorted clone using standard library (faster than no_std version).
    ///
    /// This version is available only when the `no-std` feature is **not** enabled.
    /// Uses `sort_by` internally for optimal performance with heap allocation.
    ///
    /// # Arguments
    ///
    /// * `compare` - Comparator function defining the ordering between two values
    #[cfg(not(feature = "no-std"))]
    pub fn get_sorted_by(&self, compare: impl FnMut(&T, &T) -> Ordering) -> Self
    where
        T: Clone,
    {
        let mut cloned = self.clone();
        cloned.sort_by(compare);
        cloned
    }

    /// Selects and returns up to `N` smallest values using Vec (faster than no_std version).
    ///
    /// This version is available only when the `no-std` feature is **not** enabled.
    /// Collects indices into a `Vec`, uses `select_nth_unstable_by` for optimal performance,
    /// then sorts the selected values before returning.
    ///
    /// # Arguments
    ///
    /// * `compare` - Comparator function defining the ordering between two values
    #[cfg(not(feature = "no-std"))]
    pub fn select_n_first_by<const N: usize>(
        &self,
        mut compare: impl FnMut(&T, &T) -> Ordering,
    ) -> Vec<T>
    where
        T: Clone,
    {
        if self.len == 0 || N == 0 {
            return Vec::new();
        }

        let mut indices = Vec::with_capacity(self.len);
        let mut current = self.head.unwrap();

        loop {
            indices.push(current);
            let n = unsafe { &*self.nodes[current].as_ptr() };
            match n.next {
                Some(next) => current = next,
                None => break,
            }
        }

        let target = min(N, self.len);

        let mut cmp_indices = |&a: &usize, &b: &usize| {
            let va = unsafe { &*self.nodes[a].as_ptr() };
            let vb = unsafe { &*self.nodes[b].as_ptr() };
            compare(&va.value, &vb.value)
        };

        if target < self.len {
            indices.select_nth_unstable_by(target - 1, &mut cmp_indices);
        }

        indices.truncate(target);
        indices.sort_unstable_by(&mut cmp_indices);

        indices
            .iter()
            .map(|&idx| {
                let n = unsafe { &*self.nodes[idx].as_ptr() };
                n.value.clone()
            })
            .collect()
    }
}
