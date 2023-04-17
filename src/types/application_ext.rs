use crate::error::{AppExtIo, GifParserError};
use crate::read_ext::{ReadExt, ReadSubBlockExt};
use std::io::Read;

#[derive(Debug)]
pub struct ApplicationExtension {
    pub identifier: [u8; 8],
    pub authent_code: [u8; 3],
    pub application_data: Vec<u8>,
}

impl ApplicationExtension {
    pub(crate) fn from_bytes<T: Read>(reader: &mut T) -> Result<Self, GifParserError> {
        let data = reader
            .read_bytes::<12>()
            .map_err(AppExtIo)
            .map_err(GifParserError::AppExtIo)?;

        if data[0] != 0x0B {
            return Err(GifParserError::AppExtInvalidBlockSize);
        }

        let app_data = reader
            .read_subblock()
            .map_err(AppExtIo)
            .map_err(GifParserError::AppExtIo)?;

        Ok(Self {
            identifier: data[1..9].try_into().expect("Valid range size [u8; 8]"),
            authent_code: data[9..].try_into().expect("Valid range size [u8; 3]"),
            application_data: app_data,
        })
    }
}
