use std::{fs::{self, File}, path::{Path, PathBuf}, io::Read};

const MAX_FILE_SIZE: usize = 1024 * 1024 * 500; // 500 MB

#[derive(Clone)]
pub struct FileServer
{
    current_directory: String,
}

impl Default for FileServer
{
    fn default() -> Self
    {
        Self::new()
    }
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

    pub fn get_dir(&self, ) -> String
    {
        self.current_directory.clone()
    }

    /// Lists all folders and files in current directory
    /// # Returns
    /// * `Vec<String>` - Vector of names    
    pub fn list_content(&self) -> Result<Vec<String>, String>
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

        let separator = std::path::MAIN_SEPARATOR.to_string();

        for path in paths
        {
            let file_name = path.unwrap().file_name().into_string().unwrap();
            let file_path = format!("{}{}{}", &self.current_directory, separator, file_name);
            let file_path = Path::new(&file_path);
            if file_path.is_dir()
            {
                folders.push(format!("{}{}", separator, file_name));
            }
            else if file_path.is_file()
            {
                folders.push(file_name);
            }
        }

        Ok(folders)
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
        let mut tmp_folder = folder.to_string();

        //Check if the folder is a path
        if !tmp_folder.contains(std::path::MAIN_SEPARATOR.to_string().as_str())
        {
            tmp_folder = format!("{}{}", std::path::MAIN_SEPARATOR, tmp_folder);
        }    

        if self.current_directory.ends_with(std::path::MAIN_SEPARATOR.to_string().as_str())
        {
            tmp_folder = tmp_folder.replace(std::path::MAIN_SEPARATOR.to_string().as_str(), "");
        }

        let new_current_dir = format!("{}{}", &self.current_directory, tmp_folder);
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
        let mut separator = std::path::MAIN_SEPARATOR.to_string();
        if self.current_directory.ends_with(separator.as_str())
        {
            separator = "".to_string();
        }

        //get path to the file
        let file_path = format!("{}{}{}", self.current_directory, separator, file_name);

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
