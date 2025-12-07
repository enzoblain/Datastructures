use core::cmp::Ordering;

/// Merges two sorted arrays in-place, keeping the N lowest elements in the first array.
///
/// Takes two sorted arrays `s1` and `s2`, merges them, and modifies `s1` in-place to
/// contain the N smallest elements in sorted order. Duplicates are preserved in the output.
///
/// # Type Parameters
///
/// - `T`: Element type that must be `Ord` and `Copy`
/// - `N`: Array size (compile-time constant)
///
/// # Arguments
///
/// * `s1` - First sorted array (mutable), will be modified to contain the result
/// * `s2` - Second sorted array (consumed)
///
/// # Example
///
/// ```ignore
/// use datastructures::slice::core::keep_lowest;
///
/// let mut a = [1, 3, 5, 7, 9];
/// let b = [2, 4, 6, 8, 10];
/// keep_lowest(&mut a, b);
/// assert_eq!(a, [1, 2, 3, 4, 5]);
/// ```
pub fn keep_lowest<T: Ord + Copy, const N: usize>(s1: &mut [T; N], s2: [T; N]) {
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
            match s1_copy[i1].cmp(&s2[i2]) {
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
