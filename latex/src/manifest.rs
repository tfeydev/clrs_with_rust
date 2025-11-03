use serde::Deserialize;
use std::fs;
use std::path::{PathBuf};

use crate::utils::workspace_root;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub chapters: Vec<Item>,
    #[serde(default)]
    pub exercises: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub pseudocode: Option<String>,
}

impl Manifest {
    /// Load the YAML manifest from the default path: `<workspace>/latex/config/report.yml`.
    pub fn load_default() -> anyhow::Result<Self> {
        let path = default_manifest_path();
        let text = fs::read_to_string(&path)
            .map_err(|e| anyhow::anyhow!("Failed to read manifest {:?}: {}", path, e))?;
        let m: Manifest = serde_yaml::from_str(&text)
            .map_err(|e| anyhow::anyhow!("Failed to parse manifest {:?}: {}", path, e))?;
        Ok(m)
    }

    /// Generates LaTeX inclusion commands for each chapter.
    /// If a .tex file exists, it is included; otherwise pseudocode is rendered inline.
    pub fn render_chapter_inputs(&self) -> String {
        let mut result = String::new();
        for c in &self.chapters {
            let tex_path = workspace_root()
                .join("latex")
                .join("chapters")
                .join(format!("{}.tex", c.id));
            if tex_path.exists() {
                result.push_str(&format!(
                    "\\input{{../latex/chapters/{}.tex}}\n",
                    c.id
                ));
            } else if let Some(ref pseudo) = c.pseudocode {
                result.push_str(&format!(
                    "\\chapter{{{}}}\n\\section*{{Pseudocode}}\n\\begin{{verbatim}}\n{}\n\\end{{verbatim}}\n\n",
                    c.title, pseudo
                ));
            } else {
                result.push_str(&format!(
                    "\\chapter{{{}}}\n\\textit{{(No content available yet.)}}\n\n",
                    c.title
                ));
            }
        }
        result
    }
}

fn default_manifest_path() -> PathBuf {
    workspace_root().join("latex").join("config").join("report.yml")
}
