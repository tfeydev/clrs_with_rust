use std::error::Error;
use std::fs;
use std::process::{Command, Stdio};
use std::path::Path;
use glob::glob;

/// Temporary file extensions created by LaTeX/BibTeX
pub const TEMP_EXTENSIONS: &[&str] = &[
    "aux", "bib", "log", "toc", "out", "bbl", "blg", "bcf", "run.xml", "fdb_latexmk", "fls",
];

/// Runs `pdflatex` in the output directory (to prevent clutter in the project root)
pub fn run_pdflatex(file: &str, work_dir: &Path) -> Result<(), Box<dyn Error>> {
    let status = Command::new("pdflatex")
        .arg("-interaction=nonstopmode")
        .arg(file)
        .current_dir(work_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        return Err(format!("pdflatex failed with exit code {:?}", status.code()).into());
    }
    Ok(())
}

/// Runs `bibtex` in the output directory
pub fn run_bibtex(file: &str, work_dir: &Path) -> Result<(), Box<dyn Error>> {
    let base_name = file.strip_suffix(".tex").unwrap_or(file);
    let status = Command::new("bibtex")
        .arg(base_name)
        .current_dir(work_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        return Err(format!("bibtex failed with exit code {:?}", status.code()).into());
    }
    Ok(())
}

/// Removes all temporary files (.aux, .log, etc.) from the given directory
pub fn cleanup_temp_files(base_name: &str, work_dir: &Path) -> Result<(), Box<dyn Error>> {
    for ext in TEMP_EXTENSIONS {
        let pattern = work_dir.join(format!("{base_name}*.{ext}"));
        for entry in glob(pattern.to_str().unwrap())? {
            if let Ok(path) = entry {
                if path.exists() {
                    let _ = fs::remove_file(path);
                }
            }
        }
    }
    Ok(())
}
