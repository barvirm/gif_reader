use crate::{
    error::{GifParserError, LogicalScreenDescriptorIo},
    read_ext::ReadExt,
};
use std::io::Read;

#[derive(Debug)]
pub struct PackedFields {
    pub global_color_table: bool,
    pub color_resolution: u8,
    pub sort_flag: bool,
    pub global_color_table_size: usize,
}

impl PackedFields {
    pub fn from_byte(byte: u8) -> PackedFields {
        PackedFields {
            global_color_table: byte & 0b10000000 != 0,
            color_resolution: (byte & 0b01110000) >> 4,
            sort_flag: byte & 0b00001000 != 0,
            global_color_table_size: 2 << (byte & 0b00000111),
        }
    }
}

#[derive(Debug)]
pub struct LogicalScreenDescriptor {
    pub local_screen_width: u16,
    pub local_screen_height: u16,
    pub packed_fields: PackedFields,
    pub background_color: u8,
    pub pixel_aspect_ratio: u8,
}

impl LogicalScreenDescriptor {
    pub(crate) fn from_bytes<T: Read>(reader: &mut T) -> Result<Self, GifParserError> {
        let bytes = reader
            .read_bytes::<7>()
            .map_err(LogicalScreenDescriptorIo)
            .map_err(GifParserError::LogicalScreenDescriptorIo)?;

        Ok(Self {
            local_screen_width: u16::from_le_bytes(
                bytes[..2].try_into().expect("Valid range for u16"),
            ),
            local_screen_height: u16::from_le_bytes(
                bytes[2..4].try_into().expect("Valid range for u16"),
            ),
            packed_fields: PackedFields::from_byte(bytes[4]),
            background_color: bytes[5],
            pixel_aspect_ratio: bytes[6],
        })
    }
}
