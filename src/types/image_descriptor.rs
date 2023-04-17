use crate::{
    error::{GifParserError, ImageDescriptorIo},
    read_ext::ReadExt,
};
use std::io::Read;

#[derive(Debug)]
pub struct PackedLocalFields {
    pub color_table_flag: bool,
    pub interlace_flag: bool,
    pub sort_flag: bool,
    pub color_table_size: usize,
}

impl PackedLocalFields {
    pub fn from_byte(byte: u8) -> Self {
        let color_table_flag = byte & 0b10000000 != 0;
        let interlace_flag = byte & 0b01000000 != 0;
        let sort_flag = byte & 0b00100000 != 0;
        let color_table_size = {
            let mut v = byte & 0b00000111;
            if v != 0 {
                v = 2 << (v + 1);
            }
            v
        }
        .into();

        Self {
            color_table_flag,
            interlace_flag,
            sort_flag,
            color_table_size,
        }
    }
}

#[derive(Debug)]
pub struct ImageDescriptor {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16,
    pub packed: PackedLocalFields,
}

impl ImageDescriptor {
    pub(crate) fn from_bytes<T: Read>(reader: &mut T) -> Result<Self, GifParserError> {
        let data = reader
            .read_bytes::<9>()
            .map_err(|e| ImageDescriptorIo(e))
            .map_err(|e| GifParserError::ImageDescriptorIo(e))?;

        let left = u16::from_le_bytes(data[0..2].try_into().expect("Valid range for u16"));
        let top = u16::from_le_bytes(data[2..4].try_into().expect("Valid range for u16"));

        let width = u16::from_le_bytes(data[4..6].try_into().expect("Valid range for u16"));
        let height = u16::from_le_bytes(data[6..8].try_into().expect("Valid range for u16"));

        let packed = PackedLocalFields::from_byte(data[8]);

        Ok(ImageDescriptor {
            left,
            top,
            width,
            height,
            packed,
        })
    }
}
