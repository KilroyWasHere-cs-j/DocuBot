use std::borrow::Cow;
use chrono::{DateTime, Local, Utc};
use tiny_http::{Header, Response, Server};
use docueyes::corpus::Page;
use docueyes::engine::Engine;
use crate::consts::{MAX_QUERY_LENGTH, MAX_RESULTS, MIN_QUERY_LENGTH, SERVER_LOCATION, TEMPERATURE};
use serde::Serialize;

#[derive(Serialize, Debug)]
enum SuccessCode {
    Success,
    Failed,
}

#[derive(Serialize, Debug)]
struct RespBody<'a> {
    #[serde(serialize_with = "serialize_datetime")]
    datetime: DateTime<Local>,
    code: SuccessCode,
    query: String,
    resolved: Vec<&'a Page>
}

fn serialize_datetime<S>(
    dt: &DateTime<Local>,
    serializer: S
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&dt.to_rfc3339())
}

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
    // TODO harden this code and probably make it it's own struct
    std::thread::spawn(move || {
        let server = Server::http(SERVER_LOCATION).expect("failed to start server");

        for request in server.incoming_requests() {
            let url = request.url();
            let query = url.strip_prefix("/search?q=").unwrap_or("");
            let norm_query = query.replace("%20", " ");

            println!("Query is {}", norm_query);
            // Safety checks
            if norm_query.is_empty() {
                break;
            }
            if norm_query.len() <= MAX_QUERY_LENGTH || norm_query.len() >= MIN_QUERY_LENGTH {
                let search_return = match engine.search(&*norm_query) {
                    Ok(r) => r,
                    Err(_) => {
                        break;
                    }
                };

                let resolved_pages =
                    engine.resolve(search_return, TEMPERATURE, MAX_RESULTS);

                let response_body = RespBody {
                    datetime: DateTime::from(Utc::now()),
                    code: SuccessCode::Success,
                    query: query.parse().unwrap(),
                    resolved: resolved_pages,
                };

                // TODO: Switch to JSON response
                let response = Response::from_string(serde_json::to_string(&response_body).unwrap_or("".to_string()))
                    .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
                    .with_header(Header::from_bytes(&b"Maker"[..], &b"Kilroy Was Here"[..]).unwrap());;

                if let Err(e) = request.respond(response) {
                    tracing::error!("Failed to send response: {}", e);
                }
            }
        }
    })
}