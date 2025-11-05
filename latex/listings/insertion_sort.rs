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
