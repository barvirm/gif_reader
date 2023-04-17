use crate::{
    error::{GifParserError, PlainTextExtIo},
    read_ext::{ReadExt, ReadSubBlockExt},
};
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
    pub(crate) fn from_bytes<T: Read>(reader: &mut T) -> Result<Self, GifParserError> {
        let data = reader
            .read_bytes::<13>()
            .map_err(|e| PlainTextExtIo(e))
            .map_err(|e| GifParserError::PlainTextExtIo(e))?;

        if data[0] != 0x0C {
            return Err(GifParserError::PlainTextExtInvalidBlockSize);
        }

        let text = {
            let data = reader
                .read_subblock()
                .map_err(|e| PlainTextExtIo(e))
                .map_err(|e| GifParserError::PlainTextExtIo(e))?;
            String::from_utf8(data).map_err(|e| GifParserError::PlainTextExtInvalidUtf8(e))?
        };

        let text_grid_left = data[1..3].try_into().expect("Valid range for u16");
        let text_grid_top = data[3..5].try_into().expect("Valid range for u16");
        let text_grid_width = data[5..7].try_into().expect("Valid range for u16");
        let text_grid_height = data[7..9].try_into().expect("Valid range for u16");

        Ok(Self {
            text_grid_left: u16::from_le_bytes(text_grid_left),
            text_grid_top: u16::from_le_bytes(text_grid_top),
            text_grid_width: u16::from_le_bytes(text_grid_width),
            text_grid_height: u16::from_le_bytes(text_grid_height),
            cell_width: data[9],
            cell_height: data[10],
            text_fg_color_index: data[11],
            text_bg_color_index: data[12],
            plain_text_data: text,
        })
    }
}
