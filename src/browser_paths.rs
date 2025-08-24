use std::path::PathBuf;

// Chrome browser paths
#[cfg(target_os = "linux")]
pub fn chrome_path() -> Option<PathBuf> {
    // Allow override for testing
    if let Ok(test_path) = std::env::var("TEST_CHROME_PATH") {
        return Some(PathBuf::from(test_path));
    }
    dirs::home_dir().map(|p| p.join(".config/google-chrome"))
}

#[cfg(target_os = "macos")]
pub fn chrome_path() -> Option<PathBuf> {
    if let Ok(test_path) = std::env::var("TEST_CHROME_PATH") {
        return Some(PathBuf::from(test_path));
    }
    dirs::home_dir().map(|p| p.join("Library/Application Support/Google/Chrome"))
}

#[cfg(target_os = "windows")]
pub fn chrome_path() -> Option<PathBuf> {
    if let Ok(test_path) = std::env::var("TEST_CHROME_PATH") {
        return Some(PathBuf::from(test_path));
    }
    dirs::data_local_dir().map(|p| p.join("Google\\Chrome\\User Data"))
}

// Firefox browser paths
#[cfg(target_os = "linux")]
pub fn firefox_path() -> Option<PathBuf> {
    if let Ok(test_path) = std::env::var("TEST_FIREFOX_PATH") {
        return Some(PathBuf::from(test_path));
    }
    dirs::home_dir().map(|p| p.join(".mozilla/firefox"))
}

#[cfg(target_os = "macos")]
pub fn firefox_path() -> Option<PathBuf> {
    if let Ok(test_path) = std::env::var("TEST_FIREFOX_PATH") {
        return Some(PathBuf::from(test_path));
    }
    dirs::home_dir().map(|p| p.join("Library/Application Support/Firefox"))
}

#[cfg(target_os = "windows")]
pub fn firefox_path() -> Option<PathBuf> {
    if let Ok(test_path) = std::env::var("TEST_FIREFOX_PATH") {
        return Some(PathBuf::from(test_path));
    }
    dirs::data_dir().map(|p| p.join("Mozilla\\Firefox"))
}