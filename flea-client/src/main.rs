extern crate getopts;
use flea_client_lib::client::FleaClient;
use getopts::Options;
use std::env;

#[macro_use]
extern crate log;

// Set the correct port number if you have changed the one in the server
const SERVER_PORT: &'static str = "1972";

fn print_usage(program: &str, opts: Options) 
{
    let brief = format!("Usage: {} ADDRESS COMMAND [options]", program);
    println!("{}", opts.usage(&brief));
}

fn main() 
{
    env_logger::init();

    info!("Flea Client starting..");

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    
    let mut opts = Options::new();

    opts.reqopt("a", "address", "address of The Flea Server", "ADDRESS");
    opts.reqopt("c", "command", "command to run on the Flea Server", "COMMAND");
    opts.optopt("v", "value", "additional command value", "");
    opts.optflag("h", "help", "prints this help menu");

    let matches = match opts.parse(&args[1..]) 
    {
        Ok(m) => { m }
        Err(f) => { println!("{}", f.to_string()); print_usage(&program, opts); return }
    };

    if matches.opt_present("h")
    {
        print_usage(&program, opts);
        return;
    }

    let mut address = matches.opt_str("a").unwrap();
    address.push_str(":");
    address.push_str(SERVER_PORT);

    let command = matches.opt_str("c").unwrap();

    let value = match matches.opt_str("v")
    {
        Some(x) =>
        {
            x
        },
        None =>
        {
            "".to_string()
        }
    };

    debug!("Passing parameters: address={} name={} value={}", address, command, value);

    let xml = format!("<Command name='{}' value='{}'></Command>", command, value);
    
    let client = FleaClient{};
    client.send_command(&address, &xml, command.as_str(), value.as_str());
}
