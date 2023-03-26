use crate::read_ext::ReadExt;
use std::io::Read;

#[derive(Debug)]
pub enum Disposal {
    NoDisponse = 0,
    DontDispose = 1,
    RestoreBackground = 2,
    RestoreToPrevious = 3,
}

impl TryFrom<u8> for Disposal {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Disposal::NoDisponse,
            1 => Disposal::DontDispose,
            2 => Disposal::RestoreBackground,
            3 => Disposal::RestoreToPrevious,
            _ => panic!("Unknow disposal"),
        })
    }
}

#[derive(Debug)]
pub struct GraphicsControlPackedFields {
    pub transparent_color_flag: bool,
    pub user_input: bool,
    pub disposal_method: Disposal,
}

impl GraphicsControlPackedFields {
    pub fn from_byte(byte: u8) -> Result<Self, &'static str> {
        Ok(Self {
            transparent_color_flag: byte & 0b00000001 != 0,
            user_input: byte & 0b00000010 != 0,
            disposal_method: Disposal::try_from((byte & 0b00011100) >> 3)?,
        })
    }
}

#[derive(Debug)]
pub struct GraphicsControlExtension {
    pub packed: GraphicsControlPackedFields,
    pub delay_time: u16,
    pub transparent_color_index: u8,
}

impl GraphicsControlExtension {
    pub(crate) fn from_bytes<T: Read>(reader: &mut T) -> Result<Self, &'static str> {
        let data = reader.read_bytes::<6>().unwrap();

        let block_size = data[0];
        if block_size != 0x04 {
            return Err("Graphics Control Extension has always size 0x04");
        }

        let gce = GraphicsControlExtension {
            packed: GraphicsControlPackedFields::from_byte(data[1])?,
            delay_time: u16::from_le_bytes(data[2..4].try_into().unwrap()),
            transparent_color_index: data[4],
        };

        let terminator = data[5];
        if terminator != 0 {
            return Err("Invalid terminator");
        }

        Ok(gce)
    }
}
