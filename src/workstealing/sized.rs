use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicU64, Ordering};

/// Errors returned by `SizedWorkStealingPool` operations.
#[derive(Debug, PartialEq, Eq)]
pub enum SizedWorkStealingPoolError {
    IsFull,
    IsEmpty,
}

/// Bounded work-stealing pool inspired by the Chase-Lev deque.
///
/// Provides lock-free `insert`/`take` for the owner and `steal` for workers
/// using a packed atomic state. Capacity is fixed at compile time via `N`.
pub struct SizedWorkStealingPool<T: Sized, const N: usize> {
    queue: [MaybeUninit<T>; N],
    state: AtomicU64,
}

fn pack(top: u32, bot: u32) -> u64 {
    ((top as u64) << 32) | (bot as u64)
}

fn unpack(value: u64) -> (u32, u32) {
    let top = (value >> 32) as u32;
    let bot = (value & 0xFFFF_FFFF) as u32;

    (top, bot)
}

impl<T, const N: usize> SizedWorkStealingPool<T, N> {
    /// Creates an empty pool with capacity `N`.
    pub fn new() -> Self {
        let queue: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        Self {
            queue,
            state: AtomicU64::new(0),
        }
    }

    /// Pushes a value at the bottom of the deque. Fails if the pool is full.
    pub fn insert(&mut self, value: T) -> Result<(), SizedWorkStealingPoolError>
    where
        T: Copy,
    {
        loop {
            let state_old = self.state.load(Ordering::Acquire);
            let (top, bot) = unpack(state_old);

            if bot - top == (N as u32) {
                return Err(SizedWorkStealingPoolError::IsFull);
            }

            let new_bot = bot + 1;
            let state_new = pack(top, new_bot);

            self.queue[bot as usize % N].write(value);

            match self.state.compare_exchange(
                state_old,
                state_new,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return Ok(()),
                Err(_) => {
                    continue;
                }
            }
        }
    }

    /// Steals the most recently inserted value (LIFO) from the deque. Intended for worker threads.
    pub fn steal(&self) -> Option<T>
    where
        T: Copy,
    {
        loop {
            let state_old = self.state.load(Ordering::Acquire);
            let (top, bot) = unpack(state_old);

            if top == bot {
                return None;
            }

            let new_bot = bot.checked_sub(1)?;
            let index = new_bot as usize % N;
            let value = unsafe { self.queue[index].assume_init_read() };

            let state_new = pack(top, new_bot);

            match self.state.compare_exchange(
                state_old,
                state_new,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return Some(value),
                Err(_) => continue,
            }
        }
    }

    /// Pops the oldest value (FIFO) from the deque. Intended for the owner thread.
    pub fn take(&mut self) -> Option<T>
    where
        T: Copy,
    {
        loop {
            let state_old = self.state.load(Ordering::Acquire);
            let (top, bot) = unpack(state_old);

            if top == bot {
                return None;
            }

            let index = top as usize % N;
            let value = unsafe { self.queue[index].assume_init_read() };

            let new_top = top + 1;
            let state_new = pack(new_top, bot);

            match self.state.compare_exchange(
                state_old,
                state_new,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return Some(value),
                Err(_) => continue,
            }
        }
    }
}

impl<T: Sized, const N: usize> Default for SizedWorkStealingPool<T, N> {
    fn default() -> Self {
        Self::new()
    }
}
