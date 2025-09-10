/*
 *
 * Author: Gabriel Tower
 * Date: 2025-02-09
 * Kilroy Was Here
 *
 * This is a Rust text-embedding based search engine for documentation. It uses BERT to generate embeddings from input text organized into a corpus.
 * Engine handles model management, search, and corpus management.
 * Model is a simplistic interface for the creation and management of the embedding models.
 * Corpus is a file that holds structures and helper functions for working with a corpus
 *
 */

mod bits;
mod consts;
mod server;
mod logg;

use colored::*;
use docueyes::corpus::load_corpus;
use docueyes::engine::Engine;
use std::env;
use std::fs;
use crate::consts::{CORPUS_PATH, EMBEDDINGS_PATH};
use crate::logg::Logg;
use crate::server::spinup_server;

fn main() -> anyhow::Result<()> {

    Logg::start_logger("docu-log")?;
    Logg::info("Logging started".to_string());
    let args: Vec<String> = env::args().collect();
    print!("{}\n", format!("{}", consts::BANNER).purple().bold());

    let corpus = load_corpus(CORPUS_PATH)?;
    let mut engine = Engine::new(corpus);

    // Based on file existence and CLI arguments handle loading and compilation of embeddings
    if args.get(1) == Some(&String::from("--recompile")) {
        engine.build_embeddings()?;
        Logg::info("Embeddings recompiling triggered".to_string());
        Logg::info("Embeddings recompiled successfully".to_string());
        Logg::info("Caching generating embeddings".to_string());
        engine.cache_embeddings(EMBEDDINGS_PATH)?;
        Logg::info("Embeddings cached successfully".to_string());
    } else {
        match fs::exists("embeddings.txt") {
            Ok(true) => {
                Logg::info("Loading embeddings from found file".to_string());
                engine.load_embeddings(EMBEDDINGS_PATH)?;
                Logg::info("Embeddings loaded successfully".to_string());
            }
            Ok(false) => {
                Logg::info("Embeddings not found, compiling embeddings".to_string());
                engine.build_embeddings()?;
                Logg::info("Embeddings compiled successfully".to_string());
                Logg::info("Caching generated embeddings".to_string());
                engine.cache_embeddings(EMBEDDINGS_PATH)?;
                Logg::info("Embeddings cached successfully".to_string());
            }
            Err(e) => {
                Logg::error(format!("{:?}", e));
            },
        }
    }

    // Run the BIT (Basic Information Tool) module
    Logg::warn("Running BIT tests".to_string());
    bits::run(&engine)?;
    Logg::warn("BIT tests all PASSED".to_string());

    Logg::warn("Entering maine".to_string());
    println!("{}", "Running".green().bold());
    let main_control_thread = spinup_server(engine);
    main_control_thread.join().unwrap();
    println!("{}", "Dead".green().bold());
    Logg::warn("Dead".to_string());
    Ok(())
}
