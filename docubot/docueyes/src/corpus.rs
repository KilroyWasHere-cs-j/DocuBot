/*
 * Corpus is a file that holds structures and helper function pretaining to the corpus
 */

use anyhow::Result;
use serde::Deserialize;
use std::fs;

// Custom type for embeddings instead of an ungly Vec<Vec<f32>>
pub type Embeddings = Vec<Vec<f32>>;

///
/// Create a new instance of the Codex struct.
///
/// # Returns
/// A Result containing a new instance of the Codex struct.
///
#[derive(Debug, Deserialize)]
pub struct Corpus {
    pub pages: Vec<Page>,
}

///
/// Create a new instance of the Page struct.
///
/// # Returns
/// A Result containing a new instance of the Page struct.
///
#[derive(Debug, Deserialize)]
pub struct Page {
    pub name: String,
    pub body: String,
}

///
/// Load the Codex from a JSON file.
///
/// # Returns
/// A Result containing a new instance of the Codex struct.
///
pub fn load_corpus(path: &str) -> Result<Corpus> {
    let json = fs::read_to_string(path)?;
    let codex: Corpus = serde_json::from_str(&json)?;
    if codex.pages.is_empty() {
        return Err(anyhow::anyhow!("No pages found in the codex"));
    }
    println!("Loaded codex pages...");
    for page in &codex.pages {
        println!("Page: {}", page.name);
    }
    Ok(codex)
}
