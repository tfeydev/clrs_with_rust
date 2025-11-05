use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;
use tempfile::TempDir;
use walkdir::WalkDir;

/// Struct representing the YAML report file
#[derive(Debug, Deserialize)]
struct Report {
    chapters: Vec<Chapter>,
}

/// Single algorithm entry in the YAML
#[derive(Debug, Deserialize)]
struct Chapter {
    id: String,
    title: String,
    pseudocode: String,
}

/// Public entry point for generating LaTeX chapters and building the PDF.
pub fn generate_clrs_doc() -> Result<(), Box<dyn Error>> {
    // Base directories
    let base_dir = Path::new("latex");
    let config_file = base_dir.join("config/report.yml");
    let listings_dir = base_dir.join("listings");
    let generated_dir = base_dir.join("generated");
    let algorithms_dir = Path::new("algorithms/src");

    // Ensure directories exist
    fs::create_dir_all(&listings_dir)?;
    fs::create_dir_all(&generated_dir)?;

    // Load and parse YAML config
    let yaml_str = fs::read_to_string(&config_file)?;
    let report: Report = serde_yaml::from_str(&yaml_str)?;

    // Clean previous generated files
    for entry in fs::read_dir(&generated_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            fs::remove_file(entry.path())?;
        }
    }

    // Create chapters.tex
    let chapters_file = generated_dir.join("chapters.tex");
    let mut tex_out = File::create(&chapters_file)?;

    // Process all chapters
    for chapter in &report.chapters {
        let id = &chapter.id;
        let title = &chapter.title;
        let pseudocode = chapter.pseudocode.trim_end();

        // Locate Rust source file
        let rust_file_path = find_rust_file(&algorithms_dir, id)?;
        let target_file = listings_dir.join(format!("{}.rs", id));

        // Copy + sanitize Rust file (filter out doc comments)
        let file = File::open(&rust_file_path)?;
        let reader = BufReader::new(file);
        let mut clean_content = String::new();

        for line in reader.lines() {
            let line = line?;
            if !line.trim_start().starts_with("///") {
                clean_content.push_str(&line);
                clean_content.push('\n');
            }
        }

        let replacements = [
            ("²", "^2"),
            ("•", "\\\\textbullet{}"),
            ("–", "-"),
            ("—", "--"),
            ("…", "..."),
            ("“", "\""),
            ("”", "\""),
            ("’", "'"),
            ("©", "(c)"),
            ("\r\n", "\n"),
        ];
        for (from, to) in &replacements {
            clean_content = clean_content.replace(from, to);
        }
        fs::write(&target_file, clean_content.as_bytes())?;

        // Write pseudocode file (with escaping)
        let pseudo_file = generated_dir.join(format!("{}_pseudo.tex", id));
        println!("--- writing pseudocode to {:?}", pseudo_file);
        let sanitized = pseudocode
            .replace('%', "\\%")
            .replace('$', "\\$")
            .replace('#', "\\#")
            .replace('&', "\\&")
            .replace('_', "\\_")
            .replace('{', "\\{")
            .replace('}', "\\}");
        fs::write(&pseudo_file, sanitized)?;
        println!("✅ written: {}", pseudo_file.exists());

        // Write chapter entry
        writeln!(tex_out, "\\AlgorithmSection{{{}}}{{{}}}", id, title)?;
    }

    // Run LaTeX (2 passes) inside a temporary build directory
    let build_dir = TempDir::new()?;
    let main_tex = base_dir.join("main.tex");
    run_pdflatex(&main_tex, build_dir.path())?;

    // Move artefacts into latex/output and keep workspace clean
    finalize_output(base_dir, build_dir.path())?;

    // Explicitly close to surface potential filesystem errors
    build_dir.close()?;

    Ok(())
}

/// Search recursively for .rs file matching the algorithm ID
fn find_rust_file(base_dir: &Path, id: &str) -> Result<PathBuf, Box<dyn Error>> {
    for entry in WalkDir::new(base_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_name().to_str() == Some(&format!("{}.rs", id)) {
            return Ok(entry.path().to_path_buf());
        }
    }
    Err(format!("Rust source file for '{}' not found in {:?}", id, base_dir).into())
}

/// Run pdflatex with full output into the given build directory
fn run_pdflatex(tex_path: &Path, output_dir: &Path) -> Result<(), Box<dyn Error>> {
    let work_dir = tex_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| Path::new(".").to_path_buf());
    let file_name = tex_path
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("invalid tex file name")?;
    fs::create_dir_all(output_dir)?;

    for pass in 1..=2 {
        println!("--- pdflatex pass {} ---", pass);
        let output = Command::new("pdflatex")
            .current_dir(&work_dir)
            .arg("-interaction=nonstopmode")
            .arg("-halt-on-error")
            .arg("-output-directory")
            .arg(output_dir)
            .arg(file_name)
            .output()?;
        if !output.status.success() {
            println!(
                "--- pdflatex stdout ---\n{}",
                String::from_utf8_lossy(&output.stdout)
            );
            println!(
                "--- pdflatex stderr ---\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
            return Err(format!("pdflatex failed on pass {} (cwd={:?})", pass, work_dir).into());
        }
    }

    println!("✅ PDF successfully built in {:?}", work_dir);
    Ok(())
}

/// Copy final artefacts to latex/output while leaving the workspace clean.
fn finalize_output(base_dir: &Path, build_dir: &Path) -> Result<(), Box<dyn Error>> {
    let output_dir = base_dir.join("output");
    fs::create_dir_all(&output_dir)?;

    let pdf_src = build_dir.join("main.pdf");
    if !pdf_src.exists() {
        return Err(format!("expected PDF at {:?}, but it was not generated", pdf_src).into());
    }
    let pdf_dst = output_dir.join("main.pdf");
    fs::copy(&pdf_src, &pdf_dst)?;
    let workspace_pdf = base_dir.join("main.pdf");
    if workspace_pdf.exists() {
        fs::remove_file(&workspace_pdf)?;
    }

    let tex_src = base_dir.join("main.tex");
    if tex_src.exists() {
        let tex_dst = output_dir.join("main.tex");
        fs::copy(&tex_src, &tex_dst)?;
    }

    println!(
        "🧹 Temporary build directory cleaned. Output available in {:?}",
        output_dir
    );
    Ok(())
}
