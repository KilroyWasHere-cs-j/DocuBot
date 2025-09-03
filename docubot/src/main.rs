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

use anyhow::Result;
use docueyes::corpus::load_corpus;
use docueyes::engine::Engine;
use std::env;
use std::fs;

const BANNER: &str = r"
 ____   __    ___  _  _  ____   __  ____
(    \ /  \  / __)/ )( \(  _ \ /  \(_  _)
 ) D ((  O )( (__ ) \/ ( ) _ ((  O ) )(
(____/ \__/  \___)\____/(____/ \__/ (__)
          **Kilroy Was Here**
";

const TEMPERATURE: f32 = 0.5;
const MAX_RESULTS: usize = 3;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage:");
        println!("docubot <corpus_file> --recompile");
        return Ok(());
    }

    print!("{}\n", BANNER);
    println!("----------------------Starting----------------------");

    let corpus = load_corpus(args.get(1).unwrap())?;
    let mut engine = Engine::new(corpus);

    println!("\nPreparing embeddings");

    // Based on the existance and CLI arguments handle loading and compilation of embeddings
    if args.get(2) == Some(&String::from("--recompile")) {
        engine.build_embeddings()?;
        println!("Embeddings recompiling triggered");
        println!("Embeddings recompiled successfully");
        println!("Caching generated embeddings");
        engine.cache_embeddings("embeddings.txt")?;
        println!("Embeddings cached successfully");
    } else {
        match fs::exists("embeddings.txt") {
            Ok(true) => {
                println!("Loading embeddings from found file");
                engine.load_embeddings("embeddings.txt")?;
                println!("Embeddings loaded successfully");
            }
            Ok(false) => {
                println!("Embeddings not found, compiling embeddings");
                engine.build_embeddings()?;
                println!("Embeddings compiled successfully");
                println!("Caching generated embeddings");
                engine.cache_embeddings("embeddings.txt")?;
                println!("Embeddings cached successfully");
            }
            Err(e) => return Err(e.into()),
        }
    }

    println!("\n----------------------Entering main controller----------------------\n");

    let search_return = engine.search("How does Salesforce operate on the internet")?;

    // Resolve needs to be massively improved
    let resolved_pages = engine.resolve(search_return, TEMPERATURE, MAX_RESULTS);
    if resolved_pages.is_empty() {
        println!("No results found");
    } else {
        for page in resolved_pages {
            println!("{}", page.name);
            println!("{}", page.body);
            println!("{}", page.link);
        }
    }

    Ok(())
}
