// /*
//  * 
//  * SUM=ARRAY(A, n)
//  * sum = 0
//  * for i = 1 to n
//  *     sum = sum + A[i]
//  * return sum
//  * 
//  */
fn sum_array(a: &[i32], n: usize) -> i32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += a[i - 1];
    }
    sum
}
 
//  /// # SUM-ARRAY (Idomaatic Rust)
//  /// using build-in iterator methods
fn sum_array_iter(a: &[i32]) -> i32 {
   a.iter().sum()
}
 
 fn main() {
     let a: Vec<i32> = vec![5, 2, 4, 6, 1, 3];
     let n = a.len();
     
     println!("Initail array A: {:?}", a);
     println!("Array length n: {}", n);
     
     // --- 1. Using the CLRS-Style Explicit Indexing Function ---
         let clrs_sum = sum_array(&a, n);
         println!("\n[1] Sum (CLRS Explicit Style): {}", clrs_sum);
         
         // --- 2. Using the Idiomatic Rust Iterator Function ---
         let idiomatic_result = sum_array_iter(&a);
         println!("[2] Sum (Idiomatic Rust Style): {}", idiomatic_result);
         
         // Note: Both methods should yield the same result: 256
}
