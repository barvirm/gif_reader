#[derive(Debug)]
pub enum BlockIdentifier {
    Extension = 0x21,
    ImageDescriptor = 0x2C,
    Trailer = 0x3B,
}

impl TryFrom<u8> for BlockIdentifier {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x21 => Ok(BlockIdentifier::Extension),
            0x2c => Ok(BlockIdentifier::ImageDescriptor),
            0x3b => Ok(BlockIdentifier::Trailer),
            _ => Err("Unknown block identifier"),
        }
    }
}
