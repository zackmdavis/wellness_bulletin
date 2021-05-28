use std::fs;
use std::time::Duration;

use reqwest::header;
use egg_mode::tweet::DraftTweet;

mod activity;
mod sleep;

use activity::ActivityResponse;
use sleep::SleepResponse;


fn read_oura_access_token() -> Result<String, std::io::Error> {
    fs::read_to_string(".oura_access_token")
}

fn read_secret_file_data(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string(filename)?;
    Ok(file_content.trim_end().to_string())
}

fn construct_twitter_token() -> Result<egg_mode::Token, Box<dyn std::error::Error>> {
    let consumer_key = read_secret_file_data(".twitter_consumer_key")?;
    let consumer_secret = read_secret_file_data(".twitter_consumer_secret")?;
    let access_token_key = read_secret_file_data(".twitter_access_token_key")?;
    let access_token_secret = read_secret_file_data(".twitter_access_token_secret")?;
    let consumer_token = egg_mode::KeyPair::new(consumer_key, consumer_secret);
    let access_token = egg_mode::KeyPair::new(access_token_key, access_token_secret);
    let token = egg_mode::Token::Access {
        consumer: consumer_token,
        access: access_token,
    };
    Ok(token)
}


fn oura_request(endpoint: &str) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
    let token_file_content = read_oura_access_token()?;
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

#[tokio::main]
async fn main() {
    println!("Hello wellness world!");
    let token = construct_twitter_token().expect("I hope this works");
    let tweet = DraftTweet::new("Hello egg-mode World!");
    tweet.send(&token).await.unwrap();
}
