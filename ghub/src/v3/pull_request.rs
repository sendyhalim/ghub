use std::collections::HashMap;
use std::rc::Rc;

use reqwest::Client as HttpClient;
use serde_json::Value;

use crate::types::ResultDynError;
use crate::v3::util as client_util;

pub struct GithubPullRequestClient {
  pub(crate) http_client: Rc<HttpClient>,
}

impl GithubPullRequestClient {
  pub fn new(http_client: Rc<HttpClient>) -> ResultDynError<GithubPullRequestClient> {
    let client = GithubPullRequestClient { http_client };

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

impl GithubPullRequestClient {
  pub async fn create<'a>(&mut self, input: CreatePullRequestInput<'a>) -> ResultDynError<Value> {
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
      .post(&client_util::api_path(&format!(
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

  pub async fn merge<'a>(&mut self, input: MergePullRequestInput<'a>) -> ResultDynError<Value> {
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
      .put(&client_util::api_path(&format!(
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
