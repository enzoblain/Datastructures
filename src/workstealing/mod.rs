//! Fixed-capacity work-stealing pool inspired by the Chase-Lev deque.
//!
//! Provides a bounded pool with steal/take operations for cooperative schedulers.
pub mod sized;

pub use sized::SizedWorkStealingPool;
