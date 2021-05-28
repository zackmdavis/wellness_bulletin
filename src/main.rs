use std::fs;
use std::time::Duration;

use reqwest::header;

mod activity;
mod sleep;

use activity::ActivityResponse;
use sleep::SleepResponse;


fn read_access_token() -> Result<String, std::io::Error> {
    fs::read_to_string(".oura_access_token")
}

fn oura_request(endpoint: &str) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
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

    let response = client.get(&format!("https://api.ouraring.com/v1/{}", endpoint)).send()?;
    Ok(response)
}

enum WellnessDomain {
    Activity,
    Sleep,
}


fn bulletin(wellness_domain: WellnessDomain) -> Result<String, Box<dyn std::error::Error>> {
    match wellness_domain {
        WellnessDomain::Activity => {
            let response = oura_request("activity")?;
            let activity_summaries = response.json::<ActivityResponse>()?;
            let latest = &activity_summaries.activity[activity_summaries.activity.len()-1];
            Ok(format!("Date: {}, Score: {}", latest.summary_date, latest.score))
        },
        WellnessDomain::Sleep => {
            let response = oura_request("sleep")?;
            let sleep_summaries = response.json::<SleepResponse>()?;
            let latest = &sleep_summaries.sleep[sleep_summaries.sleep.len()-1];
            Ok(format!("Date: {}, Score: {}", latest.summary_date, latest.score))
        },
    }
}

fn main() {
    println!("Hello wellness world!");
    println!("{:?}", bulletin(WellnessDomain::Activity));
}
