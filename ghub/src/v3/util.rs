use reqwest::Response;
use serde_json::Value;

use crate::types::ResultDynError;

pub(crate) fn api_path(api_path: &str) -> String {
  return format!("https://api.github.com{}", api_path);
}

pub(crate) async fn result_from_server_response(res: Response) -> ResultDynError<Value> {
  if res.error_for_status_ref().is_err() {
    let res_body: Value = res.json().await.map_err(failure::Error::from)?;
    let err_msg: String = res_body["errors"][0]["message"]
      .as_str()
      .ok_or(failure::format_err!(
        "Could not read github error message. Response body {}",
        serde_json::to_string(&res_body)?
      ))?
      .to_owned();

    return Err(failure::err_msg(err_msg));
  }

  let res_body = res.json().await.map_err(failure::Error::from)?;

  return Ok(res_body);
}
