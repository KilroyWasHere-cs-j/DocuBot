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
 *
 */

use docueyes::corpus::load_corpus;
use std::env;

const BANNER: &str = r"
 ____   __    ___  _  _  ____   __  ____
(    \ /  \  / __)/ )( \(  _ \ /  \(_  _)
 ) D ((  O )( (__ ) \/ ( ) _ ((  O ) )(
(____/ \__/  \___)\____/(____/ \__/ (__)
";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: docubot <corpus_file>");
        return;
    }

    print!("{}\n", BANNER);
    println!("Starting...");

    let corpus = load_corpus("documentations.json").unwrap();
    let mut engine = docueyes::engine::Engine::new(corpus);

    println!("Generating embeddings");
    if let Err(e) = engine.build_embeddings() {
        eprintln!("Embedding build failed: {}", e);
    }
    println!("Generating embeddings complete");

    println!("Entering main engine loop");

    // Break this into a function with server

    loop {
        println!("Enter a query:");
        let mut query = String::new();
        std::io::stdin().read_line(&mut query).unwrap();
        let results = engine.search(&query);
        println!("Results:");
        for result in results {
            println!("{}", result);
        }
    }
}
