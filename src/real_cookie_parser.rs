use crate::Cookie;
use chrono::DateTime;
use rusqlite::Connection;
use std::path::PathBuf;

pub async fn get_chrome_cookies_real(domain: Option<String>) -> napi::Result<Vec<Cookie>> {
    let chrome_path = match crate::browser_paths::chrome_path() {
        Some(path) => path,
        None => {
            println!("Chrome path not found");
            return Ok(vec![]);
        }
    };
    
    let cookies_db = chrome_path.join("Default/Cookies");
    
    if !cookies_db.exists() {
        println!("Chrome cookies database not found at: {}", cookies_db.display());
        return Ok(vec![]);
    }
    
    println!("Found Chrome cookies database at: {}", cookies_db.display());
    
    // Copy to temp file to avoid locking the browser's database
    let temp_db = std::env::temp_dir().join("chrome_cookies_temp.db");
    if let Err(e) = std::fs::copy(&cookies_db, &temp_db) {
        println!("Failed to copy Chrome database: {}", e);
        return Ok(vec![]);
    }
    
    // Open the SQLite database
    let conn = match Connection::open(&temp_db) {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to open Chrome database: {}", e);
            let _ = std::fs::remove_file(temp_db);
            return Ok(vec![]);
        }
    };
    
    let mut query = "SELECT host_key, name, value, path, expires_utc, is_secure, is_httponly FROM cookies".to_string();
    let mut params: Vec<&dyn rusqlite::ToSql> = vec![];
    
    if let Some(ref domain_filter) = domain {
        query.push_str(" WHERE host_key LIKE ?");
        params.push(domain_filter);
    }
    
    let mut stmt = match conn.prepare(&query) {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("Failed to prepare Chrome query: {}", e);
            let _ = std::fs::remove_file(temp_db);
            return Ok(vec![]);
        }
    };
    
    let cookie_iter = stmt.query_map(params.as_slice(), |row| {
        let expires_utc: i64 = row.get(4)?;
        let expires = if expires_utc > 0 {
            DateTime::from_timestamp(expires_utc / 1_000_000, 0)
                .map(|dt| dt.to_rfc3339())
        } else {
            None
        };
        
        Ok(Cookie {
            domain: row.get(0)?,
            name: row.get(1)?,
            value: row.get::<_, String>(2)?, // Note: might be encrypted
            path: Some(row.get(3)?),
            expires,
            secure: Some(row.get::<_, i32>(5)? != 0),
            http_only: Some(row.get::<_, i32>(6)? != 0),
        })
    });
    
    let mut cookies = vec![];
    if let Ok(iter) = cookie_iter {
        for cookie_result in iter {
            if let Ok(cookie) = cookie_result {
                cookies.push(cookie);
            }
        }
    }
    
    println!("Found {} Chrome cookies", cookies.len());
    
    // Clean up temp file
    let _ = std::fs::remove_file(temp_db);
    
    Ok(cookies)
}

pub async fn get_firefox_cookies_real(domain: Option<String>) -> napi::Result<Vec<Cookie>> {
    let firefox_path = match crate::browser_paths::firefox_path() {
        Some(path) => path,
        None => {
            println!("Firefox path not found");
            return Ok(vec![]);
        }
    };
    
    if !firefox_path.exists() {
        println!("Firefox path not found at: {}", firefox_path.display());
        return Ok(vec![]);
    }
    
    println!("Found Firefox path at: {}", firefox_path.display());
    
    // Find a profile directory (simplified approach)
    let profile_dirs: Vec<PathBuf> = std::fs::read_dir(&firefox_path)
        .map(|entries| {
            entries
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
                .filter(|entry| {
                    let file_name = entry.file_name();
                    let name = file_name.to_string_lossy();
                    name.contains("default") || name.ends_with(".default") || name.ends_with(".default-release")
                })
                .map(|entry| entry.path())
                .collect()
        })
        .unwrap_or_else(|_| vec![]);
    
    for profile_path in profile_dirs {
        let cookies_db = profile_path.join("cookies.sqlite");
        if !cookies_db.exists() {
            continue;
        }
        
        println!("Found Firefox cookies database at: {}", cookies_db.display());
        
        // Copy to temp file
        let temp_db = std::env::temp_dir().join("firefox_cookies_temp.db");
        if let Err(e) = std::fs::copy(&cookies_db, &temp_db) {
            println!("Failed to copy Firefox database: {}", e);
            continue;
        }
        
        let conn = match Connection::open(&temp_db) {
            Ok(conn) => conn,
            Err(e) => {
                println!("Failed to open Firefox database: {}", e);
                let _ = std::fs::remove_file(temp_db);
                continue;
            }
        };
        
        let mut query = "SELECT host, name, value, path, expiry, isSecure, isHttpOnly FROM moz_cookies".to_string();
        let mut params: Vec<&dyn rusqlite::ToSql> = vec![];
        
        if let Some(ref domain_filter) = domain {
            query.push_str(" WHERE host LIKE ?");
            params.push(domain_filter);
        }
        
        let mut stmt = match conn.prepare(&query) {
            Ok(stmt) => stmt,
            Err(e) => {
                println!("Failed to prepare Firefox query: {}", e);
                let _ = std::fs::remove_file(temp_db);
                continue;
            }
        };
        
        let cookie_iter = stmt.query_map(params.as_slice(), |row| {
            let expiry: Option<i64> = row.get(4)?;
            let expires = expiry
                .and_then(|ts| DateTime::from_timestamp(ts, 0))
                .map(|dt| dt.to_rfc3339());
            
            Ok(Cookie {
                domain: row.get(0)?,
                name: row.get(1)?,
                value: row.get(2)?,
                path: Some(row.get(3)?),
                expires,
                secure: Some(row.get::<_, i32>(5)? != 0),
                http_only: Some(row.get::<_, i32>(6)? != 0),
            })
        });
        
        let mut cookies = vec![];
        if let Ok(iter) = cookie_iter {
            for cookie_result in iter {
                if let Ok(cookie) = cookie_result {
                    cookies.push(cookie);
                }
            }
        }
        
        println!("Found {} Firefox cookies", cookies.len());
        
        // Clean up temp file
        let _ = std::fs::remove_file(temp_db);
        
        return Ok(cookies);
    }
    
    println!("No Firefox profile with cookies found");
    Ok(vec![])
}