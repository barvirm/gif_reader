use crate::{error::GifParserError, read_ext::ReadExt};
use std::io::Read;

#[derive(Debug)]
pub enum Version {
    V87a,
    V89a,
}

#[derive(Debug)]
pub struct Header {
    pub signature: &'static str,
    pub version: Version,
}

impl Header {
    pub(crate) fn from_bytes<T: Read>(reader: &mut T) -> Result<Header, GifParserError> {
        let header = reader.read_bytes::<6>().unwrap();

        let signature = match &header[..3] {
            b"GIF" => "GIF",
            _ => return Err(GifParserError::InvalidGifSignature),
        };

        let version = match &header[3..] {
            b"89a" => Version::V89a,
            b"87a" => Version::V87a,
            _ => return Err(GifParserError::InvalidGifVersion),
        };

        Ok(Self { signature, version })
    }
}
