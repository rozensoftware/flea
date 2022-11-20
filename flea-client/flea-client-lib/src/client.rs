use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::str::from_utf8;
use log::{debug, error};

const MAX_INPUT_BUFFER: usize = 1024;

pub struct FleaClient
{    
}

impl FleaClient
{
    pub fn send_command(&self, address: &str, cmd: &str) -> bool
    {
        debug!("Connecting to {} ..", address);                

        match TcpStream::connect(address) 
        {
            Ok(mut stream) => 
            {
                debug!("Connected, trying to read data..");

                stream.write(cmd.as_bytes()).unwrap();
    
                let mut ret_value = true;
                let mut data = [0 as u8; MAX_INPUT_BUFFER];
                let mut read_string: String = "".to_string();

                loop 
                {
                    match stream.read(&mut data)
                    {
                        Ok(data_len) => 
                        {
                            if data_len == 0
                            {
                                break;
                            }

                            match from_utf8(&data[..data_len])
                            {
                                Ok(str) =>
                                {
                                    read_string.push_str(str);
                                },
                                Err(e) =>
                                {
                                    error!("Couldn't read data (bad data, not a string): {:?}", e);
                                    ret_value = false;
                                    break;
                                }
                            }
                        },
                        Err(e) => 
                        {
                            error!("Couldn't read data: {:?}", e);
                            ret_value = false;
                            break;
                        }
                    }                        
                }
                
                stream.shutdown(Shutdown::Both).unwrap();
                if ret_value
                {
                    println!("Response from the Flea Server is:");
                    println!("{}", read_string);
                }
                ret_value
            },
            Err(e) => 
            {
                error!("Couldn't connect: {:?}", e);
                false
            }
        }    
    }
}