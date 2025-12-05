//! Double-linked list data structures.
//!
//! Provides a fixed-size, double-linked list implementation with compile-time capacity constraints.
//! This approach is optimized for scenarios where the maximum capacity is known at compile time,
//! enabling stack allocation without runtime overhead.

pub mod sized;

pub use sized::SizedDoubleLinkedList;
