use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::str::from_utf8;
use log::{debug, error};

const MAX_INPUT_BUFFER: usize = 1024;
const SPECIAL_COMMAND_GET_SCREENSHOT: &str = "screenshot";

pub struct FleaClient
{    
}

impl FleaClient
{
    pub fn send_command(&self, address: &str, xml: &str, cmd: &str) -> bool
    {
        debug!("Connecting to {} ..", address);                

        match TcpStream::connect(address) 
        {
            Ok(mut stream) => 
            {
                debug!("Connected, sent: {}", xml);
                debug!("Waiting for response..");

                stream.write(xml.as_bytes()).unwrap();
    
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
                if cmd.eq(SPECIAL_COMMAND_GET_SCREENSHOT)
                {
                    ret_value = self.bytes_to_file("screenshot.png", &self.digits_to_bytes(&read_string));
                    println!("File screenshot.png was saved successfully");
                }
                else
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

    /// Converts string digits into a byte array
    /// * digits - a string of digits
    /// Returns a byte array
    /// Example: "123456" -> [0x12, 0x34, 0x56]
    /// Note: the string must contain an even number of digits
    fn digits_to_bytes(&self, digits: &str) -> Vec<u8>
    {
        let mut bytes = Vec::<u8>::new();
        let mut i = 0;
        let count = digits.len();

        while i < count
        {
            let byte = u8::from_str_radix(&digits[i..i+2], 16).unwrap();
            bytes.push(byte);
            i += 2;
        }

        bytes
    }

    ///Saves byte array to the specified file
    /// * file_name - a file name to save
    /// * data - a byte array to save
    /// Returns true if the file was saved successfully
    fn bytes_to_file(&self, file_name: &str, data: &[u8]) -> bool
    {
        let mut file = match std::fs::File::create(file_name)
        {
            Ok(f) => f,
            Err(e) =>
            {
                error!("Couldn't create file: {}", e);
                return false;
            }
        };

        match file.write_all(data)
        {
            Ok(_) => {},
            Err(e) =>
            {
                error!("Couldn't write to file: {}", e);
                return false;
            }
        }

        true
    }
}