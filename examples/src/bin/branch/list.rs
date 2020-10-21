use ghub::types::ResultDynError;
use ghub::v3::branch;
use ghub::v3::client::GithubClient;

use lib::client;

#[tokio::main]
async fn main() -> ResultDynError<()> {
  env_logger::init();

  let client: GithubClient = client::new()?;
  let input = branch::ListBranchInput {
    repo_path: "sendyhalim/dummy",
    protected: None,
    per_page: 100,
    page: 1,
  };

  println!("Listing branch {:?}", input);

  let res_body = client.branch.list(input).await?;

  println!(
    "Done listing branch {}",
    serde_json::to_string_pretty(&res_body)?
  );

  return Ok(());
}
