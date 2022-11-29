use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str;
use log::{debug, error};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::commandparser::CommandParser;
use crate::commandprocessor::{CommandProcessor, FleaCommand};

const MAX_READ_BUFFER_SIZE: usize = 1024;

/// Returns vector of characters without the ending line codes
///
/// * str - An input string having end line special characters 0x0d, 0x0a
fn remove_newline_characters(str: &str) -> Vec<u8>
{
    let s1 = str.to_string().replace("\n", "");
    let s2 = s1.replace("\r", "");
    let temp_str = s2.as_bytes();
    let end_str_idx = temp_str.iter().position(|&x| x == 0).unwrap();
    temp_str[..end_str_idx].to_vec()
}

/// Sends response to client
/// * 'stream' - Stream connection to the client
/// * 'command_name' - A command name
/// * 'value_name' - A command value
fn replay(mut stream: &TcpStream, command_name: String, value_name: String) -> bool
{
    debug!("Received command: {} with value: {}", command_name, value_name);

    let command_processor: CommandProcessor = FleaCommand::new();
    let mut b = true;
    let cmd = command_processor.process(&command_name.as_str(), &value_name.as_str());
    let mut data_idx = 0;

    loop 
    {
        match stream.write(&cmd[data_idx..].as_bytes())
        {
            Ok(sent_bytes) =>
            {
                data_idx += sent_bytes;
                if data_idx == cmd.len() 
                {
                    break;
                }        
            },
            Err(e) =>
            {
                error!("Couldn't send response: {}", e);
                b = false;
                break;
            }
        }
    }

    b
}

/// Handles connection with client
/// * stream - a TCP stream to the client
fn handle_client(mut stream: TcpStream) 
{
    let mut data = [0 as u8; MAX_READ_BUFFER_SIZE];

    let command = CommandParser{};

    while match stream.read(&mut data) 
    {
        Ok(size) => 
        {
            let mut b = true;
            let mut data_str: String = "".to_string();

            if size == 0 || size >= MAX_READ_BUFFER_SIZE - 1
            {
                stream.shutdown(Shutdown::Both).unwrap();
                b = false;
            }
            else
            {
                data[size] = 0;
                if let Ok(rets) = str::from_utf8(&data) 
                {
                    debug!("Received command:{}", rets);                    
                    let data_vec = remove_newline_characters(rets).iter().cloned().collect();
                    data_str = String::from_utf8(data_vec).unwrap();
                }
                else
                {
                    stream.shutdown(Shutdown::Both).unwrap();
                    b = false;
                }    
            }
            
            if b
            {
                let ret = command.get_command(&data_str);
                match ret
                {
                    Ok(x) => 
                    {
                        if x.0.len() > 0
                        {
                            replay(&stream, x.0, x.1);
                            stream.shutdown(Shutdown::Both).unwrap();
                            b = false;
                        }
                    },
                    Err(x) => error!("{}", x)
                }     
            }
            
            b
        },
        Err(_) => 
        {
            error!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

pub struct FleaServer
{
}

impl FleaServer
{        
    pub fn start(self: &Self, address: &str, running: &Arc<AtomicBool>)
    {
        let listener = TcpListener::bind(address).unwrap();
        listener.set_nonblocking(true).expect("Cannot set non-blocking socket!");

        // accept connections and process them, spawning a new thread for each one
        debug!("Server listening on {}", address);
        
        for stream in listener.incoming() 
        {
            match stream 
            {
                Ok(stream) => 
                {
                    debug!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move|| {
                        // connection succeeded
                        handle_client(stream)
                    });
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => 
                {
                    if !running.load(Ordering::SeqCst)
                    {
                        break;
                    }
                    thread::sleep(std::time::Duration::from_millis(100));
                    continue;
                }
                Err(e) =>
                {
                    error!("Error: {}", e);
                }
            }
        }
        
        // close the socket server
        drop(listener);        
    }
}