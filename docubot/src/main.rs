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
use docueyes::engine::Engine;
use lazy_static::lazy_static;
use std::env;
use std::fs;
use tiny_http::{Response, Server};

const BANNER: &str = r"
 ____   __    ___  _  _  ____   __  ____
(    \ /  \  / __)/ )( \(  _ \ /  \(_  _)
 ) D ((  O )( (__ ) \/ ( ) _ ((  O ) )(
(____/ \__/  \___)\____/(____/ \__/ (__)
          **<<Kilroy Was Here>>**
";

const TEMPERATURE: f32 = 0.34;
const MAX_RESULTS: usize = 10;

// Constants specific to the BIT tests
const BIT_TEMPERATURE: f32 = 0.34; // 0.34
const BIT_MAX_RESULTS: usize = 10; // 10
lazy_static! {
    static ref BIT_TEST_PAGE_NAMES: Vec<&'static str> = vec![
        "Salesforce is cloud-based",
        "CRM stands for Customer Relationship Management",
        "Salesforce automates workflows",
        "AppExchange is like an app store",
        "Einstein AI powers insights",
        "Trailhead teaches Salesforce"
    ];
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage:");
        println!("docubot <corpus_file> --recompile");
        return Ok(());
    }

    print!("{}\n", BANNER);
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

    println!("\n----------------------Preforming BIT Tests----------------------\n");

    let search_return = engine.search("Salesforce use AI")?;
    println!("{:?}", search_return);
    let resolved_pages = engine.resolve(search_return, BIT_TEMPERATURE, BIT_MAX_RESULTS);

    println!("BIT 1 Running");
    for page in resolved_pages {
        if !BIT_TEST_PAGE_NAMES.contains(&page.name.as_str()) {
            println!("BIT 1 Failed");
            break;
        }
    }
    println!("BIT 1 Passed");

    println!("\n----------------------Entering Main Control Loop----------------------\n");

    Ok(())
}
