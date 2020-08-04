use std::sync::Arc;

use reqwest::header;
use reqwest::header::HeaderMap;
use reqwest::Client as HttpClient;
use reqwest::ClientBuilder as HttpClientBuilder;

use crate::types::ResultDynError;
use crate::v3::pull_request::GithubPullRequestClient;

pub struct GithubClient {
  http_client: Arc<HttpClient>,
  pub pull_request: GithubPullRequestClient,
}

impl GithubClient {
  pub fn new(personal_access_token: &str) -> ResultDynError<GithubClient> {
    let mut default_headers = HeaderMap::new();
    default_headers.insert(
      header::ACCEPT,
      header::HeaderValue::from_static("application/vnd.github.v3+json"),
    );

    default_headers.insert(
      header::AUTHORIZATION,
      header::HeaderValue::from_str(&format!("token {}", personal_access_token))?,
    );

    default_headers.insert(
      header::USER_AGENT,
      header::HeaderValue::from_static("reqwest"),
    );

    log::debug!(
      "Creating http client with default headers {:?}",
      default_headers
    );

    let http_client = HttpClientBuilder::new()
      .default_headers(default_headers)
      .build()?;

    let http_client = Arc::new(http_client);

    let pull_request = GithubPullRequestClient {
      http_client: http_client.clone(),
    };

    let client = GithubClient {
      http_client: http_client.clone(),
      pull_request,
    };

    return Ok(client);
  }
}
