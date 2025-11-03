use std::env;
use algorithms;

use crate::algorithms::{
    insertion_sort::insertion_sort as InsertionSort,
};

fn main() {
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Please provie the name of the problem to run.");
        println!("Available problems: insertion_sort");
        return;
    }
    
    let problem = &args[1];
    
    match problem.as_str() {
        "insertion_sort" => {
            let mut data = [5, 2, 4, 6, 1, 3];
            
            println!("--- Running Insertion Sort ---");
            println!("Input: {:?}", data);
            
            InsertionSort(&mut data);
            
            println!("Output: {:?}", data);
        }
        _ => {
            println!("Unknown problem: {}", problem);
        }
    }
}