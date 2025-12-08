use core::cmp::Ordering;

/// Compares two `Option<T>` values, treating `None` as the smallest value.
///
/// This comparator is useful when you want to sort or filter options while
/// keeping `None` values at the beginning of sorted sequences. It delegates to the
/// provided `compare_t` function when both values are `Some`.
///
/// # Ordering Rules
///
/// - `(None, None)` → `Equal`
/// - `(None, Some(_))` → `Less` (None sorts before Some)
/// - `(Some(_), None)` → `Greater` (Some sorts after None)
/// - `(Some(x), Some(y))` → `compare_t(x, y)` (delegate to custom comparator)
///
/// # Type Parameters
///
/// - `T`: The wrapped type
/// - `F`: Comparator function for comparing two `T` values
///
/// # Arguments
///
/// * `a` - First option value
/// * `b` - Second option value
/// * `compare_t` - Comparator function that defines ordering between two `T` values
///
/// # Example
///
/// ```ignore
/// use datastructures::option::core::put_option_first;
///
/// let a = Some(5);
/// let b = Some(3);
/// let c: Option<i32> = None;
///
/// assert_eq!(put_option_first(&a, &b, |x, y| x.cmp(y)), std::cmp::Ordering::Greater);
/// assert_eq!(put_option_first(&c, &b, |x, y| x.cmp(y)), std::cmp::Ordering::Less);
/// assert_eq!(put_option_first(&c, &c, |x, y| x.cmp(y)), std::cmp::Ordering::Equal);
/// ```
pub fn put_option_first<T, F>(a: &Option<T>, b: &Option<T>, compare_t: F) -> Ordering
where
    F: Fn(&T, &T) -> Ordering,
{
    match (a, b) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(x), Some(y)) => compare_t(x, y),
    }
}

/// Compares two `Option<T>` values, treating `None` as the largest value.
///
/// This comparator is useful when you want to sort or filter options while
/// keeping `None` values at the end of sorted sequences. It delegates to the
/// provided `compare_t` function when both values are `Some`.
///
/// # Ordering Rules
///
/// - `(None, None)` → `Equal`
/// - `(None, Some(_))` → `Greater` (None sorts after Some)
/// - `(Some(_), None)` → `Less` (Some sorts before None)
/// - `(Some(x), Some(y))` → `compare_t(x, y)` (delegate to custom comparator)
///
/// # Type Parameters
///
/// - `T`: The wrapped type
/// - `F`: Comparator function for comparing two `T` values
///
/// # Arguments
///
/// * `a` - First option value
/// * `b` - Second option value
/// * `compare_t` - Comparator function that defines ordering between two `T` values
///
/// # Example
///
/// ```ignore
/// use datastructures::option::core::put_option_last;
///
/// let a = Some(5);
/// let b = Some(3);
/// let c: Option<i32> = None;
///
/// assert_eq!(put_option_last(&a, &b, |x, y| x.cmp(y)), std::cmp::Ordering::Greater);
/// assert_eq!(put_option_last(&b, &c, |x, y| x.cmp(y)), std::cmp::Ordering::Less);
/// assert_eq!(put_option_last(&c, &c, |x, y| x.cmp(y)), std::cmp::Ordering::Equal);
/// ```
pub fn put_option_last<T, F>(a: &Option<T>, b: &Option<T>, compare_t: F) -> Ordering
where
    F: Fn(&T, &T) -> Ordering,
{
    match (a, b) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (Some(x), Some(y)) => compare_t(x, y),
    }
}
