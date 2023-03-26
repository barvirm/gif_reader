#[derive(Debug)]
pub enum ExtensionIdentifier {
    GraphicsControl = 0xF9,
    PlainText = 0x01,
    Application = 0xFF,
    Comment = 0xFE,
}
impl TryFrom<u8> for ExtensionIdentifier {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0xF9 => Ok(ExtensionIdentifier::GraphicsControl),
            0x01 => Ok(ExtensionIdentifier::PlainText),
            0xFF => Ok(ExtensionIdentifier::Application),
            0xFE => Ok(ExtensionIdentifier::Comment),
            _ => Err("Unknown extension identifier"),
        }
    }
}
