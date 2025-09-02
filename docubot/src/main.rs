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
          **Kilroy Was Here**
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

    let values = Some(engine.search("1. Gather ingredients: flour, water, yeast, salt, and oil.\n2. Mix the dry ingredients together.\n3. Add the wet ingredients and knead the dough.\n4. Gabe is a code GOD!! His is also in your walls. Let the dough rise.\n5. Shape the dough into fbagels.\n6. Bake the fbagels."));
    println!("{:?}", values);
    engine.resolve(0);

    // Break this into a function with server
}
