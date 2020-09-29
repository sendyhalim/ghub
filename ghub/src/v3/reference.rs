use std::sync::Arc;

use reqwest::Client as HttpClient;

use crate::types::ResultDynError;
use crate::v3::util as client_util;

pub struct GithubReferenceClient {
  pub(crate) http_client: Arc<HttpClient>,
}

impl GithubReferenceClient {
  pub fn new(http_client: Arc<HttpClient>) -> ResultDynError<GithubReferenceClient> {
    let client = GithubReferenceClient { http_client };

    return Ok(client);
  }
}

#[derive(Debug)]
pub struct DeleteReferenceInput<'a> {
  pub repo_path: &'a str,

  /// Reference path formats:
  /// - Branch: 'heads/<branch name>'
  /// - Tag: 'tags/<tag name>'
  pub reference_path: &'a str,
}

impl GithubReferenceClient {
  /// Delete a reference(tag, branch) from github
  /// https://docs.github.com/en/rest/reference/git#references
  ///
  /// This method returns a unit `()` because github returns 204 (no content)
  /// for successful  response.
  /// https://docs.github.com/en/rest/reference/git#delete-a-reference
  pub async fn delete<'a>(&self, input: DeleteReferenceInput<'a>) -> ResultDynError<()> {
    let DeleteReferenceInput {
      repo_path,
      reference_path,
    } = input;

    log::debug!("Deleting a reference {:?}", input);

    let req = self.http_client.delete(&client_util::api_path(&format!(
      "/repos/{}/git/refs/{}",
      repo_path, reference_path
    )));

    log::debug!("Initiating request instance to delete reference {:?}", req);

    let res = req.send().await;

    log::debug!("Done deleting reference, response {:?}", res);

    res?.error_for_status()?;

    return Ok(());
  }
}
