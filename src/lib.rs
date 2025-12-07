#![cfg_attr(feature = "no-std", no_std)]

//! Data structure implementations with optional `no_std` support.
//!
//! Provides a collection of generic data structures.
//! When the `no_std` feature is enabled, all data structures are adapted to work without
//! the standard library with compile-time size guarantees, making them suitable for embedded
//! and kernel environments.

pub mod double_linked_list;
pub use double_linked_list as DoubleLinkedList;

pub mod slice;

#[derive(Debug)]
pub enum LinkedListError {
    IndexOutOfRange,
    ListIsFull,
}

pub struct Const<const N: usize>;
