pub(crate) fn api_path(api_path: &str) -> String {
  return format!("https://api.github.com{}", api_path);
}
