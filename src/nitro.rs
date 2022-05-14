
use reqwest::blocking::Client;

#[derive(Debug, Deserialize)]
pub struct Login {
  pub access_token: String,
}

pub type LoginResult = Result<Login, reqwest::Error>;

pub fn exchange_token(code: &str) -> LoginResult {
  let client = Client::new();
  let params = [
      ("client_id", "nitro-cli-app"),
      ("grant_type", "authorization_code"),
      ("code", code),
      ("csrf-token", "nocheck"),
      ("redirect_uri", "http://localhost:9001/callback")];
  let res = client.post("https://sso.gonitrodev.com/token")
    .form(&params)
    .send()?
    .json::<Login>()?;
   Ok(res)
}

pub fn auth_url() -> String {
  let scopes = [
    "Documents",
  ]
  .join(",");

  let params = [
    String::from("client_id=nitro-cli-app"),
    String::from("redirect_uri=http://localhost:9001/callback"),
    String::from("response_type=code"),
    String::from("csrf-token=nocheck"),
    format!("scope={}", scopes),
  ]
  .join("&");
  format!("https://sso.gonitrodev.com/authorize?{}", params)
}