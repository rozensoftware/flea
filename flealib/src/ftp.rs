use std::{path::PathBuf, fs::File, io::{Read, Cursor, Write}};

use ftp::{FtpStream, FtpError};
use log::{debug, error};

const FTP_STD_PORT: u16 = 21;

pub struct FTP
{
    current_directory: PathBuf,
}

impl FTP
{
    pub fn new(dir: PathBuf) -> FTP
    {
        FTP
        {
            current_directory: dir,
        }
    }

    /// Reads a file and store its content in a vec
    /// * file_path - a file with the absolute path to read from
    fn read_file_to_vec(&self, file_path: &str) -> std::io::Result<Vec<u8>> 
    {
        let mut file = File::open(&file_path)?;
    
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
    
        return Ok(data);
    }

    /// Writes data to a file on disk
    /// * file_name - a path with a name where the data will be written to
    /// * data - array od u8 bytes to save in a file
    fn write_file(&self, file_name: PathBuf, data: Vec<u8>) -> std::io::Result<()>
    {
        let mut file = File::create(file_name)?;

        file.write_all(&data)?;

        Ok(())
    }

    /// Receives a file from remote FTP server
    /// * addr - an FTP server address
    /// * user - login name
    /// * pass - password
    /// * file_name - a file name to download from FTP server
    pub fn receive_file_from_ftp(&self, addr: &str, user: &str, pass: &str, file_name: &str, ftp_folder: &str) -> Result<(), FtpError>
    {
        let mut ftp_stream = FtpStream::connect((addr, FTP_STD_PORT))?;

        ftp_stream.login(user, pass)?;
    
        debug!("Connected to FTP server.");

        ftp_stream.cwd(ftp_folder)?;

        match ftp_stream.simple_retr(file_name)
        {
            Ok(x) =>
            {
                let file_path = self.current_directory.join(file_name);
                match self.write_file(file_path, x.into_inner())
                {
                    Ok(_) =>
                    {
                        debug!("File received from FTP server");
                    },
                    Err(x) =>
                    {
                        error!("Couldn't write a file to disk:{}", x.to_string());                
                        ftp_stream.quit()?;
                        return Err(FtpError::InvalidResponse(x.to_string()))        
                    }
                }
                
            },
            Err(y) =>
            {
                error!("Couldn't receive the file from FTP server:{}", y.to_string());                
                ftp_stream.quit()?;
                return Err(FtpError::InvalidResponse(y.to_string()))
            }
        }

        ftp_stream.quit()
    }

    /// Sends a file to remote FTP server
    /// * addr - an FTP server address
    /// * user - login name
    /// * pass - password
    /// * file_path - a path to the file to be sent
    /// * ftp_folder - a folder on FTP server to store the file
    pub fn send_file_to_ftp(&self, addr: &str, user: &str, pass: &str, file_path: &PathBuf, ftp_folder: &str) -> Result<(), FtpError>
    {
        let mut ftp_stream = FtpStream::connect((addr, FTP_STD_PORT))?;

        ftp_stream.login(user, pass)?;
    
        debug!("Connected to FTP server.");

        ftp_stream.cwd(ftp_folder)?;

        // Store a file
        match self.read_file_to_vec(file_path.to_str().unwrap())
        {
            Ok(file_data) =>
            {
                let mut reader = Cursor::new(file_data);
                ftp_stream.put(file_path.file_name().unwrap().to_str().unwrap(), &mut reader)?;
                debug!("File uploaded to FTP server.")
            },
            Err(x) =>
            {
                error!("Couldn't upload the file to FTP server:{}", x.to_string());                
                ftp_stream.quit()?;
                return Err(FtpError::InvalidResponse(x.to_string()))
            }
        };

        ftp_stream.quit()
    }
}