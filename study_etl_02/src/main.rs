// get api token from file
use std::env;
use dotenv::dotenv;
use urlencoding::encode;
use reqwest::blocking::Client;
use reqwest::blocking::Response;
use reqwest::header::HeaderMap;
use once_cell::sync::Lazy;

// typedefs
use reqwest::StatusCode;
use std::error::Error;

// env variables import
// 가져올 환경변수의 타입 정의
struct NexonApiVars {
    api_url_home: String,
    test: String,
    live: String,
}

static NEXON_API_VARS: Lazy<NexonApiVars> = Lazy::new(|| {
    dotenv().ok();
    NexonApiVars {
        api_url_home: env::var("NEXON_API_HOME").expect("Missing NEXON_API_HOME"),
        test: env::var("NEXON_API_TOKEN_TEST").expect("Missing NEXON_API_TOKEN_TEST"),
        live: env::var("NEXON_API_TOKEN_LIVE").expect("Missing NEXON_API_TOKEN_LIVE"),
    }
});

// String(str) 과 &str의 차이: 간단하게 가변여부
// 캐릭터 이름을 입력받아 OCID를 가져오는 함수
fn get_ocid(
        character_name: &str
    ) -> Result<String, Box<dyn Error>> {

    // 요청정보 생성
    let api_service_url: &str = "/maplestory/v1/id";
    let api_param: String = format!(
        "{}{}?character_name={}",
        NEXON_API_VARS.api_url_home.as_str(),
        api_service_url,
        encode(character_name)
    );

    // request 생성
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(
        "x-nxopen-api-key",
        NEXON_API_VARS.test.parse()?
    );
    let res_client: Client = Client::new();
    let response: Response = res_client
        .get(&api_param)
        .headers(headers)
        .send()?;
    let status: StatusCode = response.status();
    let status_code: &str = status.as_str();
    let body: String = response.text()?;
    println!("Response Status: {}", status_code);
    println!("Response Body: {}", body);

    // 응답 처리
    if status.is_success() {
        let json: serde_json::Value = serde_json::from_str(&body)?;
        if let Some(ocid) = json.get("ocid") {
            Ok(ocid.as_str().unwrap_or("").to_string())
        } else {
            Err(format!("OCID not found in response: {}", body).into())
        }
    } else {
        Err(format!("[{}] Failed to get OCID: {}", status_code, body).into())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let req_character_name: &'static str = "마법사악";

    // OCID 가져오기
    match get_ocid(req_character_name) {
        Ok(ocid) => println!("OCID for {}: {}", req_character_name, ocid),
        Err(e) => eprintln!("Error fetching OCID: {}", e),
    }
    Ok(())
}