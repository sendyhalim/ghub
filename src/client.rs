use std::collections::HashMap;

use crate::types::ResultDynError;

use reqwest::header;
use reqwest::header::HeaderMap;
use reqwest::Client as HttpClient;
use reqwest::ClientBuilder as HttpClientBuilder;
use serde_json::Value;

pub struct GithubClientV3 {
  pub api_key: String,
  pub api_secret: String,
  pub http_client: HttpClient,
}

impl GithubClientV3 {
  pub fn new(api_key: String, api_secret: String) -> ResultDynError<GithubClientV3> {
    let mut default_headers = HeaderMap::new();
    default_headers.insert(
      header::ACCEPT,
      header::HeaderValue::from_str("application/vnd.github.v3+json")?,
    );

    let client = GithubClientV3 {
      api_key,
      api_secret,
      http_client: HttpClientBuilder::new()
        .default_headers(default_headers)
        .build()?,
    };

    return Ok(client);
  }
}

impl GithubClientV3 {
  pub async fn createPullRequest(
    &mut self,
    title: &str,
    repo_path: &str,
    branch_name: &str,
    into_branch: &str,
  ) -> ResultDynError<Value> {
    let mut req_body = HashMap::new();
    req_body.insert("title", title);
    req_body.insert("head", branch_name);
    req_body.insert("base", into_branch);

    log::debug!(
      "Creating pull request {}, req_body: {:?}",
      repo_path,
      req_body
    );

    let res = self
      .http_client
      .post(&format!("/repos/{}/pulls", repo_path))
      .json(&req_body)
      .send()
      .await?;

    let res_body: Value = res.json().await.map_err(failure::Error::from)?;

    log::debug!("Done creating pull request, response {:?}", res_body);

    return Ok(res_body);
  }
}
