/// # Insertion Sort
///
/// Sorts a mutable slice **in-place** using the *Insertion Sort* algorithm.
///
/// This implementation is intentionally direct, following the pseudocode from
/// *"Introduction to Algorithms" (CLRS)*. It uses zero-based indexing, idiomatic Rust loops,
/// and generic type constraints.
///
/// # Pseudocode (CLRS)
///
/// ```text
/// INSERTION-SORT(A)
/// for i = 2 to n
///     key = A[i]
///     j = i - 1
///     while j > 0 and A[j] > key
///         A[j + 1] = A[j]
///         j = j - 1
///     A[j + 1] = key
/// ```
///
/// # Example
///
/// ```rust
/// use clrs::insertion_sort::insertion_sort;
///
/// let mut data = [3.2, 1.1, 4.5, 2.0];
/// insertion_sort(&mut data);
/// assert_eq!(data, [1.1, 2.0, 3.2, 4.5]);
/// ```
///
/// # Notes
///
/// * The algorithm runs in **O(n²)** time.
/// * It is **stable**, meaning equal elements preserve their relative order.
/// * It performs very well on small or nearly sorted data.
pub fn insertion_sort<T>(arr: &mut [T])
where
    T: Ord + Clone,
{
    for i in 1..arr.len() {
        let key = arr[i].clone();
        let mut j = i;

        // Move elements greater than key one position ahead.
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1].clone();
            j -= 1;
        }

        arr[j] = key;
    }
}
