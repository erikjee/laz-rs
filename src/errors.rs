use crate::las::laszip::{LazItemType, CompressorType};
use std::fmt;

#[derive(Debug)]
pub enum LasZipError {
    UnknownLazItem(u16),
    UnsupportedLazItemVersion(LazItemType, u16),
    UnknownCompressorType(u16),
    UnsupportedCompressorType(CompressorType),
    IoError(std::io::Error),
}


impl From<std::io::Error> for LasZipError {
    fn from(e: std::io::Error) -> Self {
        LasZipError::IoError(e)
    }
}

impl fmt::Display for LasZipError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            LasZipError::UnknownLazItem(t) => write!(f, "Item with type code: {} is unknown", t),
            LasZipError::UnsupportedLazItemVersion(item_type, version) => write!(f, "Item {:?} with compression version: {} is not supported", item_type, version),
            LasZipError::UnknownCompressorType(compressor_type) => write!(f, "Compressor type {} is not valid", compressor_type),
            LasZipError::UnsupportedCompressorType(compressor_type) => write!(f, "Compressor type {:?} is not supported", compressor_type),
            LasZipError::IoError(e) => write!(f, "IoError: {}", e)
        }
    }
}