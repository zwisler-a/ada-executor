use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::protocol::commands::{AdaCommand, COMMON_HEADER_SIZE};
use crate::protocol::protocol_parser::{parse_data_container, parse_header};

pub struct TcpServer {
    address: String,
    queue: Arc<Mutex<Vec<AdaCommand>>>,
}

impl TcpServer {
    pub fn new(address: &str, queue: Arc<Mutex<Vec<AdaCommand>>>) -> Self {
        Self {
            address: address.to_string(),
            queue,
        }
    }

    fn handle_client(queue: Arc<Mutex<Vec<AdaCommand>>>, mut stream: TcpStream) {
        let mut buffer = [0; COMMON_HEADER_SIZE as usize];
        while match stream.read(&mut buffer) {
            Ok(size) if size > 0 => {
                let header_opt = parse_header(&buffer[..size]);
                if header_opt.is_some() {
                    let header = header_opt.unwrap();
                    let mut command: AdaCommand = AdaCommand {
                        header: header.clone(),
                        data: None,
                    };
                    log::debug!("Got command: {:?}", header);
                    if header.content_length > COMMON_HEADER_SIZE {
                        let mut content_buffer = vec![0; (header.content_length - COMMON_HEADER_SIZE) as usize];
                        match stream.read(&mut content_buffer) {
                            Ok(_) => {
                                command.data = parse_data_container(content_buffer, 0);
                                log::debug!("Setting data {:?}", command.data);
                            }
                            Err(_) => {
                                log::debug!("No data for the given command")
                            }
                        };
                    }


                    let mut collection = queue.lock().unwrap();
                    log::debug!("Adding {} to queue", command);
                    collection.push(command);
                }


                true
            }
            Ok(_) => {
                // Connection closed
                log::debug!("Connection closed by peer: {}", stream.peer_addr().unwrap());
                false
            }
            Err(e) => {
                log::error!("An error occurred, terminating connection with {}: {}", stream.peer_addr().unwrap(), e);
                stream.shutdown(std::net::Shutdown::Both).unwrap();
                false
            }
        } {}
    }

    pub fn run(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.address)?;
        log::info!("Server listening on {}", self.address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    log::debug!("New connection: {}", stream.peer_addr().unwrap());
                    let queue_clone = Arc::clone(&self.queue);
                    thread::spawn(move || {
                        TcpServer::handle_client(queue_clone, stream)
                    });
                }
                Err(e) => {
                    log::error!("Error: {}", e);
                }
            }
        }

        Ok(())
    }
}
