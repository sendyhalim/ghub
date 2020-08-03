use std::collections::HashMap;

use crate::types::ResultDynError;

use reqwest::header;
use reqwest::header::HeaderMap;
use reqwest::Client as HttpClient;
use reqwest::ClientBuilder as HttpClientBuilder;
use serde_json::Value;

pub struct GithubClientV3 {
  pub http_client: HttpClient,
}

impl GithubClientV3 {
  pub fn new(personal_access_token: &str) -> ResultDynError<GithubClientV3> {
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

    let client = GithubClientV3 {
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
  pub fn api_path(api_path: &str) -> String {
    return format!("https://api.github.com{}", api_path);
  }
}

pub struct CreatePullRequestInput<'a> {
  pub title: &'a str,
  pub repo_path: &'a str,
  pub branch_name: &'a str,
  pub into_branch: &'a str,
}

pub struct MergePullRequestInput<'a> {
  pub repo_path: &'a str,
  pub pull_number: &'a str,
  pub merge_method: GithubMergeMethod,
}

impl GithubClientV3 {
  pub async fn create_pull_request<'a>(
    &mut self,
    input: CreatePullRequestInput<'a>,
  ) -> ResultDynError<Value> {
    let CreatePullRequestInput {
      title,
      repo_path,
      branch_name,
      into_branch,
    } = input;
    let mut req_body = HashMap::new();
    req_body.insert("title", title);
    req_body.insert("head", branch_name);
    req_body.insert("base", into_branch);

    log::debug!(
      "Creating pull request {}, req_body: {:?}",
      repo_path,
      req_body
    );

    let req = self
      .http_client
      .post(&GithubClientV3::api_path(&format!(
        "/repos/{}/pulls",
        repo_path
      )))
      .body(serde_json::to_string(&req_body)?);

    log::debug!("Initiating request instance {:?}", req);

    let res = req.send().await;

    log::debug!("Done creating pull request, response {:?}", res);

    let res_body: Value = res?.json().await.map_err(failure::Error::from)?;

    return Ok(res_body);
  }

  pub async fn merge_pull_request<'a>(
    &mut self,
    input: MergePullRequestInput<'a>,
  ) -> ResultDynError<Value> {
    let MergePullRequestInput {
      repo_path,
      pull_number,
      merge_method,
    } = input;
    let mut req_body = HashMap::new();
    req_body.insert("merge_method", merge_method_to_string(merge_method));

    log::debug!(
      "Merging pull request {} pull number {}, req_body: {:?}",
      repo_path,
      pull_number,
      req_body
    );

    let req = self
      .http_client
      .put(&GithubClientV3::api_path(&format!(
        "/repos/{}/pulls/{}/merge",
        repo_path, pull_number
      )))
      .body(serde_json::to_string(&req_body)?);

    log::debug!("Initiating request instance {:?}", req);

    let res = req.send().await;

    log::debug!("Done merging pull request, response {:?}", res);

    let res_body: Value = res?.json().await.map_err(failure::Error::from)?;

    return Ok(res_body);
  }
}
