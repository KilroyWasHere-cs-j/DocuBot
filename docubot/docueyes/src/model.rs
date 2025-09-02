/*
 * Model is a simplist interface for the creation and management of the embedding models.
 */

use crate::corpus::Corpus;
use crate::corpus::Embeddings;
use anyhow::Result;
use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsModel;
use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType,
};

///
/// This is a nice wrapper around the SentenceEmbeddingsModel from rust_bert.
///
pub struct Model {
    model: SentenceEmbeddingsModel,
}

impl Model {
    ///
    /// Create a new instance of the Model struct.
    ///
    /// # Returns
    /// A Result containing a new instance of the Model struct.
    ///
    pub fn new() -> Self {
        let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
            .create_model()
            .expect("Failed to create model");
        Model { model }
    }

    ///
    /// Generate embeddings for a list of sentences using the SentenceEmbeddingsModel.
    ///
    /// # Arguments
    /// * `sentences` - A slice of strings representing the sentences to generate embeddings for.
    /// * `model` - A reference to the SentenceEmbeddingsModel to use for generating embeddings.
    ///
    /// # Returns
    /// A Result containing a vector of vectors of f32 representing the embeddings for each sentence.
    ///
    pub fn generate_embeddings(&self, corpus: &Corpus) -> Result<Vec<Embeddings>> {
        let mut embeddings = Vec::new();
        for page in &corpus.pages {
            // embeddings.push(self.model.encode(&[body])?);
            embeddings.push(self.model.encode(&[page.body.clone()])?);
        }
        Ok(embeddings)
    }
}
