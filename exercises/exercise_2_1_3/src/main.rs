/// # Insertion Sort - Decreasing Order (Demonstration Version)
///
/// Sorts a mutable slice in-place into **monotonically decreasing order**, 
/// and prints the array state after each major iteration.
pub fn insertion_sort_decreasing_with_steps<T>(arr: &mut [T])
where
    // The T: Debug constraint is needed for printing (println!("{:?}")).
    T: PartialOrd + Copy + std::fmt::Debug, 
{
    if arr.len() < 2 {
        println!("Array is too small to sort or already sorted: {:?}", arr);
        return;
    }
    
    // Outer loop starts from the second element (index 1).
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;

        // While loop: Shift elements *smaller* than the key to the right.
        // This ensures the array is sorted in decreasing order (Largest to Smallest).
        while j > 0 && arr[j - 1] < key { // <--- Der Vergleich wurde zu '<' geändert!
            arr[j] = arr[j - 1];
            j -= 1;
        }

        // Insert the key into its correct sorted position.
        arr[j] = key;

        // --- STEP-BY-STEP OUTPUT ---
        println!("After inserting element at index {}: {:?}", i, arr);
        // ---------------------------
    }
}

fn main() {
    let mut arr = vec![31, 41, 59, 26, 41, 58];
    println!("Initial array: {:?}", arr);
    println!("--- Starting Insertion Sort (Decreasing) Steps ---");
    
    // Use the modified function here
    insertion_sort_decreasing_with_steps(&mut arr); 

    println!("--- End of Sort ---");
    println!("Sorted array: {:?}", arr);
}