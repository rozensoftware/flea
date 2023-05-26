use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str;
use log::{debug, error};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::commandparser::CommandParser;
use crate::commandprocessor::{CommandProcessor, FleaCommand, STOP_COMMAND};
use crate::fileserver::FileServer;

const MAX_READ_BUFFER_SIZE: usize = 1024;

/// Returns vector of characters without the ending line codes
///
/// * str - An input string having end line special characters 0x0d, 0x0a
fn remove_newline_characters(str: &str) -> Vec<u8>
{
    let s1 = str.to_string().replace('\n', "");
    let s2 = s1.replace('\r', "");
    let temp_str = s2.as_bytes();
    let end_str_idx = temp_str.iter().position(|&x| x == 0).unwrap();
    temp_str[..end_str_idx].to_vec()
}

/// Sends response to client
/// * 'stream' - Stream connection to the client
/// * 'command_name' - A command name
/// * 'value_name' - A command value
/// # Returns
/// * 'bool' - True if the response was sent successfully, false otherwise
/// * 'bool' - True if the command is STOP_COMMAND, false otherwise
fn replay(mut stream: &TcpStream, command_name: String, value_name: String, file_server: &Arc<Mutex<FileServer>>) -> (bool, bool)
{
    debug!("Received command: {} with value: {}", command_name, value_name);

    let mut command_processor: CommandProcessor = FleaCommand::new();
    let mut b = true;
    let cmd = command_processor.process(command_name.as_str(), value_name.as_str(), file_server);
    
    if cmd.is_empty()
    {
        return (true, false);
    }

    if cmd == STOP_COMMAND
    {
        return (true, true);
    }

    let mut data_idx = 0;

    if stream.set_nonblocking(true).is_err()
    {
        error!("Couldn't set non-blocking mode");
        return (false, false);
    }
    
    loop 
    {
        match stream.write(cmd[data_idx..].as_bytes())
        {
            Ok(sent_bytes) =>
            {
                data_idx += sent_bytes;
                if data_idx == cmd.len() 
                {
                    break;
                }        
            },
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => 
            {
                thread::sleep(std::time::Duration::from_millis(100));
                continue;
            }
            Err(e) =>
            {
                error!("Couldn't send response: {}", e);
                b = false;
                break;
            }
        }
    }

    (b, false)
}

/// Handles connection with client
/// * stream - a TCP stream to the client
/// * file_server - a file server
fn handle_client(mut stream: TcpStream, file_server: &Arc<Mutex<FileServer>>, running: &Arc<AtomicBool>)
{
    let mut data = [0_u8; MAX_READ_BUFFER_SIZE];
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
                    let data_vec = remove_newline_characters(rets).to_vec();
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
                        if !x.0.is_empty()
                        {
                            let status = replay(&stream, x.0, x.1, file_server);
                            stream.shutdown(Shutdown::Both).unwrap();
                            if status.1
                            {
                                running.store(false, Ordering::Relaxed);
                            }
                            b = false;
                        }
                    },
                    Err(x) => 
                    {
                        stream.shutdown(Shutdown::Both).unwrap();
                        error!("{}", x);
                        b = false;
                    }
                }     
            }
            
            b
        },
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => 
        {
            thread::sleep(std::time::Duration::from_millis(100));
            true
        },
        Err(e) => 
        {
            error!("An error occurred, terminating connection with {}\nError:{}", stream.peer_addr().unwrap(), e.to_string());
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
    pub fn start(&self, address: &str, running: &Arc<AtomicBool>)
    {
        let listener = TcpListener::bind(address).unwrap();
        listener.set_nonblocking(true).expect("Cannot set non-blocking socket!");

        //Here we are creating a new instance of FileServer and wrapping it in Arc and Mutex
        //Arc is a thread-safe reference-counting pointer. Mutex is a mutual exclusion primitive useful for protecting shared data.
        //As we have only one instance of FileServer, two or more clients can access it at the same time
        //They can change the current directory, get a file, etc.
        //This could cause a problem but this is done by design. Normally we should have only one peer though.
        let file_server_data = Arc::new(Mutex::new(FileServer::new()));

        // accept connections and process them, spawning a new thread for each one
        debug!("Server listening on {}", address);
        
        for stream in listener.incoming() 
        {
            match stream 
            {
                Ok(stream) => 
                {
                    debug!("New connection: {}", stream.peer_addr().unwrap());

                    // connection succeeded
                    let file_server = Arc::clone(&file_server_data);
                    let r = Arc::clone(running);

                    thread::spawn(move || {
                        handle_client(stream, &file_server, &r);
                    });
                },
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => 
                {
                    if !running.load(Ordering::SeqCst)
                    {
                        break;
                    }
                    thread::sleep(std::time::Duration::from_millis(100));
                    continue;
                },
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