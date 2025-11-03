use std::env;
use std::fs;
use glob::glob;
use std::path::{Path};
use std::error::Error;
use std::process::{Command, Stdio};

// Algorithms-Crate
use algorithms;

use crate::algorithms::{
    sorting::insertion_sort::insertion_sort as InsertionSort,
};

// ------------------------------------------
// COMPILATION HELPERS
// ------------------------------------------
fn run_pdflatex(file: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("pdflatex")
        .arg(file)
        .arg("-interaction=nonstopmode")
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        return Err(format!("pdflatex failed with exit code: {}. Check log file.", output.status).into());
    }
    Ok(())
}

fn run_bibtex(file: &str) -> Result<(), Box<dyn Error>> {
    let base_name = file.strip_suffix(".tex").unwrap_or(file);
    let output = Command::new("bibtex")
        .arg(base_name)
        .arg("-terse")
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.contains("Fatal Error") && !stderr.contains("ERROR") {
            return Ok(());
        }
        return Err(format!("BibTeX failed with exit code: {}. Details: {}", output.status, stderr).into());
    }
    Ok(())
}

// ------------------------------------------
// LATEX GENERATION LOGIC
// ------------------------------------------
const TEMP_EXTENSIONS: &[&str] = &[
    "aux", "bib","log", "toc", "out", "bbl", "blg", "bcf", "run.xml", "fdb_latexmk", "fls"
];

fn generate_clrs_doc() -> Result<(), Box<dyn Error>> {
    let source_file_name = "CLRS_Analysis_Report.tex";
    let bib_file_name = "CLRS_Analysis_Report.bib";
    let output_dir = Path::new("output");

    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }
    
    let output_pdf_path = output_dir.join("CLRS_Analysis_Report.pdf");
    let output_tex_path = output_dir.join("CLRS_Analysis_Report.tex");
    
    let rust_code_snippet = r#"
pub fn insertion_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Copy,
{
    if arr.len() < 2 { return; }
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}
"#.trim();

    let latex_content = format!(r#"
\documentclass[12pt, a4paper]{{report}}
\usepackage[utf8]{{inputenc}}
\usepackage{{amsmath}}
\usepackage{{amssymb}}
\usepackage{{listings}}
\usepackage{{xcolor}}

\lstdefinelanguage{{Rust}}{{
    keywords={{as, break, const, continue, crate, else, enum, extern, false, fn, for, if, impl, in, let, loop, match, mod, move, mut, pub, ref, return, Self, self, static, struct, super, trait, true, type, unsafe, use, where, while}},
    keywordstyle=\color{{blue}}\bfseries,
    identifierstyle=\color{{black}},
    sensitive=true,
    comment=[l]{{//}},
    morecomment=[s]{{/*}}{{*/}},
    commentstyle=\color{{gray}}\ttfamily,
    string=[b]{{"}},
    stringstyle=\color{{red}}\ttfamily,
    showstringspaces=false
}}
\usepackage[english]{{babel}}
\usepackage[
    backend=bibtex, 
    style=numeric,
    citestyle=numeric
]{{biblatex}}
\addbibresource{{CLRS_Analysis_Report.bib}}

\title{{Comprehensive Analysis of Fundamental Algorithms in Rust}}
\author{{Thor}}
\date{{\today}}

\lstset{{
    language=Rust,
    basicstyle=\ttfamily\footnotesize,
    breaklines=true,
    captionpos=b,
    frame=single,
    showstringspaces=false
}}

\begin{{document}}
\maketitle
\tableofcontents

\begin{{abstract}}
This report documents the analysis and implementation of core algorithms from "Introduction to Algorithms" (CLRS) using Rust. Focus is on verifying theoretical complexity against practical implementation, starting with Insertion Sort.
\end{{abstract}}

\chapter{{Introduction}}
\section{{Motivation and Scope}}
The goal is to bridge theoretical analysis (CLRS) and systems-level Rust programming. Algorithms like Insertion Sort ensure correctness and crate stability.

\chapter{{Algorithm Analysis: Insertion Sort}}
\section{{Algorithm Description and Pseudocode}}
Insertion Sort maintains the sorted subarray invariant.

\subsection{{Pseudocode Listing}}
\begin{{lstlisting}}[caption={{Insertion Sort Pseudocode (CLRS)}}]
INSERTION-SORT(A)
  for j = 2 to A.length
    key = A[j]
    i = j - 1
    while i > 0 and A[i] > key
      A[i+1] = A[i]
      i = i - 1
    A[i+1] = key
\end{{lstlisting}}

\section{{Complexity Analysis}}
Worst-case occurs for reverse-sorted arrays: $T(n) = \Theta(n^2)$.

\section{{Implementation and Verification}}
\subsection{{Rust Code Listing}}
\begin{{lstlisting}}[caption={{Insertion Sort in Rust (\texttt{{algorithms}} Crate)}}]
{}
\end{{lstlisting}}

\subsection{{Execution Example}}
Running the algorithm with a small, unsorted array \{{5, 2, 4, 6, 1, 3\}} yields:
\begin{{verbatim}}
--- Running Insertion Sort ---
Input: [5, 2, 4, 6, 1, 3]
Output: [1, 2, 3, 4, 5, 6]
\end{{verbatim}}

\chapter*{{References}}
\addcontentsline{{toc}}{{chapter}}{{References}}
\printbibliography

\end{{document}}
"#, rust_code_snippet);

    let bib_content = r#"
@book{CLRS,
    author    = {Thomas H. Cormen and Charles E. Leiserson and Ronald L. Rivest and Clifford Stein},
    title     = {Introduction to Algorithms},
    edition   = {Fourth},
    publisher = {The MIT Press},
    year      = {2022},
    address   = {Cambridge, Massachusetts},
}
"#;

    fs::write(&source_file_name, latex_content)?;
    fs::write(bib_file_name, bib_content)?;

    println!("--- Step 1: Generating Source Files ---");
    run_pdflatex(source_file_name)?;
    println!("--- Step 2: Running BibTeX ---");
    run_bibtex(source_file_name)?;
    println!("--- Step 3: Compiling (Run 2) ---");
    run_pdflatex(source_file_name)?;
    println!("--- Step 4: Compiling (Final Run) ---");
    run_pdflatex(source_file_name)?;
    
    fs::rename("CLRS_Analysis_Report.pdf", &output_pdf_path)?;
    fs::rename("CLRS_Analysis_Report.tex", &output_tex_path)?;

    for ext in TEMP_EXTENSIONS {
        let temp_file = format!("CLRS_Analysis_Report*.{}", ext);
        for entry in glob(&temp_file)? {
            match entry {
                Ok(path) => fs::remove_file(path)?,
                Err(e) => eprintln!("Error deleting file: {}", e),
            }
        }
    }
    
    println!("\nSUCCESS! PDF generated: CLRS_Analysis_Report.pdf");

    Ok(())
}

// ------------------------------------------
// MAIN FUNCTION
// ------------------------------------------
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run -- <command>");
        println!("Commands: doc, insertion_sort");
        return Ok(());
    }

    match args[1].as_str() {
        "doc" => generate_clrs_doc()?,
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
