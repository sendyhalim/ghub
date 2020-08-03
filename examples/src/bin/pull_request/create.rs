use ghub::client as ghub_client;
use ghub::client::GithubClientV3;
use ghub::types::ResultDynError;

use lib::client;

#[tokio::main]
async fn main() -> ResultDynError<()> {
  env_logger::init();

  let mut client: GithubClientV3 = client::new()?;

  let res_body = client
    .create_pull_request(ghub_client::CreatePullRequestInput {
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
