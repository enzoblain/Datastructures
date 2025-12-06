//! Double-linked list data structures.
//!
//! Provides both fixed-size and dynamic double-linked list implementations.
//! - `sized`: Fixed-size list with compile-time capacity constraints (stack allocation)
//! - `dynamic`: Dynamic list with heap allocation for unlimited capacity (std only)
//!
//! Use `SizedDoubleLinkedList` when the capacity is known and ≤ 63 for better performance.
//! Use `DoubleLinkedList` when the capacity is unknown or may exceed 63 elements.

pub mod sized;

#[cfg(not(feature = "no-std"))]
pub mod dynamic;

pub use sized::SizedDoubleLinkedList;

#[cfg(not(feature = "no-std"))]
pub use dynamic::DoubleLinkedList;
