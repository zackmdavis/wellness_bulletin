use std::fs;
use std::time::Duration;

use reqwest::header;

fn read_access_token() -> Result<String, std::io::Error> {
    fs::read_to_string(".oura_access_token")
}

fn activity_bulletin() -> Result<(), Box<dyn std::error::Error>> {
    let token_file_content = read_access_token()?;
    let access_token = token_file_content.trim_end();
    println!("{:?}", access_token);

    let mut headers = header::HeaderMap::new();
    let mut auth_header_value = header::HeaderValue::from_str(&format!("Bearer {}", access_token))?;
    auth_header_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_header_value);

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .default_headers(headers)
        .build()?;

    let response = client.get("https://api.ouraring.com/v1/activity").send()?;
    println!("{:?}", response);

    Ok(())
}

fn main() {
    println!("Hello wellness world!");
    println!("{:?}", activity_bulletin());
}
