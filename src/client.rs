use std::{thread, vec};
use std::io::{self, Write};
use std::net::TcpStream;
use std::time::Duration;
use crate::network::data_container::Container;
use crate::network::data_container::Data::{Float, Integer};

use crate::protocol::commands::{AdaCommand, AdaCommandHeader, AdaCommandType};

mod protocol;
mod network;

pub fn main() -> io::Result<()> {
    let server_address = "127.0.0.1:7878";
    let mut data = Container::new();
    data.add_item("int", Integer(1));
    data.add_item("float", Float(2.0));
    let commands = vec![
        AdaCommand {
            header: AdaCommandHeader {
                version: 1,
                command_type: AdaCommandType::ExecuteNode,
                content_length: 0,
            },
            data: Some(data),
        }
    ];

    // Attempt to connect to the server
    match TcpStream::connect(server_address) {
        Ok(mut stream) => {
            println!("Successfully connected to server at {}", server_address);
            for command in commands {
                let to_send = command.to_bytes();
                stream.write_all(&*to_send)?;
                stream.flush()?;
                thread::sleep(Duration::from_millis(1000));
                println!("Binary data sent: {:?}", to_send);
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
        }
    }

    Ok(())
}
