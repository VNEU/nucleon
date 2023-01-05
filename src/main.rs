#[macro_use]
extern crate log;
extern crate env_logger;
extern crate argparse;
extern crate mio;

mod backend;
mod tcplb;

use std::sync::{Arc, Mutex};
use std::env;

use mio::*;

fn main() {
    let mut servers: Vec<String> = Vec::new();
    let bind = "127.0.0.1:8000".to_string();
    let log_level = "info".to_string(); // Default Log level [debug, info, warn, error] (info)

    env::set_var("RUST_LOG", log_level);

    env_logger::init().unwrap();

    // add 127.0.0.1:80001 to servers
    servers.push("127.0.0.1:8001".to_string()); 
    servers.push("127.0.0.1:8002".to_string()); 
    servers.push("127.0.0.1:8003".to_string()); 
    servers.push("127.0.0.1:8004".to_string()); 
    servers.push("127.0.0.1:8005".to_string()); 
    servers.push("127.0.0.1:8006".to_string()); 

    let backend = Arc::new(Mutex::new(backend::RoundRobinBackend::new(servers).unwrap()));

    let mut proxy = tcplb::Proxy::new(&bind, backend.clone());
    let mut event_loop = EventLoop::new().unwrap();

    // Register interest in notifications of new connections
    event_loop.register_opt(&proxy.listen_sock,
                            Token(1),
                            EventSet::readable(),
                            PollOpt::edge())
              .unwrap();


    // Start handling events
    event_loop.run(&mut proxy).unwrap();

}
