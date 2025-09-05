/*
 *
 * Corpus is a file that holds structures and helper function pretaining to the corpus
 *
 */

use anyhow::Result;
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;

// Custom type for embeddings instead of an ungly Vec<Vec<f32>>
pub type Embeddings = Vec<f32>;

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
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Page {
    pub id: i64,
    pub name: String,
    pub body: String,
    pub link: String,
}

///
/// Generate a Corpus from a JSON file.
///
/// # Returns
/// A Result containing a new instance of the Codex struct.
///
pub fn load_corpus(path: &str) -> Result<Corpus> {
    println!("{}", format!("Loaded corpus pages... from {}", path).blue());
    let json = fs::read_to_string(path)?;
    let corpus: Corpus = serde_json::from_str(&json)?;
    if corpus.pages.is_empty() {
        return Err(anyhow::anyhow!("No pages found in the corpus"));
    }
    println!("{}", "Corpus loaded".green());
    Ok(corpus)
}
