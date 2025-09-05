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

use colored::*;
use docueyes::corpus::load_corpus;
use docueyes::engine::Engine;
use std::env;
use std::fs;
use tiny_http::{Response, Server};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage:");
        println!("docubot <corpus_file> --recompile");
        return Ok(());
    }

    print!("{}\n", format!("{}", consts::BANNER).purple().bold());
    println!("----------------------Starting----------------------");

    let corpus = load_corpus(args.get(1).unwrap()).unwrap();
    let mut engine = Engine::new(corpus);

    println!("{}", "\nPreparing embeddings".yellow());

    // Based on file existence and CLI arguments handle loading and compilation of embeddings
    if args.get(2) == Some(&String::from("--recompile")) {
        engine.build_embeddings()?;
        println!("{}", "Embeddings recompiling triggered".blue());
        println!("{}", "Embeddings recompiled successfully".green());
        println!("{}", "Caching generating embeddings".blue());
        engine.cache_embeddings("embeddings.txt")?;
        println!("{}", "Embeddings cached successfully".green());
    } else {
        match fs::exists("embeddings.txt") {
            Ok(true) => {
                println!("{}", "Loading embeddings from found file".blue());
                engine.load_embeddings("embeddings.txt")?;
                println!("{}", "Embeddings loaded successfully".green());
            }
            Ok(false) => {
                println!("{}", "Embeddings not found, compiling embeddings".yellow());
                engine.build_embeddings()?;
                println!("{}", "Embeddings compiled successfully".green());
                println!("{}", "Caching generated embeddings".blue());
                engine.cache_embeddings("embeddings.txt")?;
                println!("{}", "Embeddings cached successfully".green());
            }
            Err(e) => return Err(e.into()),
        }
    }

    // Run the BIT (Basic Information Tool) module
    bits::run(&engine).unwrap();

    println!(
        "{}",
        "\n----------------------Entering Main Control Loop----------------------\n"
            .green()
            .bold()
    );

    let main_control_thread = std::thread::spawn(move || {
        let server = Server::http("0.0.0.0:8080").unwrap();
        println!(
            "{}",
            format!("Spawned server at: {}:{}", "0.0.0.0", "8080")
                .blue()
                .bold()
        );

        for request in server.incoming_requests() {
            let url = request.url();
            println!(
                "{}",
                "\n************************************** Request caught *************************************\n"
                .green()
                .bold()
            );
            let query = url.strip_prefix("/search?q=").unwrap_or("");
            let search_return = engine.search(query).unwrap();
            println!("{}", format!("Request: {}", query).blue());
            let resolved_pages =
                engine.resolve(search_return, consts::TEMPERATURE, consts::MAX_RESULTS);
            println!(
                "{}",
                format!("Resolved pages: {}", resolved_pages.len()).blue()
            );

            let body = resolved_pages
                .iter()
                .map(|p| format!("{}\n{}\n{}\n", p.name, p.body, p.link))
                .collect::<String>();

            // TODO: Switch to JSON
            let response = Response::from_string(body);
            request.respond(response).unwrap();

            println!(
                "{}",
                "\n************************************** Request processed *************************************\n"
                .green()
                .bold()
            );
        }
    });

    main_control_thread.join().unwrap();
    Ok(())
}
