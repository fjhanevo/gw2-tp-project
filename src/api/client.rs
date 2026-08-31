use std::time::Duration;
use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;
use thiserror::Error;

const BASE_URL: &str = "https://api.guildwars2.com/v2";
const MAX_REQUEST: usize = 200;
const MAX_RETRIES: u32 = 4;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("http request failed: {0}")] 
    Request(#[from] reqwest::Error),

    #[error("rate limited after {0} retries")]
    RateLimited(u32),

    #[error("server error after retries: {0}")]
    ServerError(StatusCode),

    #[error("unexpected status: {0}")]
    UnexpectedStatus(StatusCode),
}

/// General batch fetcher to get data from the API, also deals with error handling
async fn fetch_with_retry<T: DeserializeOwned>(
    client: &Client,
    path: &str,
    ids: &[u32],
) -> Result<T, FetchError> {
    let ids_param = ids
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(",");

    let url = format!("{BASE_URL}/{path}/?ids={ids_param}");

    let mut attempt = 0;
    loop {
        attempt += 1;
        let response = client.get(&url).send().await?;
        let status = response.status();

        match status {
            StatusCode::OK => return Ok(response.json::<T>().await?),

            StatusCode::TOO_MANY_REQUESTS => {
                if attempt > MAX_RETRIES {
                    return Err(FetchError::RateLimited(attempt));
                }
                let wait = retry_after_header(&response)
                    .unwrap_or_else(|| backoff_delay(attempt));
                tokio::time::sleep(wait).await;
            }

            s if s.is_server_error() => {
                if attempt > MAX_RETRIES {
                    return Err(FetchError::ServerError(s));
                }
                tokio::time::sleep(backoff_delay(attempt)).await
            }

            other => return Err(FetchError::UnexpectedStatus(other)),
        }
    }
}

/// Uses a retry-after header if the API sends one, falls back to exponential backoff if not
fn retry_after_header(response: &reqwest::Response) -> Option<Duration> {
    response
        .headers()
        .get(reqwest::header::RETRY_AFTER)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
        .map(Duration::from_secs)

}

fn backoff_delay(attempt: u32) -> Duration {
    Duration::from_millis(200 * 2u64.pow(attempt.min(6)))
}

