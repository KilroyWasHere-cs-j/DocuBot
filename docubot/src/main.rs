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

use colored::*;
use docueyes::corpus::load_corpus;
use docueyes::engine::Engine;
use std::env;
use std::fs;
use tracing::{span, Level};
use crate::consts::{CORPUS_PATH, EMBEDDINGS_PATH};
use crate::server::spinup_server;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    print!("{}\n", format!("{}", consts::BANNER).purple().bold());

    let span = span!(Level::TRACE, "DocuBotSpan");
    let _guard = span.enter();

    println!(
        "{}",
        "\n----------------------Starting----------------------\n"
            .green()
            .bold()
    );

    let corpus = load_corpus(CORPUS_PATH)?;
    let mut engine = Engine::new(corpus);

    println!("{}", "\nPreparing embeddings".yellow());

    // Based on file existence and CLI arguments handle loading and compilation of embeddings
    if args.get(1) == Some(&String::from("--recompile")) {
        engine.build_embeddings()?;
        println!("{}", "Embeddings recompiling triggered".blue());
        println!("{}", "Embeddings recompiled successfully".green());
        println!("{}", "Caching generating embeddings".blue());
        engine.cache_embeddings(EMBEDDINGS_PATH)?;
        println!("{}", "Embeddings cached successfully".green());
    } else {
        match fs::exists("embeddings.txt") {
            Ok(true) => {
                println!("{}", "Loading embeddings from found file".blue());
                engine.load_embeddings(EMBEDDINGS_PATH)?;
                println!("{}", "Embeddings loaded successfully".green());
            }
            Ok(false) => {
                println!("{}", "Embeddings not found, compiling embeddings".yellow());
                engine.build_embeddings()?;
                println!("{}", "Embeddings compiled successfully".green());
                println!("{}", "Caching generated embeddings".blue());
                engine.cache_embeddings(EMBEDDINGS_PATH)?;
                println!("{}", "Embeddings cached successfully".green());
            }
            Err(e) => return Err(e.into()),
        }
    }

    // Run the BIT (Basic Information Tool) module
    bits::run(&engine)?;

    println!(
        "{}",
        "\n----------------------Entering Main Control Loop----------------------\n"
            .green()
            .bold()
    );


    let main_control_thread = spinup_server(engine);
    main_control_thread.join().unwrap();
    Ok(())
}
