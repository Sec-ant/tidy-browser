use napi_derive::napi;
use serde::{Deserialize, Serialize};

mod browser_paths;
mod real_cookie_parser;

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
pub async fn get_chrome_cookies(domain: Option<String>) -> napi::Result<Vec<Cookie>> {
  real_cookie_parser::get_chrome_cookies_real(domain).await
}

#[napi]
pub async fn get_firefox_cookies(domain: Option<String>) -> napi::Result<Vec<Cookie>> {
  real_cookie_parser::get_firefox_cookies_real(domain).await
}

#[napi]
pub fn get_safari_cookies(_domain: Option<String>) -> napi::Result<Vec<Cookie>> {
  // Safari implementation would go here
  Ok(vec![])
}

#[napi]
pub async fn get_cookies(browser: String, domain: Option<String>) -> napi::Result<Vec<Cookie>> {
  match browser.to_lowercase().as_str() {
    "chrome" => get_chrome_cookies(domain).await,
    "firefox" => get_firefox_cookies(domain).await,
    "safari" => get_safari_cookies(domain),
    _ => Err(napi::Error::new(
      napi::Status::InvalidArg,
      format!("Unsupported browser: {}", browser),
    )),
  }
}