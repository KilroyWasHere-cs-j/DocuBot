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

mod api;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use anyhow::Result;
use api::endpoints::{echo, hello, manual_hello};
use docueyes::corpus::load_corpus;
use std::env;

const BANNER: &str = r"
 ____   __    ___  _  _  ____   __  ____
(    \ /  \  / __)/ )( \(  _ \ /  \(_  _)
 ) D ((  O )( (__ ) \/ ( ) _ ((  O ) )(
(____/ \__/  \___)\____/(____/ \__/ (__)
          **Kilroy Was Here**
";

/// Program specific constants
const TEMPERATURE: f32 = 0.7;
const FAIL_THRESHOLD: f32 = 0.1;

// #[actix_web::main]
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: docubot <corpus_file>");
        return Ok(());
    }

    print!("{}\n", BANNER);
    println!("Starting...");

    let corpus = load_corpus(args.get(1).unwrap())?;
    let mut engine = docueyes::engine::Engine::new(corpus);

    println!("Generating embeddings");
    if let Err(e) = engine.build_embeddings() {
        eprintln!("Embedding build failed: {}", e);
    }
    println!("Generation of embeddings is complete");

    // let search_return = engine.search("I like planes give me some ideas on fun things to do")?;

    // println!("{:?}", search_return);

    // // Resolve needs to be massively improved
    // println!(
    //     "{:?}",
    //     engine.resolve(docueyes::engine::ResolveLevel::First, search_return)
    // );

    // let _ = HttpServer::new(|| {
    //     App::new()
    //         .service(hello)
    //         .service(echo)
    //         .route("/hey", web::get().to(manual_hello))
    // })
    // .bind(("127.0.0.1", 8080))?
    // .run()
    // .await;

    Ok(())
}
