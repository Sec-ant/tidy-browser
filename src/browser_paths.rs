use std::path::PathBuf;

// Chrome browser paths
#[cfg(target_os = "linux")]
pub fn chrome_path() -> Option<PathBuf> {
    dirs::home_dir().map(|p| p.join(".config/google-chrome"))
}

#[cfg(target_os = "macos")]
pub fn chrome_path() -> Option<PathBuf> {
    dirs::home_dir().map(|p| p.join("Library/Application Support/Google/Chrome"))
}

#[cfg(target_os = "windows")]
pub fn chrome_path() -> Option<PathBuf> {
    dirs::data_local_dir().map(|p| p.join("Google\\Chrome\\User Data"))
}

// Firefox browser paths
#[cfg(target_os = "linux")]
pub fn firefox_path() -> Option<PathBuf> {
    dirs::home_dir().map(|p| p.join(".mozilla/firefox"))
}

#[cfg(target_os = "macos")]
pub fn firefox_path() -> Option<PathBuf> {
    dirs::home_dir().map(|p| p.join("Library/Application Support/Firefox"))
}

#[cfg(target_os = "windows")]
pub fn firefox_path() -> Option<PathBuf> {
    dirs::data_dir().map(|p| p.join("Mozilla\\Firefox"))
}

// Safari browser paths (macOS only)
#[cfg(target_os = "macos")]
pub fn safari_path() -> Option<PathBuf> {
    dirs::home_dir().map(|p| p.join("Library/Cookies/Cookies.binarycookies"))
}