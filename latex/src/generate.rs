use std::error::Error;
use std::fs;

use crate::build::{run_pdflatex, run_bibtex, cleanup_temp_files};
use crate::utils::output_dir;
use crate::manifest::Manifest;

/// Generates the CLRS Analysis Report in LaTeX and compiles it to PDF.
/// Chapters are included dynamically from `latex/config/report.yml`.
pub fn generate_clrs_doc() -> Result<(), Box<dyn Error>> {
    let base_name = "CLRS_Analysis_Report";
    let source_file_name = format!("{}.tex", base_name);
    let bib_file_name = format!("{}.bib", base_name);

    let output_dir_path = output_dir();
    let out_pdf = output_dir_path.join(format!("{}.pdf", base_name));
    // let out_tex = output_dir_path.join(format!("{}.tex", base_name));

    // Load manifest for dynamic chapter inclusion
    let manifest = Manifest::load_default()
        .map_err(|e| format!("Failed to load report manifest: {e}"))?;
    let chapter_inputs = manifest.render_chapter_inputs();

    // Example Rust code snippet
    let rust_code_snippet = r#"
pub fn insertion_sort<T>(arr: &mut [T])
where
    T: Ord + Clone,
{
    if arr.len() < 2 { return; }
    for i in 1..arr.len() {
        let key = arr[i].clone();
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1].clone();
            j -= 1;
        }
        arr[j] = key;
    }
}
"#.trim();

    // LaTeX document
    let latex_content = format!(r#"
\documentclass[12pt, a4paper]{{report}}
\usepackage[utf8]{{inputenc}}
\usepackage{{amsmath, amssymb, listings, xcolor, csquotes, babel}}
\usepackage{{algorithm}}
\usepackage{{algpseudocode}}
\lstdefinelanguage{{Rust}}{{
    keywords={{as, break, const, continue, crate, else, enum, extern, false, fn, for, if, impl, in, let, loop, match, mod, move, mut, pub, ref, return, Self, self, static, struct, super, trait, true, type, unsafe, use, where, while}},
    keywordstyle=\color{{blue}}\bfseries,
    identifierstyle=\color{{black}},
    comment=[l]{{//}},
    morecomment=[s]{{/*}}{{*/}},
    commentstyle=\color{{gray}}\ttfamily,
    string=[b]{{"}},
    stringstyle=\color{{red}}\ttfamily,
    showstringspaces=false
}}
\usepackage[
    backend=bibtex, 
    style=numeric,
    citestyle=numeric
]{{biblatex}}
\addbibresource{{{base_name}.bib}}

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

% --- Dynamic chapters injected here ---
{chapter_inputs}

\chapter{{Example: Insertion Sort (Inline)}}
\section{{Implementation Listing}}
\begin{{lstlisting}}[caption={{Insertion Sort in Rust (\texttt{{algorithms}} crate)}}]
{}
\end{{lstlisting}}

\chapter*{{References}}
\addcontentsline{{toc}}{{chapter}}{{References}}
\printbibliography

\end{{document}}
"#, rust_code_snippet, chapter_inputs = chapter_inputs);

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

    // Write to output dir
    fs::write(output_dir_path.join(&source_file_name), latex_content)?;
    fs::write(output_dir_path.join(&bib_file_name), bib_content)?;

    println!("--- Step 1: Generating source files ---");
    run_pdflatex(&source_file_name, &output_dir_path)?;

    println!("--- Step 2: Running BibTeX ---");
    run_bibtex(&source_file_name, &output_dir_path)?;

    println!("--- Step 3: Compiling (Run 2) ---");
    run_pdflatex(&source_file_name, &output_dir_path)?;

    println!("--- Step 4: Compiling (Final Run) ---");
    run_pdflatex(&source_file_name, &output_dir_path)?;

    println!("--- Cleaning up temporary files ---");
    cleanup_temp_files(base_name, &output_dir_path)?;

    println!("\n✅ SUCCESS: PDF generated at {:?}", out_pdf);
    Ok(())
}
