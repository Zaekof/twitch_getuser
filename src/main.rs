extern crate twitch_lib;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::io;
use crate::twitch_lib::twitch::{Request, Api, user::User};

fn main() {
    dotenv().ok();

    let key = "CLIENT_ID";
    let mut _client_id= String::new();
    match env::var(key) {
        Ok(val) => _client_id = val,
        Err(e) => {
            println!("couldn't interpret {}: {}", key, e);
            panic!();
        },
    };

    let key = "CLIENT_SECRET";
    let mut _client_secret= String::new();
    match env::var(key) {
        Ok(val) => _client_secret = val,
        Err(e) => {
            println!("couldn't interpret {}: {}", key, e);
            panic!();
        },
    };

    println!("Enter a login : ");
    let input_login = read_string();

    const TWITCH_API_BASE_URL: &'static str = "https://api.twitch.tv/helix/users";
    let client_secret: &'static str = string_to_static_str(_client_secret);
    let client_id: &'static str = string_to_static_str(_client_id);
    let login: &'static str = string_to_static_str(input_login);

    println!("{:?}", user_request(&TWITCH_API_BASE_URL, &client_id, &client_secret, &login));
}
fn user_request(url: &'static str, client_id: &'static str, client_secret: &'static str, login: &'static str) -> User {
    let api = Api::new(url, client_id, client_secret, login);
    Request::get_user(&api)
}
fn read_string() -> String {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<String>() {
            Ok(login) => return login,
            Err(_) => {
                eprintln!("Please enter a valid login : ")
            },
        }
    }
}
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
