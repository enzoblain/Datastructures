//! Vector utilities (std-only).
//!
//! Helpers around `Vec` for keeping the smallest elements when merging. These
//! utilities consume the second vector to avoid extra cloning and are only
//! available when the `std` feature is enabled.

pub mod core;
