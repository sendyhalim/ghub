use dotenv;
use ghub::client::GithubClientV3;
use ghub::types::ResultDynError;

pub fn new() -> ResultDynError<GithubClientV3> {
  dotenv::dotenv().ok();

  return GithubClientV3::new(&std::env::var("GITHUB_TOKEN")?);
}
