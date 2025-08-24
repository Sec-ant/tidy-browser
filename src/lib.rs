use napi_derive::napi;
use serde::{Deserialize, Serialize};

mod browser_paths;
mod cookie_parser;

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cookie {
  pub domain: String,
  pub name: String,
  pub value: String,
  pub path: Option<String>,
  pub expires: Option<String>, // ISO 8601 string
  pub secure: Option<bool>,
  pub http_only: Option<bool>,
}

#[napi]
pub fn get_chrome_cookies(domain: Option<String>) -> napi::Result<Vec<Cookie>> {
  println!("get_chrome_cookies called with domain: {:?}", domain);
  let mock_cookie = Cookie {
      domain: domain.unwrap_or_else(|| "example.com".to_string()),
      name: "test".to_string(),
      value: "mock_value".to_string(),
      path: Some("/".to_string()),
      expires: Some("2024-12-31T23:59:59Z".to_string()),
      secure: Some(true),
      http_only: Some(false),
  };
  
  Ok(vec![mock_cookie])
}

#[napi]
pub fn get_firefox_cookies(domain: Option<String>) -> napi::Result<Vec<Cookie>> {
  println!("get_firefox_cookies called with domain: {:?}", domain);
  let mock_cookie = Cookie {
      domain: domain.unwrap_or_else(|| "firefox.com".to_string()),
      name: "firefox_test".to_string(),
      value: "firefox_value".to_string(),
      path: Some("/".to_string()),
      expires: None,
      secure: Some(false),
      http_only: Some(true),
  };
  
  Ok(vec![mock_cookie])
}

#[napi]
pub fn get_safari_cookies(_domain: Option<String>) -> napi::Result<Vec<Cookie>> {
  Ok(vec![])
}

#[napi]
pub fn get_cookies(browser: String, domain: Option<String>) -> napi::Result<Vec<Cookie>> {
  println!("get_cookies called with browser: {}, domain: {:?}", browser, domain);
  match browser.to_lowercase().as_str() {
    "chrome" => get_chrome_cookies(domain),
    "firefox" => get_firefox_cookies(domain),
    "safari" => get_safari_cookies(domain),
    _ => Err(napi::Error::new(
      napi::Status::InvalidArg,
      format!("Unsupported browser: {}", browser),
    )),
  }
}