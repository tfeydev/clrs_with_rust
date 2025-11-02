/// # ADD-BINARY-INTEGERS
///
/// Implements the addition of two n-bit binary integers stored in 
/// arrays 'a' and 'b' and returns the (n+1)-bit sum in array 'c'.
/// 
/// The procedure processes the bits from right (LSB) to left (MSB), 
/// managing a carry bit similar to manual binary addition.
///
/// # Pseudocode (CLRS 2.1-5 Adaptation)
///
/// ```text
/// ADD-BINARY-INTEGERS(A, B, n)
/// 1  carry = 0
/// 2  for i = 1 to n 
/// 3      sum = A[n - i] + B[n - i] + carry  // Accesses indices n-1 down to 0
/// 4      C[n - i + 1] = sum mod 2           // Result bit
/// 5      carry = floor(sum / 2)             // New carry
/// 6  C[0] = carry                         // Final carry
/// 7  return C
/// ```
pub fn add_binary_integers(a: &[u8], b: &[u8]) -> Vec<u8> {
    let n = a.len();
    if n != b.len() {
        panic!("Input arrays must have the same length (n).");
    }

    // C must have n+1 elements, initialized to zero.
    let mut c = vec![0u8; n + 1];
    
    // Corresponds to Pseudocode Line 1: carry = 0
    let mut carry: u8 = 0;

    // Corresponds to Pseudocode Line 2: for i = 1 to n 
    // Rust-idiomatic approach: Iterate over indices from n-1 down to 0.
    for i in (0..n).rev() {
        // Corresponds to Pseudocode Line 3 logic: sum = A[i] + B[i] + carry
        // Rust index i corresponds to CLRS index (n - i) in the loop: 
        // e.g., i=n-1 (LSB) corresponds to CLRS i=1
        let sum = a[i] + b[i] + carry;

        // Corresponds to Pseudocode Line 4 logic: C[i + 1] = sum mod 2
        // The result bit goes into C[i+1] (or C[n - i + 1] in CLRS terms).
        c[i + 1] = sum % 2; 

        // Corresponds to Pseudocode Line 5 logic: carry = floor(sum / 2)
        carry = sum / 2;
    }

    // Corresponds to Pseudocode Line 6: C[0] = carry (The final MSB is the last carry)
    c[0] = carry;

    // Corresponds to Pseudocode Line 7: return C
    c
}

fn main() {
    // Example 1: A = 1011 (11) and B = 1101 (13). n=4
    // Expected sum: 24 (Binary 11000). C should be [1, 1, 0, 0, 0] (length n+1=5)
    let a1: Vec<u8> = vec![1, 0, 1, 1]; 
    let b1: Vec<u8> = vec![1, 1, 0, 1]; 

    let c1 = add_binary_integers(&a1, &b1);

    println!("--- Binary Addition (2.1-5) ---");
    println!("A (11): {:?}", a1);
    println!("B (13): {:?}", b1);
    println!("C (24): {:?}", c1); // Output: [1, 1, 0, 0, 0]
    
    // Example 2: No overflow, showing C[0] remains 0.
    // A2 = 0010 (2) and B2 = 0011 (3). n=4
    // Expected sum: 5 (Binary 00101). C should be [0, 0, 1, 0, 1]
    let a2: Vec<u8> = vec![0, 0, 1, 0]; 
    let b2: Vec<u8> = vec![0, 0, 1, 1]; 
    let c2 = add_binary_integers(&a2, &b2);
    
    println!("\n--- Second Test ---");
    println!("A (2): {:?}", a2);
    println!("B (3): {:?}", b2);
    println!("C (5): {:?}", c2); // Output: [0, 0, 1, 0, 1]
}