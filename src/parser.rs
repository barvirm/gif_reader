use crate::error::GifParserError;
use crate::read_ext::{ReadExt, ReadSubBlockExt};
use std::io::Read;
use weezl::decode::Decoder as LzwDecoder;
use weezl::BitOrder;

use crate::types::{
    ApplicationExtension, BlockIdentifier, ColorTable, ExtensionIdentifier,
    GraphicsControlExtension, Header, ImageDescriptor, LogicalScreenDescriptor, PlainTextExtension,
};

#[derive(Debug)]
enum State {
    Header,
    LogicalScreenDescriptor,
    GlobalColorTable(usize),
    LocalColorTable(usize),
    ImageDescriptor,
    ImageData(),
    Trailer,

    ExtGraphicsControl,
    ExtApplication,

    ExtPlainText,
    ExtComment,
}

#[derive(Debug)]
pub enum GifParts {
    Header(Header),
    LogicalScreenDescriptor(LogicalScreenDescriptor),
    GlobalColorTable(ColorTable),
    ExtGraphicsControl(GraphicsControlExtension),
    ApplicationExtension(ApplicationExtension),
    LocalColorTable(ColorTable),
    ImageDescriptor(ImageDescriptor),
    ImageData(Vec<u8>),
    ExtPlainText(PlainTextExtension),
    ExtComment(String),
    Trailer,
}

pub struct GifParser<T: Read> {
    reader: T,
    state: State,
}

impl<T: Read> GifParser<T> {
    pub fn new(reader: T) -> Self {
        Self {
            reader,
            state: State::Header,
        }
    }

    fn read_next_state(&mut self) -> State {
        let byte = self.reader.read_byte().unwrap();
        match BlockIdentifier::try_from(byte).unwrap() {
            BlockIdentifier::Extension => {
                let ext_identifier = self.reader.read_byte().unwrap();
                let ext_identifier = ExtensionIdentifier::try_from(ext_identifier).unwrap();
                match ext_identifier {
                    ExtensionIdentifier::GraphicsControl => State::ExtGraphicsControl,
                    ExtensionIdentifier::PlainText => State::ExtPlainText,
                    ExtensionIdentifier::Application => State::ExtApplication,
                    ExtensionIdentifier::Comment => State::ExtComment,
                }
            }
            BlockIdentifier::ImageDescriptor => State::ImageDescriptor,
            BlockIdentifier::Trailer => State::Trailer,
        }
    }

    fn get_next_part(&mut self) -> Result<GifParts, GifParserError> {
        let part = match self.state {
            State::Trailer => GifParts::Trailer,
            State::Header => {
                let header = Header::from_bytes(&mut self.reader)?;

                self.state = State::LogicalScreenDescriptor;
                GifParts::Header(header)
            }
            State::LogicalScreenDescriptor => {
                let l = LogicalScreenDescriptor::from_bytes(&mut self.reader).unwrap();

                self.state = match l.packed_fields.global_color_table {
                    true => State::GlobalColorTable(l.packed_fields.global_color_table_size),
                    false => self.read_next_state(),
                };
                GifParts::LogicalScreenDescriptor(l)
            }
            State::GlobalColorTable(size) => {
                let color_table = ColorTable::from_bytes(&mut self.reader, size).unwrap();

                self.state = self.read_next_state();
                GifParts::GlobalColorTable(color_table)
            }
            State::ExtGraphicsControl => {
                let g = GraphicsControlExtension::from_bytes(&mut self.reader).unwrap();

                self.state = self.read_next_state();
                GifParts::ExtGraphicsControl(g)
            }
            State::ExtApplication => {
                let a = ApplicationExtension::from_bytes(&mut self.reader).unwrap();

                self.state = self.read_next_state();
                GifParts::ApplicationExtension(a)
            }

            State::LocalColorTable(size) => {
                let t = ColorTable::from_bytes(&mut self.reader, size).unwrap();

                self.state = State::ImageData();
                GifParts::LocalColorTable(t)
            }
            State::ImageData() => {
                let lzw_code_size = self.reader.read_byte().unwrap();
                let mut decoder = LzwDecoder::new(BitOrder::Lsb, lzw_code_size);

                let data = self.reader.read_subblock().unwrap();
                let image_data = decoder.decode(&data).unwrap();

                self.state = self.read_next_state();
                GifParts::ImageData(image_data)
            }
            State::ImageDescriptor => {
                let i = ImageDescriptor::from_bytes(&mut self.reader).unwrap();

                self.state = match i.packed.color_table_flag {
                    true => State::LocalColorTable(i.packed.color_table_size),
                    false => State::ImageData(),
                };
                GifParts::ImageDescriptor(i)
            }
            State::ExtPlainText => {
                let p = PlainTextExtension::from_bytes(&mut self.reader).unwrap();

                self.state = self.read_next_state();
                GifParts::ExtPlainText(p)
            }
            State::ExtComment => {
                let data = self.reader.read_subblock().unwrap();
                let comment = String::from_utf8(data).unwrap();

                self.state = self.read_next_state();
                GifParts::ExtComment(comment)
            }
        };

        Ok(part)
    }
}

impl<T: Read> Iterator for GifParser<T> {
    type Item = Result<GifParts, GifParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.get_next_part() {
            Ok(part) => match part {
                GifParts::Trailer => None,
                gif_part => Some(Ok(gif_part)),
            },
            Err(error) => Some(Err(error)),
        }
    }
}
