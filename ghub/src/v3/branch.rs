use std::sync::Arc;

use serde_json::Value;

use crate::types::ResultDynError;
use crate::v3::reference::DeleteReferenceInput;
use crate::v3::reference::GithubReferenceClient;

pub struct GithubBranchClient {
  pub(crate) reference_client: Arc<GithubReferenceClient>,
}

impl GithubBranchClient {
  pub fn new(reference_client: Arc<GithubReferenceClient>) -> ResultDynError<GithubBranchClient> {
    let client = GithubBranchClient { reference_client };

    return Ok(client);
  }
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
}
