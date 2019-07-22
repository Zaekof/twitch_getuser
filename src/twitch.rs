pub mod user;

use reqwest::Url;
use minreq::{get};
use user::{User};

#[derive(Debug)]
pub struct Api {
  base_url: &'static str,
  client_id: &'static str,
  client_secret: &'static str,
  login: &'static str
}

pub trait Request {
  fn get_user(&self) -> User;
  fn req_get_user(&self) -> String;
}

impl Api {
  pub fn new(base_url: &'static str, client_id: &'static str, client_secret: &'static str, login: &'static str) -> Api {
    Api {
      base_url,
      client_id,
      client_secret,
      login
    }
  }
}

impl Request for Api {
  fn get_user(&self) -> User {

    let data = self.req_get_user();

    let s: &'static str = string_to_static_str(data);
    let parsed = json::parse(s).unwrap();

    let uid = &parsed["data"][0]["id"];
    let login = &parsed["data"][0]["login"];
    let display_name = &parsed["data"][0]["display_name"];
    let description = &parsed["data"][0]["description"];

    User::new(
      uid.to_string(),
      login.to_string(),
      display_name.to_string(),
      description.to_string()
    )
  }
  fn req_get_user(&self) -> String {
    let url = Url::parse_with_params(self.base_url,
                                     &[("login", self.login)]).unwrap();

    let response = get(url.to_string())
      .with_header("Client-Id", self.client_id)
      .with_header("Authorization", "Bearer".to_string() + self.client_secret)
      .send();
    match response {
      Ok(response) => {
        return response.body
      },
      Err(_) => {
        return String::from("an error")
      }
    }
  }
}

fn string_to_static_str(s: String) -> &'static str {
  Box::leak(s.into_boxed_str())
}