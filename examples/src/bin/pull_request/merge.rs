use ghub::client as ghub_client;
use ghub::client::GithubClientV3;
use ghub::client::GithubMergeMethod;
use ghub::types::ResultDynError;

use lib::client;

#[tokio::main]
async fn main() -> ResultDynError<()> {
  env_logger::init();

  let mut client: GithubClientV3 = client::new()?;

  let res_body = client
    .merge_pull_request(ghub_client::MergePullRequestInput {
      repo_path: "sendyhalim/dummy",
      pull_number: "6",
      merge_method: GithubMergeMethod::Squash,
    })
    .await?;

  println!("Done merging {:?}", serde_json::to_string(&res_body)?);

  return Ok(());
}
