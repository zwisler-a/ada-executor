use std::fmt::{Display, Formatter};
use std::mem;

use uuid::Uuid;

use crate::network::data_container::Container;

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum AdaCommandType {
    CloseConnection = 1,
    ExecuteNode = 30,
    Unknown,
}


impl TryFrom<u8> for AdaCommandType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(AdaCommandType::CloseConnection),
            30 => Ok(AdaCommandType::ExecuteNode),
            _ => Ok(AdaCommandType::Unknown),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AdaCommandHeader {
    pub version: u8,
    pub content_length: u32,
    pub command_type: AdaCommandType,
}

pub struct AdaNetworkCommandHeader {
    pub network: Uuid,
    pub node: Uuid,
    pub data_length: u32,
}

pub struct AdaCommand {
    pub header: AdaCommandHeader,
    pub data: Option<Container>,
}


impl AdaCommand {
    pub fn new(command_type: AdaCommandType) -> Self {
        AdaCommand {
            header: { AdaCommandHeader { command_type, content_length: 0, version: 0 } },
            data: None,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let header = self.header.clone();
        bytes.push(header.version);
        let size = self.get_total_bytes();
        bytes.extend_from_slice(&size.to_be_bytes());
        bytes.push(header.command_type as u8);

        if let Some(data) = &self.data {
            bytes.extend_from_slice(&*data.get_bytes());
        }

        bytes
    }

    fn get_total_bytes(&self) -> u32 {
        let mut size = 1 + 4 + 1;
        let mut data_size = 0; // Initialize data_size to 0

        if let Some(data) = &self.data {
            data_size = data.get_data_size();
        }

        (size + data_size) as u32
    }
}

impl Display for AdaCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[AdaCommand][{:?}]", self.header)
    }
}