/*
 *
 * Collection of Built-In-Tests used for validating the functionality of DocuBot.
 *
 * TODO: Implement tests...
 *
 */

use crate::consts::{BIT_MAX_RESULTS, BIT_TEMPERATURE, BIT_TEST_PAGE_NAMES};
use anyhow::Result;
use colored::*;
use docueyes::engine::Engine;

pub fn run(engine: &Engine) -> Result<()> {
    println!(
        "{}",
        "\n----------------------Preforming BIT Tests----------------------\n"
            .blue()
            .bold()
    );
    bit_1(engine)?;
    println!("{}", "BIT 1 Passed".green().bold());
    Ok(())
}

fn bit_1(engine: &Engine) -> Result<()> {
    let search_return = engine.search("Salesforce use AI")?;
    println!("Search return similarities BIT {}", format!("{:?}", search_return).bright_blue().bold());
    let resolved_pages = engine.resolve(search_return, BIT_TEMPERATURE, BIT_MAX_RESULTS);

    println!("{}", "BIT 1 Running".green().bold());
    println!("Query is {}", "Salesforce use AI");
    for page in resolved_pages {
        if !BIT_TEST_PAGE_NAMES.contains(&page.name.as_str()) {
        }
    }
    Ok(())
}
