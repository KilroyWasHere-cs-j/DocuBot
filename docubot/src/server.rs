use colored::Colorize;
use tiny_http::{Response, Server};
use docueyes::engine::Engine;
use crate::consts::{MAX_RESULTS, SERVER_LOCATION, TEMPERATURE};


///
/// Spins up an instance of the API server for Docubot
///
/// # Arguments
/// - engine `Engine` an instance of the current DocuBot search engine
///
/// # Returns
/// - handle 'JoinHandle' a handle to the newly created thread
///
pub fn spinup_server(engine: Engine) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let server = Server::http(SERVER_LOCATION).expect("failed to start server");
        println!(
            "{}",
            format!("Spawned server at: {}", SERVER_LOCATION)
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

            // Add match
            let query = url.strip_prefix("/search?q=").unwrap_or("");
            let search_return = match engine.search(query) {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!("Search failed: {}", e);
                    continue;
                }
            };
            println!("{}", format!("Request: {}", query).blue());
            let resolved_pages =
                engine.resolve(search_return, TEMPERATURE, MAX_RESULTS);
            println!(
                "{}",
                format!("Resolved pages: {}", resolved_pages.len()).blue()
            );

            let mut body = String::with_capacity(1024);
            for p in &resolved_pages {
                use std::fmt::Write;
                writeln!(body, "{}\n{}\n{}\n", p.name, p.body, p.link).unwrap();
            }

            // TODO: Switch to JSON
            let response = Response::from_string(body);
            request.respond(response).expect("A fatal error has occurred in server");

            println!(
                "{}",
                "\n************************************** Request processed *************************************\n"
                    .green()
                    .bold()
            );
        }
    })
}