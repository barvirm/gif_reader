use crate::read_ext::ReadExt;
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
    pub(crate) fn from_bytes<T: Read>(reader: &mut T) -> Result<Header, &'static str> {
        let header = reader.read_bytes::<6>().unwrap();

        let signature = match &header[..3] {
            b"GIF" => "GIF",
            _ => panic!("THIS IS NOT GIF FORMAT"),
        };

        let version = match &header[3..] {
            b"89a" => Version::V89a,
            b"87a" => Version::V87a,
            _ => panic!("Unknow version of GIF"),
        };

        Ok(Self { signature, version })
    }
}
