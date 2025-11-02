pub fn insertion_sort_with_steps<T>(arr: &mut [T])
where
    // The T: Debug constraint is added to allow printing (println!("{:?}")).
    T: PartialOrd + Copy + std::fmt::Debug, 
{
    if arr.len() < 2 {
        println!("Array is too small to sort or already sorted: {:?}", arr);
        return;
    }
    
    // We start the loop from the second element (index 1).
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;

        // Move elements greater than key one position ahead.
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }

        // Insert the key into its correct position (j is the first position where arr[j-1] <= key).
        arr[j] = key;

        // --- ADDED STEP-BY-STEP OUTPUT ---
        // 'i' is the index of the element just inserted.
        println!("After inserting element at index {}: {:?}", i, arr);
        // ---------------------------------
    }
}

fn main() {
    let mut arr = vec![31, 41, 59, 26, 41, 58];
    println!("Initial array: {:?}", arr);
    println!("--- Starting Insertion Sort Steps ---");
    
    // Use the modified function here
    insertion_sort_with_steps(&mut arr); 

    println!("--- End of Sort ---");
    println!("Sorted array: {:?}", arr);
}