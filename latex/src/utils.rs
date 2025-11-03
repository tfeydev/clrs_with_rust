use std::path::{PathBuf, Path};
use std::fs;

/// Returns the workspace root (the project root where Cargo.toml lives)
pub fn workspace_root() -> PathBuf {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    Path::new(crate_dir).parent().unwrap().to_path_buf()
}

/// Returns the `/output` directory path, creating it if missing
pub fn output_dir() -> PathBuf {
    let root = workspace_root();
    let out = root.join("output");
    if !out.exists() {
        let _ = fs::create_dir_all(&out);
    }
    out
}

/// Returns the path to `/latex/chapters`
pub fn chapters_dir() -> PathBuf {
    workspace_root().join("latex/chapters")
}

/// Returns the path to `/latex/listings`
pub fn listings_dir() -> PathBuf {
    workspace_root().join("latex/listings")
}
