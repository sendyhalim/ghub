use std::collections::HashMap;
use std::sync::Arc;

use reqwest::Client as HttpClient;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::types::ResultDynError;
use crate::v3::util as client_util;

pub struct GithubPullRequestClient {
  pub(crate) http_client: Arc<HttpClient>,
}

impl GithubPullRequestClient {
  pub fn new(http_client: Arc<HttpClient>) -> ResultDynError<GithubPullRequestClient> {
    let client = GithubPullRequestClient { http_client };

    return Ok(client);
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum GithubMergeMethod {
  #[serde(rename = "merge")]
  Merge,

  #[serde(rename = "rebase")]
  Rebase,

  #[serde(rename = "squash")]
  Squash,
}

#[derive(Debug)]
pub struct CreatePullRequestInput<'a> {
  pub title: &'a str,
  pub repo_path: &'a str,
  pub branch_name: &'a str,
  pub into_branch: &'a str,
}

#[derive(Debug)]
pub struct MergePullRequestInput<'a> {
  pub repo_path: &'a str,
  pub pull_number: &'a str,
  pub merge_method: GithubMergeMethod,
}

#[derive(Debug)]
pub struct GetPullRequestByHeadInput<'a> {
  pub repo_path: &'a str,
  pub branch_owner: &'a str,
  pub branch_name: &'a str,
}

impl GithubPullRequestClient {
  pub async fn create<'a>(&self, input: CreatePullRequestInput<'a>) -> ResultDynError<Value> {
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

    log::debug!("Initiating request instance {:?} {:?}", req, req_body);

    let res = req.send().await;

    log::debug!("Done creating pull request, response {:?}", res);

    return client_util::result_from_server_response(res?).await;
  }

  pub async fn merge<'a>(&self, input: MergePullRequestInput<'a>) -> ResultDynError<Value> {
    let MergePullRequestInput {
      repo_path,
      pull_number,
      merge_method,
    } = input;
    let mut req_body = HashMap::new();
    req_body.insert("merge_method", merge_method);

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

    log::debug!("Initiating request instance {:?} {:?}", req, req_body);

    let res = req.send().await;

    log::debug!("Done merging pull request, response {:?}", res);

    return client_util::result_from_server_response(res?).await;
  }

  /// Get pull request by head in format `{branch_owner}:{branch_name}'`
  ///
  /// So for example if my user is sendyhalim and I'm creating a PR
  /// with branch name `x` then head will be `sendyhalim:x`.
  pub async fn get_by_head<'a>(
    &self,
    input: GetPullRequestByHeadInput<'a>,
  ) -> ResultDynError<Option<Value>> {
    log::debug!("Getting pull request by head {:?}", input);

    let GetPullRequestByHeadInput {
      branch_owner,
      repo_path,
      branch_name,
    } = input;

    let req = self.http_client.get(&client_util::api_path(&format!(
      "/repos/{}/pulls?head={}:{}",
      repo_path, branch_owner, branch_name
    )));

    log::debug!(
      "Initiating request instance to list pull requests {:?}",
      req
    );

    let res = req.send().await;

    log::debug!("Done listing pull request, response {:?}", res);

    let body: Value = client_util::result_from_server_response(res?).await?;

    log::debug!("Response body from listing pull request {:?}", body);

    // Try to get the first index,
    // if it's not an object then we assume it's the PR does not exist
    let pull_request: Option<Value> = match &body[0] {
      Value::Object(obj) => Some(Value::Object(obj.to_owned())),
      _ => None,
    };

    // We will try to get the PR via single record GET API req because there are
    // some fields that only available on single record API.
    // For example, the Mergeable field is compute-intensive, so it's only exposed on Get.
    // Ref: https://github.com/octokit/octokit.net/issues/1710#issuecomment-342331188

    if pull_request.is_none() {
      return Ok(pull_request);
    }

    let pull_request = pull_request.unwrap();

    let req = self.http_client.get(&client_util::api_path(&format!(
      "/repos/{}/pulls/{}",
      repo_path, pull_request["number"]
    )));

    log::debug!(
      "Initiating request instance to get pull request detail {:?}",
      req
    );

    let res = req.send().await;

    log::debug!("Done getting pull request detail, response {:?}", res);

    return client_util::result_from_server_response(res?)
      .await
      .map(Some);
  }
}
