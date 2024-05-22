use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

use simple_logger::SimpleLogger;

use command_processor::command_scheduler;

mod protocol;
mod command_processor;
mod network;

fn main() {
    SimpleLogger::new().init().unwrap();

    let server_address = "127.0.0.1:7878";
    let command_queue = Arc::new(Mutex::new(Vec::new()));
    let command_queue_server = Arc::clone(&command_queue);
    let server = protocol::tcp_server::TcpServer::new(server_address, command_queue_server);

    let command_queue_listener = Arc::clone(&command_queue);
    command_scheduler::listen(command_queue_listener);


    let _ = server.run();
}
