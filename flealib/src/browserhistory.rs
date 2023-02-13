extern crate ini;

use std::fmt;
use rusqlite::{Connection, Result};
use std::env;
use ini::Ini;

type SqliteResult<T> = Result<T>;

const URL_SELECT: &'static str = "SELECT url, title, visit_count, last_visit_time FROM urls ORDER BY last_visit_time DESC";
const FIREFOX_URL_SELECT: &'static str = "SELECT url, IFNULL(title, ''), visit_count, IFNULL(last_visit_date, 0) FROM moz_places ORDER BY last_visit_date DESC";
#[cfg(target_os = "windows")]
const CHROME_HISTORY_PATH: &'static str = "\\Google\\Chrome\\User Data\\Default\\History";
#[cfg(target_os = "windows")]
const EDGE_HISTORY_PATH: &'static str = "\\Microsoft\\Edge\\User Data\\Default\\History";
#[cfg(target_os = "windows")]
const HISTORY_FLEA_FOLDER_NAME: &'static str = "\\flea-tmp\\";

struct History 
{
    url: String,
    title: String,
    visit_count: i32,
    #[allow(dead_code)]
    last_visit_time: i64,
}

impl fmt::Display for History
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        write!(f, "URL:{};TITLE:{};VISIT COUNT:{}\r\n", &self.url, &self.title, &self.visit_count)
    }
}

fn get_history(history_path: &str, sql_query: &str) -> SqliteResult<Vec<String>> 
{
    let conn = Connection::open(history_path)?;
    let mut stmt = conn.prepare(sql_query)?;
    let url_iter = stmt.query_map([], |row| {
        Ok(History {
            url: row.get(0)?,
            title: row.get(1)?,
            visit_count: row.get(2)?,
            last_visit_time: row.get(3)?,
        })
    })?;

    let mut history = Vec::new();
    url_iter.into_iter().for_each(|f|history.push(format!("{}", f.unwrap())));

    Ok(history)
}

/// Returns browsing history
/// # Examples
/// ```
/// let history = get_browsing_history();
/// ```
/// # Platform-specific behavior
/// This function is only available on Windows and Linux
/// # Errors
/// This function returns an empty vector if the history file is not found
#[cfg(target_os = "windows")]
pub fn get_browsing_history() -> SqliteResult<Vec<String>> 
{
    let mut path = env::var("LOCALAPPDATA").unwrap();
    path.push_str(CHROME_HISTORY_PATH);
    
    let mut ret_history = Vec::new(); 

    //Copy history file to user temp directory
    let mut temp_path = env::var("TEMP").unwrap();
    temp_path.push_str(HISTORY_FLEA_FOLDER_NAME);
    std::fs::create_dir_all(&temp_path).unwrap();

    let temp_path2 = temp_path.clone();

    if std::path::Path::new(&path).exists()
    {
        temp_path.push_str("chrome_history");
        std::fs::copy(&path, &temp_path).unwrap();
        path = temp_path;

        ret_history = get_history(&path, URL_SELECT)?;
    }

    path = env::var("LOCALAPPDATA").unwrap();
    path.push_str(EDGE_HISTORY_PATH);
    
    if std::path::Path::new(&path).exists()
    {
        temp_path = temp_path2;

        temp_path.push_str("edge_history");
        std::fs::copy(&path, &temp_path).unwrap();
        path = temp_path;

        let mut v = get_history(&path, URL_SELECT)?;
        ret_history.append(&mut v);
    }

    let mut v = get_firefox_history()?;
    ret_history.append(&mut v);

    Ok(ret_history)
}

#[cfg(target_os = "linux")]
pub fn get_browsing_history() -> SqliteResult<Vec<String>> 
{
    let mut path = env::var("HOME").unwrap();
    path.push_str("/.config/google-chrome/Default/History");

    let mut ret = Vec::new();

    if std::path::Path::new(&path).exists()
    {
        let mut v = get_history(&path, URL_SELECT)?;
        ret.append(&mut v);
    }

    // path = env::var("HOME").unwrap();
    // path.push_str("/.config/chromium/Default/History");

    // if std::path::Path::new(&path).exists()
    // {
    //     let mut v = get_history(&path, URL_SELECT)?;
    //     ret.append(&mut v);
    // }

    // path = env::var("HOME").unwrap();
    // path.push_str("/.config/brave/Default/History");

    // if std::path::Path::new(&path).exists()
    // {
    //     let mut v = get_history(&path, URL_SELECT)?;
    //     ret.append(&mut v);
    // }

    let mut v = get_firefox_history()?;
    ret.append(&mut v);

    Ok(ret)
}

/// Returns browsing history of Firefox
/// # Examples
/// ```
/// let history = get_firefox_history();
/// ```
/// # Platform-specific behavior
/// This function is only available on Windows and Linux
/// # Errors
/// This function returns an empty vector if the history file is not found
/// # Safety
/// This function is unsafe because it uses unsafe code
/// This function is only available on Windows and Linux
#[cfg(target_os = "windows")]
pub fn get_firefox_history() -> SqliteResult<Vec<String>> 
{
    let mut path = env::var("APPDATA").unwrap();
    path.push_str("\\Mozilla\\Firefox\\Profiles\\");
    let mut profile_path = path.clone();
    profile_path.push_str("profiles.ini");

    if std::path::Path::new(&profile_path).exists()
    {
        let conf = Ini::load_from_file(profile_path).unwrap();

        let profile_dir = conf.section(Some("Profile0").to_owned()).unwrap().get("Path").unwrap();
        path.push_str(profile_dir);
        path.push_str("\\places.sqlite");

        let mut temp_path = env::var("APPDATA").unwrap();
        temp_path.push_str(HISTORY_FLEA_FOLDER_NAME);
        std::fs::create_dir_all(&temp_path).unwrap();
    
        temp_path.push_str("firefox_history");
        std::fs::copy(&path, &temp_path).unwrap();
        path = temp_path;
    
        return Ok(get_history(&path, FIREFOX_URL_SELECT)?);    
    }

    Ok(Vec::new())
}

/// Returns browsing history of Firefox in Linux
/// # Examples
/// ```
/// let history = get_firefox_history();
/// ```
#[cfg(target_os = "linux")]
pub fn get_firefox_history() -> SqliteResult<Vec<String>> 
{
    const HISTORY_FLEA_LINUX_FOLDER_NAME: &'static str = "/flea-tmp/";
    const FIREFOX_PROFILE_FILENAME: &'static str = "profiles.ini";

    let mut path = env::var("HOME").unwrap();
    path.push_str("/.mozilla/firefox/");
    let mut profile_path = path.clone();
    profile_path.push_str(FIREFOX_PROFILE_FILENAME);

    if std::path::Path::new(&profile_path).exists()
    {
        let conf = Ini::load_from_file(profile_path).unwrap();

        let profile_dir = conf.section(Some("Profile0").to_owned()).unwrap().get("Path").unwrap();
        path.push_str(profile_dir);
        path.push_str("/places.sqlite");

        let mut temp_path = env::var("HOME").unwrap();
        temp_path.push_str(HISTORY_FLEA_LINUX_FOLDER_NAME);
        std::fs::create_dir_all(&temp_path).unwrap();
    
        temp_path.push_str("firefox_history");
        std::fs::copy(&path, &temp_path).unwrap();
        path = temp_path;
    
        return Ok(get_history(&path, FIREFOX_URL_SELECT)?);    
    }

    Ok(Vec::new())
}
