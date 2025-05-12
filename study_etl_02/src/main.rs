// get api token from file
use std::env;
use dotenv::dotenv;
use urlencoding::encode;
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    // get api token from file
    dotenv().ok();
    let test_token = env::var("NEXON_API_TOKEN_TEST")?;
    let api_server_url = "https://open.api.nexon.com";
    let api_service_url = "/maplestory/v1/id"; 
    let req_character_name = "마법사악";

    // get character id
    let req_character_name_encode = encode(req_character_name);
    println!("req_character_name_encode: {}", req_character_name_encode);
    let request_url = format!(
        "{}{}?character_name={}", api_server_url, api_service_url, req_character_name_encode
    );
    let mut headers = HeaderMap::new();
    headers.insert("x-nxopen-api-key", test_token.parse()?);

    let res_client = Client::new();
    let response = res_client
        .get(&request_url)
        .headers(headers)
        .send()?;
    let status = response.status();
    let body = response.text()?;

    println!("Status: {}", status);
    println!("Body: {}", body);

    Ok(())
}