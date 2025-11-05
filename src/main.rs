use std::env;
use std::error::Error;

// Import algorithm from algorithms crate
use algorithms::sorting::insertion_sort::insertion_sort as InsertionSort;

// Import LaTeX generation pipeline
use latex::generate::generate_clrs_doc;
use latex::build::build_report;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run -- <command>");
        println!("Commands:");
        println!("  doc              Generate full LaTeX documentation (PDF)");
        println!("  insertion_sort   Run the insertion sort demo");
        return Ok(());
    }

    match args[1].as_str() {
        "doc" => {
            println!("--- Generating LaTeX documentation ---");
            generate_clrs_doc()?; // Step 1: generate chapters + copy Rust files
            build_report()?;      // Step 2: compile PDF via pdflatex
            println!("✅ Report generated successfully!");
        }

        "insertion_sort" => {
            println!("--- Running Insertion Sort ---");
            let mut data = [5, 2, 4, 6, 1, 3];
            println!("Input: {:?}", data);
            InsertionSort(&mut data);
            println!("Output: {:?}", data);
        }

        cmd => println!("Unknown command: {}", cmd),
    }

    Ok(())
}
