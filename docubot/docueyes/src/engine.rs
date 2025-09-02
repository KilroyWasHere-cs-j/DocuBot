/*
 *
 * Engine handles model management, search, and corpus management.
 *
 */

use crate::corpus::Corpus;
use crate::corpus::Embeddings;
use crate::model::EmbeddingInput;
use crate::model::Model;
use anyhow::Result;

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
    pub fn search(&self, query: &str) -> Vec<String> {
        let query_embedding = self.model.generate_embeddings(EmbeddingInput::Text(query));
        // Implementation for search

        // TODO fix this I am tired right now
        // for (i, page_embedding) in self.page_embeddings.iter().enumerate() {
        //     let similarity = self.cosine_similarity(&query_embedding, page_embedding);
        //     if similarity > 0.7 {
        //         println!("Page {} is similar to the query", i);
        //     }
        // }

        unimplemented!()
    }
}
