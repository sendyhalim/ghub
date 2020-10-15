use ghub::types::ResultDynError;
use ghub::v3::client::GithubClient;
use ghub::v3::pull_request;

use lib::client;

#[tokio::main]
async fn main() -> ResultDynError<()> {
  env_logger::init();

  let client: GithubClient = client::new()?;

  let res_body = client
    .pull_request
    .get_by_head(pull_request::GetPullRequestByHeadInput {
      repo_path: "sendyhalim/dummy",
      branch_name: "do-not-delete-pr",
      branch_owner: "sendy",
    })
    .await?;

  println!(
    "Done getting pull request {}",
    serde_json::to_string(&res_body)?
  );

  return Ok(());
}
