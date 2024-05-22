use std::collections::HashMap;
use std::fmt;
use crate::protocol::commands::AdaCommandType;

// Define an enum that can hold different types of data
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum Data {
    Integer(i32) = 1,
    Float(f64) = 2,
    Text(String) = 3,
    Boolean(bool) = 4,
}

impl Data {
    pub(crate) fn get_data_type(&self) -> u8 {
        match self {
            Data::Integer(_) => 1,
            Data::Float(_) => 2,
            Data::Text(_) => 3,
            Data::Boolean(_) => 4,
        }
    }
}

impl TryFrom<u8> for Data {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Data::Integer(0)),
            2 => Ok(Data::Float(0.0)),
            3 => Ok(Data::Text("".to_string())),
            4 => Ok(Data::Boolean(false)),
            _ => Ok(Data::Boolean(false)),
        }
    }
}

// Implement Display for pretty printing
impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Data::Integer(value) => write!(f, "{}", value),
            Data::Float(value) => write!(f, "{}", value),
            Data::Text(value) => write!(f, "{}", value),
            Data::Boolean(value) => write!(f, "{}", value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Container {
    items: HashMap<String, Data>,
}

impl Container {
    pub fn get_data_size(&self) -> u32 {
        let mut size = 4;

        // Iterate through key-value pairs to calculate size
        for (key, value) in &self.items {
            size += 2 + key.len(); // Key length + key bytes
            size += 1; // Data type byte
            match value {
                Data::Integer(_) => size += 4,
                Data::Float(_) => size += 8,
                Data::Text(value) =>  size += value.len() + 1,
                Data::Boolean(_) => size += 1,
                _ => log::warn!("Unknown data type encountered, skipping value"),
            }
        }
        size as u32
    }
    pub(crate) fn get_bytes(&self) -> Vec<u8> {
        let size = self.get_data_size();
        // Allocate buffer and serialize data
        let mut buffer = Vec::with_capacity(size as usize);
        buffer.extend_from_slice(&size.to_be_bytes());
        for (key, value) in &self.items {
            // Key length
            let key_len = key.len() as u16;
            buffer.extend_from_slice(&key_len.to_be_bytes());

            // Key bytes
            buffer.extend_from_slice(key.as_bytes());

            // Data type byte
            buffer.push(value.get_data_type());

            // Data value based on type
            match value {
                Data::Integer(i) => buffer.extend_from_slice(&i.to_be_bytes()),
                Data::Float(f) => buffer.extend_from_slice(&f.to_be_bytes()),
                Data::Text(text) => {
                    let mut bytes = text.as_bytes().to_vec();
                    bytes.push(0);
                    buffer.extend_from_slice(&bytes);
                }
                Data::Boolean(b) => buffer.push(if *b { 1 } else { 0 }),
                _ => log::warn!("Unknown data type encountered, skipping value"),
            }
        }

        buffer
    }
}

impl Container {
    pub fn new() -> Self {
        Container {
            items: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, key: &str, value: Data) {
        self.items.insert(key.to_string(), value);
    }

    pub fn get_item(&self, key: &str) -> Option<&Data> {
        self.items.get(key)
    }
}