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
    .create(pull_request::CreatePullRequestInput {
      title: "Trying to merge hey!",
      repo_path: "sendyhalim/dummy",
      branch_name: "hey-test",
      into_branch: "master",
    })
    .await?;

  println!(
    "Done creating pull request {}",
    serde_json::to_string(&res_body)?
  );

  return Ok(());
}
