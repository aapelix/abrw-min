use adblock::lists::{FilterSet, ParseOptions};
use adblock::{blocker::BlockerResult, request::Request, Engine};
use log::{error, info};
use reqwest;
use std::borrow::Cow;
use std::error::Error;
use std::sync::{Arc, Mutex};
use url::Url;
use webkit2gtk::{URIRequestExt, WebViewExt};

/// Fetches a block list from a given URL.
pub async fn fetch_block_list(block_list_url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    info!("Fetching block lists");

    let response = reqwest::get(block_list_url).await?;
    let block_list = response
        .text()
        .await?
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    Ok(block_list)
}

/// Handles resource load started event.
pub fn on_resource_load_started(
    webview: &webkit2gtk::WebView,
    _resource: &webkit2gtk::WebResource,
    request: &webkit2gtk::URIRequest,
    engine: &Engine,
) {
    if let Some(url_string) = request.uri() {
        match Url::parse(&url_string).ok() {
            Some(url) => {
                let domain = match url.host_str() {
                    Some(domain) => Cow::Borrowed(domain),
                    None => return,
                };

                let request2 = Request::new(&url.as_str(), &domain, "");
                match request2 {
                    Ok(req) => {
                        let result = engine.check_network_request(&req);
                        log_result(result, webview);
                    }
                    Err(err) => error!("Error creating request: {}", err),
                }
            }
            None => error!(
                "Error parsing URL {}: {}",
                url_string,
                Url::parse(&url_string).unwrap_err()
            ),
        }
    }
}

fn log_result(result: BlockerResult, webview: &webkit2gtk::WebView) {
    match result {
        BlockerResult {
            matched: true,
            important,
            redirect,
            rewritten_url,
            exception,
            filter,
        } => {
            if important {
                info!("Request matched an important rule and should be blocked.");
                webview.stop_loading()
            } else {
                info!("Request matched a non-important rule.");
                if let Some(redirect_url) = redirect {
                    info!("Redirecting to: {}", redirect_url);
                    webview.stop_loading()
                } else if let Some(rewritten_url) = rewritten_url {
                    info!("Rewritten URL: {}", rewritten_url);
                    webview.stop_loading()
                } else if let Some(exception) = exception {
                    info!("Request is an exception: {}", exception);
                    webview.stop_loading()
                } else if let Some(filter) = filter {
                    info!("Request matched filter: {}", filter);
                    webview.stop_loading()
                }
            }
        }
        _ => {}
    }
}

pub async fn fetch_rules(
    urls: Vec<String>,
    rules: Arc<Mutex<Vec<String>>>,
    filter_set: Arc<Mutex<FilterSet>>,
) {
    let mut fetch_handles = vec![];

    for url in urls {
        let rules_clone = Arc::clone(&rules);
        let filter_set_clone = Arc::clone(&filter_set);

        let fetch_handle = tokio::spawn(async move {
            if let Ok(fetched_rules) = fetch_block_list(&url).await {
                let mut rules_guard = rules_clone.lock().unwrap();
                rules_guard.extend(fetched_rules);

                let mut filter_set_guard = filter_set_clone.lock().unwrap();
                filter_set_guard.add_filters(&*rules_guard, ParseOptions::default());
            }
        });

        fetch_handles.push(fetch_handle);
    }

    for handle in fetch_handles {
        handle.await.expect("Failed to fetch adblock rules");
    }
}
