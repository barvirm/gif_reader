use crate::read_ext::{ReadExt, ReadSubBlockExt};
use std::io::Read;

#[derive(Debug)]
pub struct ApplicationExtension {
    pub identifier: [u8; 8],
    pub authent_code: [u8; 3],
    pub application_data: Vec<u8>,
}

impl ApplicationExtension {
    pub(crate) fn from_bytes<T: Read>(reader: &mut T) -> Result<Self, &'static str> {
        let data = reader.read_bytes::<12>().unwrap();

        if data[0] != 0x0B {
            return Err("Block size is always 0B in hex");
        }

        let app_data = reader.read_subblock().unwrap();

        Ok(Self {
            identifier: data[1..9].try_into().unwrap(),
            authent_code: data[9..].try_into().unwrap(),
            application_data: app_data,
        })
    }
}
