use reqwest::{blocking::Client};
use std::error::Error;
use dotenvy::dotenv;
use std::env;

pub fn get_input_data() -> String {
    match request() {
        Ok(body) => return body,
        Err(_) => return "failed".to_string()
    }
}

fn request() -> Result<String, Box<dyn Error>> {
    dotenv().ok();
    let cookie = env::var("SESSION_COOKIE").expect("FAILURE");

    let client = Client::new();

    // Build and send the request
    let response = client
        .get("https://https://adventofcode.com/2024/day/16/input")
        .header("User-Agent", "https://github.com/A-Stone-Olguin/AdventOfCode2025 by olguinstone@gmail.com")
        .header("Accept", "text/html")
        .header("Cookie", cookie)
        .send()?;

    println!("Status: {}", response.text());

    let body = response.text()?;
    Ok(body)
}