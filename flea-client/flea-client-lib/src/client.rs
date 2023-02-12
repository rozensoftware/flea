use std::thread;
use std::io::{Read, Write, self};
use std::net::{Shutdown, TcpStream};
use std::str::from_utf8;
use log::{debug, error};

const MAX_INPUT_BUFFER: usize = 1024;
const SPECIAL_COMMAND_GET_SCREENSHOT: &str = "screenshot";
const SPECIAL_COMMAND_GET_FILE: &str = "getfile";

pub struct FleaClient
{    
}

impl FleaClient
{
    pub fn send_command(&self, address: &str, xml: &str, cmd: &str, value: &str) -> bool
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
                let mut bytes_read = 0;
                const DATA_10MB: usize = 10 * 1024 * 1024;

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
                                    bytes_read += data_len;
                                    if bytes_read >= DATA_10MB
                                    {
                                        print!(".");
                                        bytes_read = 0;
                                    }
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
                        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => 
                        {
                            thread::sleep(std::time::Duration::from_millis(100));
                            continue;
                        }
                        Err(e) => 
                        {
                            error!("Couldn't read data: {:?}", e);
                            ret_value = false;
                            break;
                        }
                    }                        
                }
                
                stream.shutdown(Shutdown::Both).unwrap();

                if read_string.len() == 0
                {
                    error!("No data received from the Flea Server");
                    ret_value = false;
                }
                else
                {
                    if cmd.eq(SPECIAL_COMMAND_GET_SCREENSHOT)
                    {
                        if let Ok(bytes) = &self.digits_to_bytes(&read_string)
                        {
                            if let Err(e) = self.bytes_to_file("screenshot.png", &bytes)
                            {
                                error!("Couldn't save screenshot: {}", e);
                                ret_value = false;
                            }
                            else
                            {
                                println!("File screenshot.png was saved successfully");
                            }
                        }
                        else
                        {
                            error!("Couldn't convert screenshot data.");
                            ret_value = false;
                        }                           
                    }
                    else if cmd.eq(SPECIAL_COMMAND_GET_FILE)
                    {
                        if let Ok(bytes) = &self.digits_to_bytes(&read_string)
                        {
                            if let Err(e) = self.bytes_to_file(value, &bytes)
                            {
                                error!("Couldn't save file: {} - {}", value, e);
                                ret_value = false;
                            }
                            else
                            {
                                println!("File {} was saved successfully", value);
                            }
                        }
                        else
                        {
                            error!("Couldn't convert file data");
                            ret_value = false;
                        }                           
                    }
                    else
                    {
                        println!("Response from the Flea Server is:");
                        println!("{}", read_string);    
                    }                    
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
    fn digits_to_bytes(&self, digits: &str) -> Result<Vec<u8>, String>
    {
        let mut bytes = Vec::<u8>::new();
        let mut i = 0;
        let count = digits.len();

        while i < count
        {
            if let Ok(byte) = u8::from_str_radix(&digits[i..i+2], 16)
            {
                bytes.push(byte);
                i += 2;    
            }
            else
            {
                return Err("Invalid digit".to_string());
            }
        }

        Ok(bytes)
    }

    ///Saves byte array to the specified file
    /// * file_name - a file name to save
    /// * data - a byte array to save
    /// Returns true if the file was saved successfully
    fn bytes_to_file(&self, file_name: &str, data: &[u8]) -> io::Result<()>
    {
        let mut file = std::fs::File::create(file_name)?;
        file.write_all(data)?;
        Ok(())
    }
}