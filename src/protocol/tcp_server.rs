use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::protocol::commands::AdaCommand;
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
        let mut buffer = [0; 6];
        while match stream.read(&mut buffer) {
            Ok(size) if size > 0 => {
                let header_opt = parse_header(&buffer[..size]);
                if header_opt.is_some() {
                    let header = header_opt.unwrap();
                    log::debug!("Got command: {:?}", header);
                    if header.content_length > 6 {
                        let mut content_buffer = vec![0; (header.content_length - 6) as usize];
                        let data = match stream.read(&mut content_buffer) {
                            Ok(_) => {
                                parse_data_container(content_buffer, 0)
                            }
                            Err(_) => { None }
                        };
                        log::debug!("Got data {:?}", data);
                    }
                }

                //let mut collection = queue.lock().unwrap();
                //log::debug!("Adding {} to queue", header_opt);
                //collection.push(header_opt);
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
