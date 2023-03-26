use crate::read_ext::{ReadExt, ReadSubBlockExt};
use std::io::Read;

#[derive(Debug)]
pub struct PlainTextExtension {
    pub text_grid_left: u16,
    pub text_grid_top: u16,
    pub text_grid_width: u16,
    pub text_grid_height: u16,
    pub cell_width: u8,
    pub cell_height: u8,
    pub text_fg_color_index: u8,
    pub text_bg_color_index: u8,
    pub plain_text_data: String,
}

impl PlainTextExtension {
    pub(crate) fn from_bytes<T: Read>(reader: &mut T) -> Result<Self, &'static str> {
        let data = reader.read_bytes::<13>().unwrap();

        if data[0] != 0x0C {
            return Err("Block size is always 0C in hex");
        }

        let text = {
            let data = reader.read_subblock().unwrap();
            String::from_utf8(data).unwrap()
        };

        Ok(Self {
            text_grid_left: u16::from_le_bytes(data[1..3].try_into().unwrap()),
            text_grid_top: u16::from_le_bytes(data[3..5].try_into().unwrap()),
            text_grid_width: u16::from_le_bytes(data[5..7].try_into().unwrap()),
            text_grid_height: u16::from_le_bytes(data[7..9].try_into().unwrap()),
            cell_width: data[9],
            cell_height: data[10],
            text_fg_color_index: data[11],
            text_bg_color_index: data[12],
            plain_text_data: text,
        })
    }
}
