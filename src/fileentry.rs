//! PAK File entry parsing.

use std::convert::{TryFrom, TryInto};
use std::mem::size_of;
use std::os::raw::c_char;

/// PAK file entry size in bytes.
pub const FILE_ENTRY_SIZE: u32 = size_of::<IdPakFileEntry>() as u32;

/// PAK path len
const NAME_LEN: usize = 56;

/// PAK file entry
#[repr(C)]
pub struct IdPakFileEntry {
    /// File path.
    ///
    /// 56 byte null-terminated string.
    /// Example: "maps/e1m1.bsp".
    pub name: [c_char; 56],

    /// The offset (from the beginning of the pak file)
    /// to the beginning of this file's contents.
    pub offset: u32,

    /// The size of this file.
    pub size: u32,
}

impl IdPakFileEntry {
    pub fn get_name(&self) -> String {
        let mut result = String::new();
        for i in self.name.iter().take_while(|item| **item != 0x0) {
            result.push(char::from(u8::from_le(*i as u8)));
        }

        result
    }
}

impl TryFrom<&[u8]> for IdPakFileEntry {
    type Error = &'static str;

    /// Create an `IdPakFileEntry` from a byte slice.
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < FILE_ENTRY_SIZE as usize {
            return Err("Not enough data for file entry.");
        }

        let mut path: [c_char; NAME_LEN] = [0; NAME_LEN];
        let (sig, tail) = bytes.split_at(NAME_LEN);
        for (i, c) in sig.iter().enumerate() {
            path[i] = *c as c_char;
        }

        let (offset, size) = tail.split_at(4);
        let offset: [u8; 4] = match offset.try_into() {
            Err(_) => return Err("Invalid offset."),
            Ok(arr) => arr,
        };
        let size: [u8; 4] = match size.try_into() {
            Err(_) => return Err("Invalid size."),
            Ok(arr) => arr,
        };

        Ok(IdPakFileEntry {
            name: path,
            offset: u32::from_le_bytes(offset),
            size: u32::from_le_bytes(size),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Valid PAK file entry as a bytearray.
    ///
    /// * path: sound/items/r_item1.wav
    /// * offset: 12
    /// * size: 6822
    const VALID_FILE_ENTRY: [u8; 64] = [
        0x73, 0x6F, 0x75, 0x6E, 0x64, 0x2F, 0x69, 0x74, 0x65, 0x6D, 0x73, 0x2F, 0x72, 0x5F, 0x69,
        0x74, 0x65, 0x6D, 0x31, 0x2E, 0x77, 0x61, 0x76, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00,
        0xA6, 0x1A, 0x00, 0x00,
    ];

    #[test]
    fn test_file_entry_from_byte_slice() {
        let name: [c_char; NAME_LEN] = [
            0x73, 0x6F, 0x75, 0x6E, 0x64, 0x2F, 0x69, 0x74, 0x65, 0x6D, 0x73, 0x2F, 0x72, 0x5F,
            0x69, 0x74, 0x65, 0x6D, 0x31, 0x2E, 0x77, 0x61, 0x76, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let file_entry = IdPakFileEntry::try_from(&VALID_FILE_ENTRY[..]).unwrap();
        for (a, b) in file_entry.name.iter().zip(name.iter()) {
            assert_eq!(a, b);
        }
        assert_eq!(file_entry.offset, 12);
        assert_eq!(file_entry.size, 6822);
    }

    #[test]
    fn test_file_entry_from_byte_slice_inv() {
        let random_bytes: [u8; 32] = [
            0x8B, 0xBB, 0xAD, 0x03, 0xAD, 0x02, 0xAD, 0x03, 0x13, 0x13, 0xAD, 0x13, 0x14, 0x13,
            0x14, 0x14, 0x14, 0x14, 0x13, 0xAD, 0x15, 0xBC, 0x60, 0x23, 0x13, 0x72, 0xAE, 0x60,
            0x12, 0x14, 0xAD, 0x14,
        ];
        match IdPakFileEntry::try_from(&random_bytes[..]) {
            Err(err) => assert_eq!(err, "Not enough data for file entry."),
            Ok(_) => panic!("Test not passed!"),
        };
    }

    #[test]
    fn test_file_entry_from_byte_slice_inv_size() {
        let extra_byte: [u8; 65] = [
            0x73, 0x6F, 0x75, 0x6E, 0x64, 0x2F, 0x69, 0x74, 0x65, 0x6D, 0x73, 0x2F, 0x72, 0x5F,
            0x69, 0x74, 0x65, 0x6D, 0x31, 0x2E, 0x77, 0x61, 0x76, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x0C, 0x00, 0x00, 0x00, 0xA6, 0x1A, 0x00, 0x00, 0x00,
        ];
        match IdPakFileEntry::try_from(&extra_byte[..]) {
            Err(err) => assert_eq!(err, "Invalid size."),
            Ok(_) => panic!("Test not passed!"),
        };
    }

    #[test]
    fn test_file_entry_get_name() {
        let file_entry = IdPakFileEntry::try_from(&VALID_FILE_ENTRY[..]).unwrap();
        assert_eq!(
            file_entry.get_name(),
            String::from("sound/items/r_item1.wav")
        );
    }
}
