/*
 *
 * Collection of Built-In-Tests used for validating the functionality of DocuBot.
 *
 * TODO: Implement tests...
 *
 */

use crate::consts::{BIT_MAX_RESULTS, BIT_TEMPERATURE, BIT_TEST_PAGE_NAMES};
use anyhow::Result;
use docueyes::engine::Engine;

pub fn run(engine: &Engine) -> Result<()> {
    bit_1(engine)?;
    Ok(())
}

fn bit_1(engine: &Engine) -> Result<()> {
    let search_return = engine.search("Salesforce use AI")?;
    let resolved_pages = engine.resolve(search_return, BIT_TEMPERATURE, BIT_MAX_RESULTS);
    for page in resolved_pages {
        if !BIT_TEST_PAGE_NAMES.contains(&page.name.as_str()) {
        }
    }
    Ok(())
}
