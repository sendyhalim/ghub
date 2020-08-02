use ghub::client::GithubClientV3;
use ghub::types::ResultDynError;

use lib::client;

#[tokio::main]
async fn main() -> ResultDynError<()> {
  env_logger::init();

  let mut client: GithubClientV3 = client::new()?;

  let res_body = client
    .create_pull_request(
      "Trying to merge hey!",
      "sendyhalim/dummy",
      "hey-test",
      "master",
    )
    .await?;

  println!(
    "Done creating pull request {}",
    serde_json::to_string(&res_body)?
  );

  return Ok(());
}
