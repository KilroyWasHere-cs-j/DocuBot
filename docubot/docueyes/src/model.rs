/*
 *
 * Model is a simplist interface for the creation and management of the embedding models.
 *
 */
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use crate::corpus::Corpus;
use crate::corpus::Embeddings;
use anyhow::Result;
use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsModel;
use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType,
};
use std::borrow::Borrow;
use serde::__private::de::Borrowed;

///
/// This provides a nice way to work with one-off or corpus embeddings input.
///
pub enum EmbeddingInput<'a> {
    Corpus(&'a Rc<RefCell<Corpus>>),
    Text(&'a str),
}

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
    pub fn generate_embeddings(&self, embedding_input: EmbeddingInput) -> Result<Vec<Embeddings>> {
        match embedding_input {
            EmbeddingInput::Corpus(corpus) => {

                let mut page_embeddings = Vec::new();

                let rc_corpus: Rc<RefCell<Corpus>> = Rc::clone(&corpus); // clone Rc (cheap)

                for page in &rc_corpus.borrow_mut().pages {
                    let mut generated_embedding = self.model.encode(&[page.body.as_str()])?;
                    page_embeddings.push(generated_embedding.pop().unwrap());
                }
                Ok(page_embeddings)
            }
            EmbeddingInput::Text(text) => {
                let mut batch = self.model.encode(&[text])?;
                let query_embedding = batch.pop().unwrap();
                Ok(vec![query_embedding])
            }
        }
    }
}
