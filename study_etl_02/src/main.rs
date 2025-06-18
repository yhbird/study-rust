// get api token from file
use std::env;
use dotenv::dotenv;
use urlencoding::encode;
use reqwest::blocking::Client;
use reqwest::blocking::Response;
use reqwest::header::HeaderMap;
use once_cell::sync::Lazy;
use chrono::{DateTime,Utc,FixedOffset,Datelike};
use num_format::{Locale,ToFormattedString};

// typedefs
use reqwest::StatusCode;
use std::error::Error;

// default constants
const DEFAULT_STRING: &str = "Unknown";
const DEFAULT_DATETIME_UTC: &str = "1970-01-01T00:00:00Z";

// env variables import
// 가져올 환경변수의 타입 정의
struct NexonApiVars {
    api_url_home: String,
    test: String,
    // live: String,
}

static NEXON_API_VARS: Lazy<NexonApiVars> = Lazy::new(|| {
    dotenv().ok();
    NexonApiVars {
        api_url_home: env::var("NEXON_API_HOME").expect("Missing NEXON_API_HOME"),
        test: env::var("NEXON_API_TOKEN_TEST").expect("Missing NEXON_API_TOKEN_TEST"),
        // live: env::var("NEXON_API_TOKEN_LIVE").expect("Missing NEXON_API_TOKEN_LIVE"),
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

// 캐릭터의 OCID를 통해 기본 정보 가져오기
fn get_character_info(ocid: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    // 요청정보 검증 및 생성
    if ocid.is_empty() {
        return Err("OCID is empty".into());
    }
    let api_service_url: &str = "/maplestory/v1/character/basic";
    let api_param: String = format!(
        "{}{}?ocid={}",
        NEXON_API_VARS.api_url_home.as_str(),
        api_service_url,
        ocid
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

    // 응답 처리
    if status.is_success() {
        let json: serde_json::Value = serde_json::from_str(&body)?;
        Ok(json)
    } else {
        Err(format!("[{}] Failed to get character info: {}", status_code, body).into())
    }
}

fn default_datetime_utc() -> DateTime<Utc> {
    // 기본 날짜: 1970-01-01T00:00:00Z
    DateTime::parse_from_rfc3339(DEFAULT_DATETIME_UTC)
        .unwrap_or_else(|_| DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z").unwrap())
        .with_timezone(&Utc)
}

// 날짜를 보기 쉽게 포맷팅 (년, 월, 일)
fn format_datetime_utc(dt: DateTime<Utc>) -> Result<String, Box<dyn Error>> {
    let formatted_str: String = format!(
        "{}년 {}월 {}일",
        dt.year(),
        dt.month(),
        dt.day()
    ).to_string();
    Ok(formatted_str)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let req_character_name: &'static str = "마법사악";

    // OCID 가져오기
    let ocid = get_ocid(req_character_name)?;

    // OCID를 통해 캐릭터 basic 정보 가져오기
    let character_basic_info = get_character_info(ocid.as_str())?;
    
    // 캐릭터 basic 정보 전처리
    // <basic> 캐릭터 이름
    let character_name: String = character_basic_info
        .get("character_name")
        .and_then(|v| v.as_str())
        .unwrap_or(DEFAULT_STRING)
        .to_string();

    // <basic> 캐릭터 생성 날짜 -> String
    // example: "2023-12-21T00:00+09:00" -> "2023-12-21T00:00:00+09:00"
    let character_date_create_str: String = character_basic_info
        .get("character_date_create")
        .and_then(|v| v.as_str())
        .map(|s| {
            if s.contains('T') && s.contains('+') {
                s.replacen("T00:00", "T00:00:00", 1)
            } else {
                s.to_string()
            }
        })
        .unwrap_or(DEFAULT_STRING.to_string());
    // <basic> 캐릭터 생성 날짜 -> DateTime<Utc>
    // if "Unknown" or empty, use default(1970-01-01T00:00:00Z)
    let character_date_create_time: DateTime<Utc> = if character_date_create_str == DEFAULT_STRING {
        default_datetime_utc()
    } else {
        character_date_create_str
            .parse::<DateTime<FixedOffset>>()
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| default_datetime_utc()) // if parse fails, use default
    };

    // <basic> 캐릭터 레벨, 직업, 직업 레벨(N차전직)
    let character_level: u16 = character_basic_info
        .get("character_level")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u16;
    let character_class: String = character_basic_info
        .get("character_class")
        .and_then(|v| v.as_str())
        .unwrap_or(DEFAULT_STRING)
        .to_string();
    let character_class_level: String = character_basic_info
        .get("character_class_level")
        .and_then(|v| v.as_str())
        .unwrap_or("0")
        .to_string();

    // <basic> 캐릭터 길드 정보
    // 길드 이름이 없을 경우 "없음"으로 처리
    let character_guild_name: String = character_basic_info
        .get("character_guild_name")
        .and_then(|v| v.as_str())
        .unwrap_or("없음")
        .to_string();

    // <basic> 캐릭터 경험치, 경험치 비율
    let character_exp: u128 = character_basic_info
        .get("character_exp")
        .and_then(|v| v.as_u64())
        .map(|v| v as u128)
        .unwrap_or(0);
    // 캐릭터 경험치 표시: 1,000 이상은 천 단위로 포맷팅
    let character_exp_str: String = match character_exp {
        val if val >= 1_000 => (val as u64).to_formatted_string(&Locale::en),
        val => val.to_string()
    };
    let character_exp_rate: String = character_basic_info
        .get("character_exp_rate")
        .and_then(|v| v.as_str())
        .unwrap_or("0.0")
        .to_string();

    // 캐릭터 정보 출력

    // 날짜를 보기 쉽게 포맷팅 (년, 월, 일)
    println!("캐릭터 이름: {}", character_name);
    println!("생성 날짜: {}", format_datetime_utc(character_date_create_time)?);
    println!("레벨: {} ({}%)", character_level, character_exp_rate);
    println!("직업: {} ({}차전직)", character_class, character_class_level);
    println!("길드: {}", character_guild_name);
    println!("경험치: {}", character_exp_str);
    Ok(())
}