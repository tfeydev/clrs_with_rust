use std::error::Error;
use std::process::Command;
use std::path::Path;

/// Simple standalone builder (optional)
pub fn build_report() -> Result<(), Box<dyn Error>> {
    let tex_file = Path::new("latex/main.tex");

    let status = Command::new("pdflatex")
        .current_dir("latex")
        .arg("-interaction=nonstopmode")
        .arg(tex_file.file_name().unwrap())
        .status()?;

    if !status.success() {
        return Err("pdflatex failed".into());
    }

    Ok(())
}
