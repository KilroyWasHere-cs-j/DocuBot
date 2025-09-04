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

mod consts;

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

    print!("{}\n", consts::BANNER);
    println!("----------------------Starting----------------------");

    let corpus = load_corpus(args.get(1).unwrap()).unwrap();
    let mut engine = Engine::new(corpus);

    println!("\nPreparing embeddings");

    // Based on file existance and CLI arguments handle loading and compilation of embeddings
    if args.get(2) == Some(&String::from("--recompile")) {
        engine.build_embeddings().unwrap();
        println!("Embeddings recompiling triggered");
        println!("Embeddings recompiled successfully");
        println!("Caching generated embeddings");
        engine.cache_embeddings("embeddings.txt").unwrap();
        println!("Embeddings cached successfully");
    } else {
        match fs::exists("embeddings.txt") {
            Ok(true) => {
                println!("Loading embeddings from found file");
                engine.load_embeddings("embeddings.txt").unwrap();
                println!("Embeddings loaded successfully");
            }
            Ok(false) => {
                println!("Embeddings not found, compiling embeddings");
                engine.build_embeddings().unwrap();
                println!("Embeddings compiled successfully");
                println!("Caching generated embeddings");
                engine.cache_embeddings("embeddings.txt").unwrap();
                println!("Embeddings cached successfully");
            }
            Err(e) => return Err(e.into()),
        }
    }
    println!("BIT 1 Passed");

    println!("\n----------------------Entering Main Control Loop----------------------\n");

    let main_control_thread = std::thread::spawn(move || {
        let server = Server::http("0.0.0.0:8080").unwrap();
        println!("Spawned server at: {}:{}", "0.0.0.0", "8080");

        for request in server.incoming_requests() {
            let url = request.url();
            println!(
                "\n************************************** Request caught *************************************\n"
            );
            let query = url.strip_prefix("/search?q=").unwrap_or("");
            let search_return = engine.search(query).unwrap();
            println!("Request: {}", query);
            let resolved_pages =
                engine.resolve(search_return, consts::TEMPERATURE, consts::MAX_RESULTS);
            println!("Resolved pages: {:?}", resolved_pages);

            let body = resolved_pages
                .iter()
                .map(|p| format!("{}\n{}\n{}\n", p.name, p.body, p.link))
                .collect::<String>();

            // TODO: Switch to JSON
            let response = Response::from_string(body);
            request.respond(response).unwrap();

            println!(
                "\n************************************** Request processed *************************************\n"
            );
        }
    });

    main_control_thread.join().unwrap();
    Ok(())
}
