use chrono::{DateTime, Local, Utc};
use tiny_http::{Header, Response, Server};
use docueyes::corpus::Page;
use docueyes::engine::Engine;
use crate::consts::{MAX_QUERY_LENGTH, MAX_RESULTS, MIN_QUERY_LENGTH, SERVER_LOCATION, SERVER_SPIN_UP_ATTEMPTS, TEMPERATURE};
use serde::Serialize;
use crate::logg::Logg;

#[derive(Serialize, Debug)]
enum SuccessCode {
    Success,
    Failed,
    Unknown,
}

#[derive(Serialize, Debug)]
struct RespBody {
    #[serde(serialize_with = "serialize_datetime")]
    datetime: DateTime<Local>,
    code: SuccessCode,
    query: String,
    resolved: Vec<Page>
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
    let mut delay = SERVER_SPIN_UP_ATTEMPTS;
    let server = loop {
        match Server::http(SERVER_LOCATION) {
            Ok(s) => break s,
            Err(e) => {
                Logg::error(format!("Bind failed: {}. Retrying in {}s...", e, delay));
                std::thread::sleep(std::time::Duration::from_secs(delay));
                delay = std::cmp::min(delay * 2, 60); // cap at 1 minute
            }
        }
    };
    std::thread::spawn(move || {
        Logg::info(format!("Server at {}", SERVER_LOCATION));

        for request in server.incoming_requests() {
            let url = request.url();
            let query = url.strip_prefix("/search?q=").unwrap_or("");
            let norm_query = query.replace("%20", " ");

            let mut success_code = SuccessCode::Unknown;
            // Safety checks
            if norm_query.is_empty() {
                Logg::error("Query is empty".to_string());
            }
            if norm_query.len() <= MAX_QUERY_LENGTH || norm_query.len() >= MIN_QUERY_LENGTH {
                let search_return = engine.search(&*norm_query).unwrap_or_else(|e| {
                    Logg::error(format!("Failed to search query cause: {}", e));
                    success_code = SuccessCode::Failed;
                    Vec::new()
                });

                Logg::info("Query good, serving".to_string());

                let resolved_pages =
                    engine.resolve(search_return, TEMPERATURE, MAX_RESULTS);

                success_code = SuccessCode::Success;
                let response_body = RespBody {
                    datetime: DateTime::from(Utc::now()),
                    code: success_code,
                    query: query.parse().unwrap(),
                    resolved: resolved_pages,
                };
                
                // TODO: Switch to JSON response
                let response = Response::from_string(serde_json::to_string(&response_body).unwrap_or_else(|e| {
                    Logg::error(format!("Failed to serialize response body: {}", e));
                    "Error".to_string()
                }))
                    .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap_or_else(|e| {
                        Logg::error("FATAL FATAL FATAL".to_string());
                        Logg::error(format!("Failed to create response header: {:?}.", e));
                        std::process::exit(1);
                    }))
                    .with_header(Header::from_bytes(&b"Maker"[..], &b"Kilroy Was Here"[..]).unwrap_or_else(|e| {
                        Logg::error("FATAL FATAL FATAL".to_string());
                        Logg::error(format!("Failed to create response header: {:?}.", e));
                        std::process::exit(1);
                    }));

                if let Err(e) = request.respond(response) {
                    Logg::error(format!("Failed to send response to server {}", e));
                }
            }
        }
    })
}