use std::fmt::{Display, Formatter};
use uuid::Uuid;
use crate::network::data_container::{Container, Data};
use crate::protocol::commands::{AdaCommandHeader, AdaCommandType, COMMON_HEADER_SIZE};


pub fn parse_header(buffer: &[u8]) -> Option<AdaCommandHeader> {
    if buffer.len() != COMMON_HEADER_SIZE as usize {
        return None;
    }

    let version = buffer[0];
    let content_length = u32::from_be_bytes([buffer[1], buffer[2], buffer[3], buffer[4]]);
    let command_type = match AdaCommandType::try_from(buffer[5]) {
        Ok(t) => t,
        Err(_) => return None, // Invalid command type
    };
    let network_id = match Uuid::from_slice(&buffer[6..22]) {
        Ok(uuid) => uuid,
        Err(_) => return None, // Invalid UUID
    };
    let node_id = match Uuid::from_slice(&buffer[22..38]) {
        Ok(uuid) => uuid,
        Err(_) => return None, // Invalid UUID
    };

    Some(AdaCommandHeader {
        version,
        content_length,
        command_type,
        node: Some(network_id),
        network: Some(node_id),
    })
}

pub fn parse_data_container(content_buffer: Vec<u8>, data_offset: usize) -> Option<Container> {
    let data_size: usize = u32::from_be_bytes([
        content_buffer[data_offset],
        content_buffer[data_offset + 1],
        content_buffer[data_offset + 2],
        content_buffer[data_offset + 3],
    ]) as usize;

    if (data_size + data_offset) > content_buffer.len() {
        log::error!("Data field is not inside buffer boundaries!");
        return None;
    }

    log::debug!("Reading data field of size {}", data_size);

    let mut container = Container::new();

    let mut offset = data_offset + 4;
    while offset < data_offset + data_size {
        // Read key length
        let key_len = u16::from_be_bytes([content_buffer[offset], content_buffer[offset + 1]]);
        offset += 2;

        // Check if remaining buffer is sufficient for key and value
        if offset + 2 + key_len as usize > content_buffer.len() {
            return None; // Invalid data format (insufficient remaining buffer)
        }

        // Extract key
        let key = std::str::from_utf8(&content_buffer[offset..offset + key_len as usize]).unwrap_or_else(|err| {
            // Handle conversion error (e.g., log the error or return an error)
            log::error!("Error converting key bytes to string: {}", err);
            "unknown"
        });
        offset += key_len as usize;

        log::debug!("Reading key {}", key);

        // Read data type byte
        let data_type = Data::try_from(content_buffer[offset]).unwrap();

        log::debug!("Reading value of type: {}", content_buffer[offset] );
        offset += 1;


        // Parse data based on type
        let data = match data_type {
            Data::Integer(_) => {
                // Integer (already handled by TryFrom)
                if offset + 4 > content_buffer.len() {
                    return None; // Insufficient buffer for integer value
                }
                let value = i32::from_be_bytes([content_buffer[offset], content_buffer[offset + 1], content_buffer[offset + 2], content_buffer[offset + 3]]);
                offset += 4;
                Some(Data::Integer(value))
            }
            Data::Float(_) => {
                // Float (already handled by TryFrom)
                if offset + 8 > content_buffer.len() {
                    return None; // Insufficient buffer for float value
                }
                let value = f64::from_be_bytes([content_buffer[offset], content_buffer[offset + 1], content_buffer[offset + 2], content_buffer[offset + 3], content_buffer[offset + 4], content_buffer[offset + 5], content_buffer[offset + 6], content_buffer[offset + 7]]);
                offset += 8;
                Some(Data::Float(value))
            }
            Data::Text(_) => {
                let slice = &content_buffer[offset..];
                let text = match slice.iter().position(|&b| b == 0) {
                    Some(null_terminator) => Some(String::from_utf8_lossy(&slice[..null_terminator]).to_string()),
                    None => Some(String::from_utf8_lossy(slice).to_string()),
                };
                match text {
                    Some(val) => {
                        offset += val.len() + 1;
                        Some(Data::Text(val.to_string()))
                    }
                    None => None
                }
            }
            Data::Boolean(_) => {
                // Boolean (already handled by TryFrom)
                if offset + 1 > content_buffer.len() {
                    return None; // Insufficient buffer for boolean value
                }
                let value = content_buffer[offset] != 0;
                offset += 1;
                Some(Data::Boolean(value))
            }
            _ => {
                log::warn!("Unknown data type encountered, skipping key-value pair");
                None
            }
        };

        log::debug!("Read data field {} with value {:?}", key, data);

        if let Some(value) = data {
            container.add_item(key, value);
        }
    }

    Some(container)
}