/*
 *
 * Author: Gabriel Tower
 * Date: 2023-04-01
 * Kilroy Was Here
 *
 * This is a Rust text-embedding based search engine for documentation. It uses BERT to generate embeddings from input text organized into a corpus.
 * Engine handles model management, search, and corpus management.
 * Model is a simplist interface for the creation and management of the embedding models.
 * Corpus is a file that holds structures and helper function pretaining to the corpus
 */

use docueyes::corpus::{self, load_corpus};
use std::env;

const BANNER: &str = r"
 ____   __    ___  _  _  ____   __  ____
(    \ /  \  / __)/ )( \(  _ \ /  \(_  _)
 ) D ((  O )( (__ ) \/ ( ) _ ((  O ) )(
(____/ \__/  \___)\____/(____/ \__/ (__)
";

fn main() {
    let args: Vec<String> = env::args().collect();

    print!("{}\n", BANNER);
    println!("Starting...");
    let corpus = load_corpus("documentations.json").unwrap();
    let mut engine = docueyes::engine::Engine::new(corpus);
    engine.build_embedding();
}
