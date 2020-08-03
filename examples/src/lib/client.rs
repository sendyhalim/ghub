use dotenv;
use ghub::types::ResultDynError;
use ghub::v3::client::GithubClient;

pub fn new() -> ResultDynError<GithubClient> {
  dotenv::dotenv().ok();

  return GithubClient::new(&std::env::var("GITHUB_TOKEN")?);
}
