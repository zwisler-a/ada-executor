use std::io::{self, Write};
use std::net::TcpStream;
use std::vec;

use crate::network::data_container::{Container, Data};
use crate::network::data_container::Data::{Float, Integer};
use crate::protocol::commands::{AdaCommand, AdaCommandHeader, AdaCommandType};

mod protocol;
mod network;

pub fn main() -> io::Result<()> {
    let server_address = "127.0.0.1:7878";
    let mut data = Container::new();
    data.add_item("int", Integer(1));
    data.add_item("float", Float(2.1));
    data.add_item("text", Data::Text("stringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstring".to_string()));
    data.add_item("text", Data::Text("stringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstring".to_string()));
    data.add_item("text", Data::Text("stringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstring".to_string()));
    data.add_item("text", Data::Text("stringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstring".to_string()));
    data.add_item("text", Data::Text("stringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstring".to_string()));
    data.add_item("text", Data::Text("stringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstring".to_string()));
    data.add_item("text", Data::Text("stringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstring".to_string()));
    data.add_item("text", Data::Text("stringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstring".to_string()));
    data.add_item("text", Data::Text("stringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstring".to_string()));
    data.add_item("text", Data::Text("stringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstring".to_string()));
    data.add_item("text", Data::Text("stringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstring".to_string()));
    data.add_item("text", Data::Text("stringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstring".to_string()));
    data.add_item("text", Data::Text("stringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstring".to_string()));
    data.add_item("text", Data::Text("stringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstring".to_string()));
    data.add_item("text", Data::Text("stringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstring".to_string()));
    data.add_item("text", Data::Text("stringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstring".to_string()));
    data.add_item("text", Data::Text("stringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstring".to_string()));
    data.add_item("text", Data::Text("stringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstringstring".to_string()));
    data.add_item("boolean", Data::Boolean(true));
    let commands = vec![
        AdaCommand {
            header: AdaCommandHeader {
                version: 1,
                command_type: AdaCommandType::ExecuteNode,
                content_length: 0,
                node: None,
                network: None,
            },
            data: Some(data),
        }
    ];


    match TcpStream::connect(server_address) {
        Ok(mut stream) => {
            println!("Successfully connected to server at {}", server_address);
            loop {
                for command in &commands {
                    let to_send = command.to_bytes();
                    if let Err(e) = stream.write_all(&to_send) {
                        eprintln!("Failed to send data: {}", e);
                        break;
                    }
                    if let Err(e) = stream.flush() {
                        eprintln!("Failed to flush stream: {}", e);
                        break;
                    }
                    // println!("Binary data sent: {:?}", to_send);
                }
                // thread::sleep(Duration::from_millis(100));
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
        }
    }
    Ok(())
}
