/*
 *
 * Engine handles model management, search, and corpus management.
 *
 */

use crate::corpus::Corpus;
use crate::corpus::Embeddings;
use crate::corpus::Page;
use crate::model::EmbeddingInput;
use crate::model::Model;
use anyhow::Result;
use std::fs::File;

///
/// ResolveLevel enum defines the level/degree of resolution for similarity calculations.
///
/// # Variants
/// * `First` - First element
/// * `Mid` - Middle element
/// * `Last` - Last element
/// * `To` - For first element advance towards last element `n` steps
///
pub enum ResolveLevel {
    First,
    Mid,
    Last,
    To,
}

///
/// Engine struct represents the engine that handles model management, search, and corpus management.
///
/// # Fields
/// * `corpus` - The corpus to generate embeddings from
/// * `model` - The model used in the embedding process
/// * `page_embeddings` - The generated embeddings
///
pub struct Engine {
    corpus: Corpus,
    model: Model,
    page_embeddings: Vec<Embeddings>,
}

impl Engine {
    ///
    /// Create a new engine with the given corpus.
    ///
    /// # Arguments
    /// * `corpus` - The corpus to generate embeddings for.
    ///
    /// # Returns
    /// * `Engine` - The created engine.
    ///
    /// # Errors
    /// * `Error` - If there is an error generating embeddings.
    ///
    /// # Examples
    /// ```
    /// let corpus = Corpus::new();
    /// let mut engine = Engine::new(corpus);
    /// engine.build_embedding();
    /// ```
    ///
    pub fn new(corpus: Corpus) -> Self {
        Engine {
            corpus: corpus,
            model: Model::new(),
            page_embeddings: Vec::new(),
        }
    }

    ///
    /// Generate embeddings for the corpus using the model.
    ///
    /// # Arguments
    /// * `corpus` - The corpus to generate embeddings for.
    ///
    /// # Returns
    /// * `Vec<Embeddings>` - The generated embeddings.
    ///
    /// # Errors
    /// * `Error` - All errors are handled internally.
    ///
    /// # Examples
    /// ```
    /// let corpus = Corpus::new();
    /// let mut engine = Engine::new(corpus);
    /// engine.build_embedding();
    /// ```
    ///
    pub fn build_embeddings(&mut self) -> Result<()> {
        let embeddings = self
            .model
            .generate_embeddings(EmbeddingInput::Corpus(&self.corpus))
            .unwrap();

        self.page_embeddings = embeddings;
        Ok(())
    }

    ///
    /// Writes text embeddings to a file.
    ///
    /// # Arguments
    /// * `path` - The path to the file.
    ///
    /// # Returns
    /// * `Result<()>` - The result of the operation.
    ///
    pub fn cache_embeddings(&mut self, path: &str) -> Result<()> {
        let mut file = File::create(path)?;
        serde_json::to_writer(&mut file, &self.page_embeddings)?;
        Ok(())
    }

    ///
    /// Reads text embeddings from a file.
    ///
    /// # Arguments
    /// * `path` - The path to the file.
    ///
    /// # Returns
    /// * `Result<()>` - The result of the operation.
    ///
    pub fn load_embeddings(&mut self, path: &str) -> Result<()> {
        let mut file = File::open(path)?;
        self.page_embeddings = serde_json::from_reader(&mut file)?;
        Ok(())
    }

    ///
    /// Helper function to clears engines stored text embeddings.
    ///
    /// # Returns
    /// * `Result<()>` - The result of the operation.
    ///
    pub fn clear_embeddings(&mut self) -> Result<()> {
        self.page_embeddings.clear();
        Ok(())
    }

    /// Calculate the cosine similarity between two `f32` vectors.
    ///
    /// # Arguments
    /// * `a` - The first vector.
    /// * `b` - The second vector.
    ///
    /// # Returns
    /// * `f32` - The cosine similarity between the two vectors.
    ///
    fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f32 {
        assert_eq!(a.len(), b.len(), "Vectors must have the same length");

        let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            0.0 // or handle divide-by-zero as you see fit
        } else {
            dot / (norm_a * norm_b)
        }
    }

    ///
    /// Search for pages similar to the query using the embeddings.
    ///
    /// # Arguments
    /// * `query` - The query to search for.
    ///
    /// # Returns
    /// * `Vec<String>` - The pages similar to the query.
    ///
    pub fn search(&self, query: &str) -> Result<Vec<f32>> {
        let query_embedding = self
            .model
            .generate_embeddings(EmbeddingInput::Text(query))?;
        // Implementation for search
        // TODO fix nothing I'm a GOD

        let mut similarities = Vec::new();
        for (i, page_embedding) in self.page_embeddings.iter().enumerate() {
            let similarity = self.cosine_similarity(&query_embedding[0], page_embedding);
            similarities.push(similarity);
        }
        // similarities.sort_by(|b, a| a.partial_cmp(b).unwrap());
        Ok(similarities)
    }

    ///
    /// Resolve the similarity set to a single value based on the resolve level.
    ///
    /// # Arguments
    /// * `resolve_level` - The level of resolution.
    /// * `set` - The similarity set.
    ///
    /// # Returns
    /// * `Option<f32>` - The resolved similarity value.
    ///
    pub fn resolve(&self, set: Vec<f32>, temperature: f32, window_size: usize) -> Vec<&Page> {
        let mut resolved_page = Vec::new();
        let mut index = 0;

        // add in a check for all dissimilariters
        for similarity in set {
            if similarity >= temperature {
                resolved_page.push(self.corpus.pages.get(index).unwrap());
            }
            if index >= window_size {
                break;
            }
            index += 1;
        }
        resolved_page
    }
}
