#[macro_use]
extern crate log;

use flealib::fleaserver::FleaServer;

//Change the ip address of the server according to your needs
const SERVER_IP: &'static str = "127.0.0.1:1972";

fn main() 
{
    env_logger::init();

    info!("Start");

    let flea_server = FleaServer{};
    flea_server.start(SERVER_IP);
}
