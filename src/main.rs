use futures::future::join_all;
use reqwest::Client;
use std::time::Duration;
use tokio::task;

#[allow(dead_code)]
#[derive(Debug)]
struct RequestError {
    status_code: u16,
    error: String,
}

async fn fetch_multiple(urls: Vec<String>) -> Vec<Result<reqwest::Response, RequestError>> {
    // Create client with a 30s max timeout
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap();

    let mut handles: Vec<_> = vec![];
    let mut responses: Vec<_> = vec![];

    // Create a handle for each url
    for url in urls.into_iter() {
        let client = client.clone();
        let handle = task::spawn(async move { client.get(&url).send().await });
        handles.push(handle);
    }

    let results = join_all(handles).await;

    for result in results {
        // Match result for HTTP_ERROR or handle JOIN_ERROR
        let res = match result {
            Ok(Ok(body)) => Ok(body),
            Ok(Err(http_error)) => Err(RequestError {
                status_code: http_error.status().map(|s| s.as_u16()).unwrap_or(0),
                error: http_error.to_string(),
            }),
            Err(join_error) => Err(RequestError {
                status_code: 0,
                error: join_error.to_string(),
            }),
        };
        responses.push(res);
    }
    responses
}

#[tokio::main]
async fn main() {
    let urls: Vec<String> = vec![
        "https://google.com".to_string(),
        "https://en.wikipedia.com".to_string(),
    ];
    let responses = fetch_multiple(urls);
    let mut valid: Vec<reqwest::Response> = vec![];

    for res in responses.await.into_iter() {
        // Match for reqwest::Response or RequestError
        match res {
            Ok(v) => {
                valid.push(v);
            }
            Err(e) => {
                println!("Error: {:?}", e.status_code);
            }
        }
    }

    for res in valid.into_iter() {
        println!("Status: {}", res.status());
    }
}
