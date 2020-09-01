use ghub::types::ResultDynError;
use ghub::v3::branch;
use ghub::v3::client::GithubClient;

use lib::client;

#[tokio::main]
async fn main() -> ResultDynError<()> {
  env_logger::init();

  let client: GithubClient = client::new()?;
  let input = branch::DeleteBranchInput {
    repo_path: "sendyhalim/dummy",
    branch_name: "testing-ghub-branch-delete-feature",
  };

  println!("Deleting branch {:?}", input);

  let res_body = client.branch.delete(input).await?;

  println!("Done deleting branch {}", serde_json::to_string(&res_body)?);

  return Ok(());
}
