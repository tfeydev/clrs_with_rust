use std::env;
use std::fs;
use std::path::PathBuf;
use std::error::Error;
use std::process::{Command, Stdio};
// Keeping this line as confirmed working for your setup
use algorithms; 

use crate::algorithms::{
    insertion_sort::insertion_sort as InsertionSort,
};

// ------------------------------------------
// LATEX GENERATION LOGIC
// ------------------------------------------

/// Generates the LaTeX source code, writes it to a .tex file, and compiles the PDF using pdflatex.
fn generate_clrs_doc() -> Result<(), Box<dyn Error>> {
    let source_file_name = "CLRS_Analysis_Report.tex";
    let output_path = PathBuf::from(source_file_name);
    
    // Final corrected LaTeX content. Escaping the '&' character for stability.
    let latex_content = r#"
\documentclass{article}
\usepackage{amsmath}
\usepackage{amssymb}
% \usepackage{amsthm}
\title{CLRS Algorithms \\\& Analysis in Rust}
\author{Thor}
\date{}

\begin{document}
\maketitle

\section*{CLRS Chapter 2: Getting Started with Algorithms}

\subsection*{2.1 Insertion Sort}
The sorting algorithm was successfully implemented in the `algorithms` crate and its functionality was tested.
The worst-case runtime analysis provides the following complexity:

\begin{equation*}
    T(n) = \Theta(n^2)
\end{equation*}

This holds true if the input array is sorted in reverse order.

\subsection*{2.2 Analyzing Algorithms - Proof Example}

The runtime for a call to algorithm $A$ can be described by the sum of costs $c_i$ times the number of executions $t_i$:

\begin{equation}
    T(n) = \sum_{i=1}^{n} c_i t_i
\end{equation}
Here, $n$ is the size of the input array.

\end{document}
"#.to_string();

    println!("--- Step 1: Generating LaTeX Source ---");
    println!("Writing source to: {:?}", output_path);
    
    fs::write(&output_path, latex_content)?;

    // ------------------------------------------
    // STEP 2: PDF COMPILATION USING pdflatex
    // ------------------------------------------

    println!("--- Step 2: Compiling PDF using pdflatex ---");
    
    let output = Command::new("pdflatex")
        .arg(source_file_name)
        .arg("-interaction=nonstopmode") // Prevents interactive input on error
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        return Err(format!("pdflatex failed with exit code: {}. Please check the output above and the .log file for details.", output.status).into());
    }

    println!("Success! The document was compiled to CLRS_Analysis_Report.pdf");
    
    Ok(())
}

// ------------------------------------------
// MAIN FUNCTION
// ------------------------------------------

// The signature is changed to Result to handle file I/O errors cleanly.
fn main() -> Result<(), Box<dyn Error>> {
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Please provide the name of the problem or command to run.");
        println!("Available commands: doc, insertion_sort");
        return Ok(());
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "doc" => {
            // NEW COMMAND: Generates the .tex file.
            generate_clrs_doc()?; 
        }
        "insertion_sort" => {
            let mut data = [5, 2, 4, 6, 1, 3];
            
            println!("--- Running Insertion Sort ---");
            println!("Input: {:?}", data);
            
            InsertionSort(&mut data);
            
            println!("Output: {:?}", data);
        }
        _ => {
            println!("Unknown command: {}", command);
            println!("Available commands: doc, insertion_sort");
        }
    }
    
    Ok(())
}