use std::sync::Arc;

use reqwest::Client as HttpClient;
use serde_json::Value;

use crate::types::ResultDynError;
use crate::v3::reference::DeleteReferenceInput;
use crate::v3::reference::GithubReferenceClient;
use crate::v3::util as client_util;

pub struct GithubBranchClient {
  pub(crate) reference_client: Arc<GithubReferenceClient>,
  pub(crate) http_client: Arc<HttpClient>,
}

impl GithubBranchClient {
  pub fn new(
    http_client: Arc<HttpClient>,
    reference_client: Arc<GithubReferenceClient>,
  ) -> ResultDynError<GithubBranchClient> {
    let client = GithubBranchClient {
      http_client,
      reference_client,
    };

    return Ok(client);
  }
}

#[derive(Debug)]
pub struct ListBranchInput<'a> {
  pub repo_path: &'a str,
  pub protected: Option<bool>,
  pub per_page: u32,
  pub page: u32,
}

#[derive(Debug)]
pub struct DeleteBranchInput<'a> {
  pub repo_path: &'a str,
  pub branch_name: &'a str,
}

impl GithubBranchClient {
  /// This method returns a unit `()` because github returns 204 (no content)
  /// for successful  response.
  /// https://docs.github.com/en/rest/reference/git#delete-a-reference
  pub async fn delete<'a>(&self, input: DeleteBranchInput<'a>) -> ResultDynError<()> {
    let DeleteBranchInput {
      repo_path,
      branch_name,
    } = input;

    log::debug!("Deleting a branch {:?}", input);

    return self
      .reference_client
      .delete(DeleteReferenceInput {
        repo_path,
        reference_path: &format!("heads/{}", branch_name),
      })
      .await;
  }

  /// https://docs.github.com/en/free-pro-team@latest/rest/reference/repos#list-branches
  pub async fn list<'a>(&self, input: ListBranchInput<'a>) -> ResultDynError<Value> {
    let ListBranchInput {
      repo_path,
      protected,
      per_page,
      page,
    } = input;

    log::debug!("Listing branches {:?}", input);

    let req = self.http_client.get(&client_util::api_path(&format!(
      "/repos/{}/branches?protected={}&per_page={}&page={}",
      repo_path,
      protected.unwrap_or(false),
      per_page,
      page
    )));

    log::debug!("Initiating request instance to list branch {:?}", req);

    let res = req.send().await;

    log::debug!("Done getting branches, response {:?}", res);

    return client_util::result_from_server_response(res?).await;
  }
}
