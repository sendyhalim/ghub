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

pub enum GithubMergeMethod {
  Merge,
  Rebase,
  Squash,
}

fn merge_method_to_string(merge_method: GithubMergeMethod) -> String {
  return match merge_method {
    GithubMergeMethod::Merge => "merge",
    GithubMergeMethod::Squash => "squash",
    GithubMergeMethod::Rebase => "rebase",
  }
  .to_owned();
}

impl GithubClientV3 {
  pub async fn create_pull_request(
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

  pub async fn merge_pull_request(
    &mut self,
    repo_path: &str,
    pull_number: &str,
    merge_method: GithubMergeMethod,
  ) -> ResultDynError<Value> {
    let mut req_body = HashMap::new();
    req_body.insert("merge_method", merge_method_to_string(merge_method));

    log::debug!(
      "Merging pull request {} pull number {}, req_body: {:?}",
      repo_path,
      pull_number,
      req_body
    );

    let res = self
      .http_client
      .post(&format!("/repos/{}/pulls/{}/merge", repo_path, pull_number))
      .json(&req_body)
      .send()
      .await?;

    let res_body: Value = res.json().await.map_err(failure::Error::from)?;

    log::debug!("Done merging pull request, response {:?}", res_body);

    return Ok(res_body);
  }
}
