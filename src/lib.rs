#![cfg_attr(feature = "no-std", no_std)]

//! Data structure implementations with optional `no_std` support.
//!
//! Provides a collection of generic data structures.
//! When the `no_std` feature is enabled, all data structures are adapted to work without
//! the standard library with compile-time size guarantees, making them suitable for embedded
//! and kernel environments.
//!
//! # Modules
//!
//! - [`mod@double_linked_list`] - Fixed-size and unlimited capacity double-linked lists
//! - [`mod@array`] - Array manipulation and conversion utilities
//! - [`mod@option`] - Option type utilities and comparisons

/// Fixed-size and unlimited capacity double-linked list implementations.
///
/// Provides `SizedDoubleLinkedList` for compile-time bounded lists and `DoubleLinkedList`
/// for unlimited capacity. Supports bidirectional traversal, sorting, and selection operations.
pub mod double_linked_list;
pub use double_linked_list as DoubleLinkedList;

/// Array manipulation utilities for `MaybeUninit` conversions and merging operations.
///
/// Contains functions for converting between `MaybeUninit` arrays and `Option` arrays,
/// plus utilities for merging sorted arrays while keeping lowest elements.
pub mod array;

/// Option type utilities including comparison functions.
///
/// Provides comparison and manipulation functions for `Option` types.
pub mod option;

/// Errors that can occur during linked list operations.
#[derive(Debug)]
pub enum LinkedListError {
    /// The index is out of range for the current list size.
    IndexOutOfRange,
    /// The list has reached its maximum capacity.
    ListIsFull,
}

/// Const generic wrapper for compile-time integer constants.
///
/// Used to enforce compile-time capacity constraints on fixed-size data structures.
pub struct Const<const N: usize>;
