use clrs::insertion_sort::insertion_sort;

/// Tests the insertion sort algorithm with different types of input data.
///
/// The tests verify correctness for integers, floating-point numbers,
/// and handle special cases like empty and single-element arrays.

#[test]
fn sorts_integers() {
    // Basic integer test: unsorted array → sorted array
    let mut data = [5, 2, 4, 6, 1, 3];
    insertion_sort(&mut data);
    assert_eq!(data, [1, 2, 3, 4, 5, 6]);
}

#[test]
fn handles_empty_and_single_element() {
    // Empty array should remain unchanged
    let mut empty: [i32; 0] = [];
    insertion_sort(&mut empty);
    assert_eq!(empty, []);

    // Single-element array should remain unchanged
    let mut single = [42];
    insertion_sort(&mut single);
    assert_eq!(single, [42]);
}

#[test]
fn sorts_floats() {
    // Floating-point test: verifies that generic types also work
    let mut data = [3.2, 1.1, 4.5, 2.0];
    insertion_sort(&mut data);
    assert_eq!(data, [1.1, 2.0, 3.2, 4.5]);
}
