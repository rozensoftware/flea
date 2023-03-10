use std::path::PathBuf;

/// Finds if there is a given file in the passed directory
/// * dir - a directory to search in
/// * file_name - a file name to search for
/// Returns a path to the file if found, None otherwise
pub(crate) fn find_update(dir: &str, file_name: &str) -> Option<String>
{
    let mut path = PathBuf::from(dir);
    path.push(file_name);
    if path.exists()
    {
        Some(path.to_str().unwrap().to_string())
    }
    else
    {
        None
    }
}

/// Starts a new process of itself
pub(crate) fn start_new_process(dir: &String, exe: String)
{
    let mut path = PathBuf::from(dir);
    path.push(exe);
    let info = format!("Starting a new process (path={})", &path.to_str().unwrap());
    std::process::Command::new(path)
        .spawn()
        .expect(&info);
}
