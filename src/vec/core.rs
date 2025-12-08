use core::cmp::Ordering;
use core::mem::MaybeUninit;

#[cfg(not(feature = "no-std"))]
use std::vec::Vec;

/// Merges two sorted vectors, keeping the lowest elements in place in `v1`.
///
/// Takes two sorted vectors (they may differ in length), merges them, and
/// overwrites `v1` with the `v1.len()` smallest elements in sorted order.
/// Duplicates are preserved. This is the dynamic-length counterpart of
/// `keep_lowest` for arrays.
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(n) temporary buffer
///
/// # Example
///
/// ```rust
/// use datastructures::vec::core::keep_lowest_vec;
///
/// let mut a = vec![1, 3, 5, 7, 9];
/// let b = vec![2, 4, 6, 8, 10];
/// keep_lowest_vec(&mut a, b);
/// assert_eq!(a, vec![1, 2, 3, 4, 5]);
/// ```
#[cfg(not(feature = "no-std"))]
pub fn keep_lowest_vec<T: Ord + Clone>(v1: &mut Vec<T>, v2: Vec<T>) {
    keep_lowest_vec_by(v1, v2, |a, b| a.cmp(b));
}

/// Same as [`keep_lowest_vec`] but with a custom comparator.
#[cfg(not(feature = "no-std"))]
pub fn keep_lowest_vec_by<T: Clone, F>(v1: &mut Vec<T>, v2: Vec<T>, compare: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let n = v1.len();

    let mut i1 = 0usize;
    let mut i2 = 0usize;
    let mut out: Vec<T> = Vec::with_capacity(n);

    while out.len() < n {
        let v = if i1 >= v1.len() {
            // v1 exhausted: pull from v2 if available
            let v2 = &v2[i2];
            i2 += 1;
            v2.clone()
        } else if i2 >= v2.len() {
            // v2 exhausted: pull from v1
            let v1v = &v1[i1];
            i1 += 1;
            v1v.clone()
        } else {
            match compare(&v1[i1], &v2[i2]) {
                Ordering::Less => {
                    let v1v = &v1[i1];
                    i1 += 1;
                    v1v.clone()
                }
                Ordering::Greater => {
                    let v2v = &v2[i2];
                    i2 += 1;
                    v2v.clone()
                }
                Ordering::Equal => {
                    let v1v = &v1[i1]; // prefer v1 when equal to keep stable ordering vs v2
                    i1 += 1;
                    v1v.clone()
                }
            }
        };

        out.push(v);
    }

    *v1 = out;
}

/// Converts a `MaybeUninit` slice to a `Vec<Option<T>>`.
///
/// Only the first `size` elements are converted; the rest are set to `None`.
#[cfg(not(feature = "no-std"))]
pub fn swap_maybeuninit_to_option_vec<T: Copy>(
    arr: &[MaybeUninit<T>],
    size: usize,
) -> Vec<Option<T>> {
    let mut out: Vec<Option<T>> = vec![None; arr.len()];

    for (i, item) in arr.iter().enumerate().take(size) {
        let value = unsafe { item.assume_init_read() };
        out[i] = Some(value);
    }

    out
}
