use std::{io, string};
use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct GraphicsControlExtIo(#[from] pub io::Error);

#[derive(Error, Debug)]
#[error(transparent)]
pub struct AppExtIo(#[from] pub io::Error);

#[derive(Error, Debug)]
#[error(transparent)]
pub struct ImageDescriptorIo(#[from] pub io::Error);

#[derive(Error, Debug)]
#[error(transparent)]
pub struct LogicalScreenDescriptorIo(#[from] pub io::Error);

#[derive(Error, Debug)]
#[error(transparent)]
pub struct PlainTextExtIo(#[from] pub io::Error);

#[derive(Error, Debug)]
pub enum GifParserError {
    #[error("Block size has always 0x0B in Application extension")]
    AppExtInvalidBlockSize,
    #[error("IO error while parsing Application extension: {0}")]
    AppExtIo(AppExtIo),
    #[error("Unknown block identifier")]
    UnknownBlockIdentifier,
    #[error("Unknown extension identifier")]
    UnknownExtensionIdentifier,
    #[error("Unknown disposal")]
    UnknownDisposalMethod,
    #[error("Block size has always 0x04 in Graphics Control Extension")]
    GraphicsControlExt,
    #[error("Invalid terminator")]
    GraphicsControlExtInvalidTerminator,
    #[error("IO error while parsing Graphics Control Block: {0}")]
    GraphicsControlExtIo(GraphicsControlExtIo),
    #[error("Invalid GIF signature")]
    InvalidGifSignature,
    #[error("Unknown GIF version")]
    InvalidGifVersion,
    #[error("IO error while parsing Image Descriptor: {0}")]
    ImageDescriptorIo(ImageDescriptorIo),
    #[error("IO error while parsing Logical Screen Descriptor: {0}")]
    LogicalScreenDescriptorIo(LogicalScreenDescriptorIo),
    #[error("Block size has always 0x0C in Plain Text Extension")]
    PlainTextExtInvalidBlockSize,
    #[error("IO error while parsing Plain Text Extension: {0}")]
    PlainTextExtIo(PlainTextExtIo),
    #[error("IO error while parsing Plain Text Extension: {0}")]
    PlainTextExtInvalidUtf8(#[from] string::FromUtf8Error),
}
