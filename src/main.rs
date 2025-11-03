use std::env;
use std::error::Error;

// Import the algorithms crate
use algorithms::sorting::insertion_sort::insertion_sort as InsertionSort;

// Import the LaTeX generation function from your latex crate
use latex::generate_clrs_doc;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run -- <command>");
        println!("Commands: doc, insertion_sort");
        return Ok(());
    }

    match args[1].as_str() {
        "doc" => {
            println!("--- Generating LaTeX documentation ---");
            generate_clrs_doc()?;
        }
        "insertion_sort" => {
            let mut data = [5, 2, 4, 6, 1, 3];
            println!("--- Running Insertion Sort ---");
            println!("Input: {:?}", data);
            InsertionSort(&mut data);
            println!("Output: {:?}", data);
        }
        cmd => println!("Unknown command: {}", cmd),
    }

    Ok(())
}
