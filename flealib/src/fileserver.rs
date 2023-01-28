use std::{fs::{self, File}, path::{Path, PathBuf}, io::Read};

const MAX_FILE_SIZE: usize = 1024 * 1024 * 500; // 500 MB

#[derive(Clone)]
pub struct FileServer
{
    current_directory: String,
}

impl FileServer
{
    pub fn new() -> Self
    {
        Self
        {
            current_directory: std::env::current_dir().unwrap().to_str().unwrap().to_string(),
        }
    }

    /// Lists all folders in current directory
    /// # Returns
    /// * `Vec<String>` - Vector of folder names    
    pub fn list_folders(&self) -> Result<Vec<String>, String>
    {
        let mut folders: Vec<String> = Vec::new();

        let paths = match fs::read_dir(&self.current_directory)
        {
            Ok(x) =>
            {
                x
            },
            Err(e) =>
            {
                return Err(e.to_string())
            }
        };

        for path in paths
        {
            let file_name = path.unwrap().file_name().into_string().unwrap();
            let file_path = format!("{}/{}", &self.current_directory, file_name);
            let file_path = Path::new(&file_path);
            if file_path.is_dir()
            {
                folders.push(format!("/{}", file_name));
            }
        }

        Ok(folders)
    }

    /// Lists all files and folders in a specified directory
    /// # Arguments
    /// * `path` - Path to the directory
    /// # Returns
    /// * `Vec<String>` - Vector of file names
    fn list_files(&self, path: &str) -> Result<Vec<String>, String>
    {
        let mut files: Vec<String> = Vec::new();

        let paths = match fs::read_dir(path)
        {
            Ok(x) =>
            {
                x
            },
            Err(e) =>
            {
                return Err(e.to_string())
            }
        };

        for path in paths
        {
            let file_name = path.unwrap().file_name().into_string().unwrap();
            files.push(file_name);
        }

        Ok(files)
    }

    /// Change a directory to one level up
    /// # Returns
    /// * `Result<(), String>` - Result of the operation
    pub fn change_directory_up(&mut self) -> Result<(), String>
    {
        let current_dir = Path::new(&self.current_directory);
        let parent_dir = match current_dir.parent()
        {
            Some(x) =>
            {
                x
            },
            None =>
            {
                return Err("Already at root".to_string())
            }
        };

        self.current_directory = parent_dir.to_str().unwrap().to_string();
        Ok(())
    }

    /// Change current directory to the new one
    /// # Arguments
    /// * 'folder' - New directory
    /// # Returns
    /// * `Result<(), String>` - Result of the operation
    pub fn change_directory(&mut self, folder: &str) -> Result<(), String>
    {
        let new_current_dir = format!("{}/{}", self.current_directory, folder);
        let new_dir = Path::new(&new_current_dir);
        match fs::metadata(new_dir)
        {
            Ok(x) =>
            {
                if x.is_dir()
                {
                    self.current_directory = new_dir.to_str().unwrap().to_string();
                    Ok(())
                }
                else
                {
                    Err("Not a directory".to_string())
                }
            },
            Err(e) =>
            {
                Err(e.to_string())
            }
        }
    }

    pub fn get_curr_dir_content(&self) -> Result<Vec<String>, String>
    {
        if let Ok(files) =self.list_files(&self.current_directory)
        {
            let mut folders = self.list_folders()?;
            folders.append(&mut files.clone());
            Ok(folders)
        }
        else
        {
            Err("Error".to_string())
        }
    }

    /// Reads a binary file knowing its path and returns its content as a u8 vector
    /// # Arguments
    /// * `file_path` - Path to the file as PathBuf
    /// # Returns
    /// * `Result<Vec<u8>, std::io::Error>` - Vector of u8 bytes or an error        
    pub fn read_binary_file_by_path(&self, file_path: &PathBuf) -> Result<Vec<u8>, std::io::Error>
    {
        //Get size of the file
        let file_size = fs::metadata(file_path.clone())?.len() as usize;
        if file_size > MAX_FILE_SIZE
        {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "File too large"));
        }
        let mut file = File::open(file_path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Ok(data)
    }

    /// Reads a binary file and returns its content as a u8 vector
    /// * file_name - a file name to read
    /// * returns a vector of u8 bytes or an error
    pub fn read_binary_file(&self, file_name: &str) -> Result<Vec<u8>, std::io::Error>
    {
        //get path to the file
        let file_path = format!("{}/{}", self.current_directory, file_name);

        //Get size of the file
        let file_size = fs::metadata(file_path.clone())?.len() as usize;
        if file_size > MAX_FILE_SIZE
        {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "File too large"));
        }
        let mut file = File::open(file_path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Ok(data)
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]    
    fn list_dir_test()
    {
        let file_server = FileServer::new();
        let current_dir = std::env::current_dir().unwrap();
        let files = file_server.list_files(&current_dir.to_str().unwrap()).unwrap();
        assert_ne!(files.len(), 0);
    }
}