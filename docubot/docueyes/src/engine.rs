/*
 * Engine handles model management, search, and corpus management.
 */

use crate::corpus::Corpus;
use crate::corpus::Embeddings;
use crate::model::Model;

pub struct Engine {
    corpus: Corpus,
    model: Model,
    embeddings: Vec<Embeddings>,
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
            embeddings: Vec::new(),
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
    /// * `Error` - If there is an error generating embeddings.
    ///
    /// # Examples
    /// ```
    /// let corpus = Corpus::new();
    /// let mut engine = Engine::new(corpus);
    /// engine.build_embedding();
    /// ```
    ///
    pub fn build_embedding(&mut self) {
        println!("Generating embeddings");
        match self.model.generate_embeddings(&self.corpus) {
            Ok(embeddings) => self.embeddings = embeddings,
            Err(err) => eprintln!("Error generating embeddings: {}", err),
        }

        // Specific tests to ensure data integrity
        if self.embeddings.is_empty() {
            eprintln!("No embeddings generated");
        }
        if self.embeddings.len() != self.corpus.pages.len() {
            eprintln!("Mismatch in number of embeddings and pages");
        }
        if self.embeddings.iter().any(|e| e.is_empty()) {
            eprintln!("Empty embeddings found");
        }
        println!("Generating embeddings complete");
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
        // let query_embedding = self.model.generate_embeddings()
        // Implementation for search
        unimplemented!()
    }
}
