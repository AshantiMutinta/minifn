use crate::common::error::MiniFnError;
use std::io::Read;

pub fn load<'a, T: Read>(
    mut reader: T,
    text_buffer: &'a mut Vec<u8>,
) -> Result<&'a str, MiniFnError> {
    reader.read_to_end(text_buffer)?;
    std::str::from_utf8(text_buffer).map_err(|s| s.into())
}
