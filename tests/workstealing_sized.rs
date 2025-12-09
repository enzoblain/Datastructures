#[cfg(test)]
mod tests {
    use datastructures::workstealing::SizedWorkStealingPool;
    use datastructures::workstealing::sized::SizedWorkStealingPoolError;

    #[test]
    fn insert_take_fifo_for_owner() {
        let mut pool: SizedWorkStealingPool<i32, 4> = SizedWorkStealingPool::new();

        assert!(pool.insert(1).is_ok());
        assert!(pool.insert(2).is_ok());
        assert!(pool.insert(3).is_ok());

        assert_eq!(pool.take(), Some(1));
        assert_eq!(pool.take(), Some(2));
        assert_eq!(pool.take(), Some(3));
        assert_eq!(pool.take(), None);
    }

    #[test]
    fn take_reads_oldest_steal_reads_newest() {
        let mut pool: SizedWorkStealingPool<i32, 4> = SizedWorkStealingPool::new();

        assert!(pool.insert(10).is_ok());
        assert!(pool.insert(20).is_ok());
        assert!(pool.insert(30).is_ok());

        // Take is FIFO: oldest element first.
        assert_eq!(pool.take(), Some(10));
        assert_eq!(pool.take(), Some(20));

        // Steal is LIFO: grabs newest among remaining.
        assert_eq!(pool.steal(), Some(30));
        assert_eq!(pool.steal(), None);
    }

    #[test]
    fn detect_full_and_empty() {
        let mut pool: SizedWorkStealingPool<u8, 2> = SizedWorkStealingPool::new();

        assert!(pool.insert(1).is_ok());
        assert!(pool.insert(2).is_ok());
        assert!(matches!(
            pool.insert(3),
            Err(SizedWorkStealingPoolError::IsFull)
        ));

        assert!(pool.take().is_some());
        assert!(pool.take().is_some());
        assert_eq!(pool.take(), None);
        assert_eq!(pool.steal(), None);
    }
}
