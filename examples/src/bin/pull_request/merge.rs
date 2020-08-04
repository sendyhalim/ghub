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
    .merge(pull_request::MergePullRequestInput {
      repo_path: "sendyhalim/dummy",
      pull_number: "7",
      merge_method: pull_request::GithubMergeMethod::Squash,
    })
    .await?;

  println!("Done merging {:?}", serde_json::to_string(&res_body)?);

  return Ok(());
}
