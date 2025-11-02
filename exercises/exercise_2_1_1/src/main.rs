use algorithms::insertion_sort::insertion_sort;

fn main() {
    let mut arr = vec![31, 41, 59, 26, 41, 58];
    println!("Initial array: {:?}", arr);
    
    insertion_sort(&mut arr);

    println!("Sorted array: {:?}", arr);
}
