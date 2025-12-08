use core::cmp::Ordering;
use core::mem::MaybeUninit;

/// Merges two sorted arrays, keeping the N lowest elements.
///
/// Takes two sorted arrays `s1` and `s2`, merges them, and modifies `s1` to
/// contain the N smallest elements in sorted order. Duplicates are preserved.
///
/// # Complexity
/// - Time: O(N)
/// - Space: O(N) due to internal copy of `s1`
///
/// # Type Parameters
///
/// - `T`: Element type that must be `Ord` and `Copy`
/// - `N`: Array size (compile-time constant)
///
/// # Arguments
///
/// * `s1` - First sorted array (mutable), modified with the result
/// * `s2` - Second sorted array (consumed)
///
/// # Example
///
/// ```ignore
/// use datastructures::array::core::keep_lowest;
///
/// let mut a = [1, 3, 5, 7, 9];
/// let b = [2, 4, 6, 8, 10];
/// keep_lowest(&mut a, b);
/// assert_eq!(a, [1, 2, 3, 4, 5]);
/// ```
pub fn keep_lowest<T: Ord + Copy, const N: usize>(s1: &mut [T; N], s2: [T; N]) {
    keep_lowest_by(s1, s2, |a, b| a.cmp(b));
}

/// Merges two sorted arrays with a custom comparator, keeping the N lowest elements.
///
/// Same as `keep_lowest` but allows custom comparison logic via the `compare` function.
///
/// # Complexity
/// - Time: O(N)
/// - Space: O(N) due to internal copy of `s1`
///
/// # Type Parameters
///
/// - `T`: Element type that must be `Copy`
/// - `N`: Array size (compile-time constant)
/// - `F`: Comparator function type
///
/// # Arguments
///
/// * `s1` - First sorted array (mutable), modified with the result
/// * `s2` - Second sorted array (consumed)
/// * `compare` - Comparator function that defines the sort order
pub fn keep_lowest_by<T: Copy, const N: usize, F>(s1: &mut [T; N], s2: [T; N], compare: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let s1_copy = *s1;

    let mut i1 = 0usize;
    let mut i2 = 0usize;
    let mut k = 0usize;

    while k < N {
        let v = if i1 >= N {
            let v2 = s2[i2];
            i2 += 1;

            v2
        } else if i2 >= N {
            let v1 = s1_copy[i1];
            i1 += 1;

            v1
        } else {
            match compare(&s1_copy[i1], &s2[i2]) {
                Ordering::Less => {
                    let v1 = s1_copy[i1];
                    i1 += 1;

                    v1
                }
                Ordering::Greater => {
                    let v2 = s2[i2];
                    i2 += 1;

                    v2
                }
                Ordering::Equal => {
                    let v = s1_copy[i1];
                    i1 += 1;

                    v
                }
            }
        };

        s1[k] = v;
        k += 1;
    }
}

/// Converts a `MaybeUninit` array to an `Option` array.
///
/// Transforms an array of uninitialized values into an array of options.
/// Only the first `size` elements are converted; the rest are set to `None`.
///
/// # Important
/// The array must contain all initialized values at the beginning. The `size` parameter
/// corresponds to the number of initialized elements.
///
/// # Complexity
/// - Time: O(size)
/// - Space: O(N)
///
/// # Type Parameters
///
/// - `T`: Element type that must be `Copy`
/// - `N`: Array size (compile-time constant)
///
/// # Arguments
///
/// * `arr` - Array of `MaybeUninit<T>` with initialized values at the start
/// * `size` - Number of initialized elements in the array
///
/// # Safety
/// Assumes that the first `size` elements are properly initialized.
///
/// # Example
///
/// ```ignore
/// use core::mem::MaybeUninit;
/// use datastructures::array::core::swap_maybeuninit_to_option;
///
/// let mut arr: [MaybeUninit<i32>; 3] = [
///     MaybeUninit::new(1),
///     MaybeUninit::new(2),
///     MaybeUninit::uninit(),
/// ];
/// let result = swap_maybeuninit_to_option(arr, 2);
/// assert_eq!(result[0], Some(1));
/// assert_eq!(result[1], Some(2));
/// assert_eq!(result[2], None);
/// ```
pub fn swap_maybeuninit_to_option<T: Copy, const N: usize>(
    arr: [MaybeUninit<T>; N],
    size: usize,
) -> [Option<T>; N] {
    let mut out = [None; N];

    for (i, item) in arr.iter().enumerate().take(size) {
        let value = unsafe { item.assume_init_read() };
        out[i] = Some(value);
    }

    out
}
