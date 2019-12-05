//! PAK header parsing.

use std::convert::{TryFrom, TryInto};
use std::mem::size_of;
use std::os::raw::c_char;

/// PAK file signature - "PACK".
const PAK_SIG: [c_char; 4] = [0x50, 0x41, 0x43, 0x4B];

/// PAK header size in bytes.
pub const HEADER_SIZE: u32 = size_of::<IdPakHeader>() as u32;

/// Signature length in bytes.
const SIG_LEN: usize = 4;

/// PAK file header
#[repr(C)]
#[derive(Debug)]
pub struct IdPakHeader {
    /// PACK", non null-terminated.
    id: [c_char; 4],

    /// Index to the beginning of the file table.
    pub offset: u32,

    /// Size of the file table.
    pub size: u32,
}

impl Default for IdPakHeader {
    fn default() -> Self {
        IdPakHeader {
            id: PAK_SIG,
            offset: HEADER_SIZE,
            size: 0,
        }
    }
}

impl TryFrom<&[u8]> for IdPakHeader {
    type Error = &'static str;

    /// Create an `IdPakHeader` from a byte slice.
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < HEADER_SIZE as usize {
            return Err("Not enough data for header.");
        }

        let mut id: [c_char; SIG_LEN] = [0; SIG_LEN];
        let (sig, tail) = bytes.split_at(SIG_LEN);
        for (i, c) in sig.iter().enumerate() {
            id[i] = *c as c_char;
        }

        if id != PAK_SIG {
            return Err("Invalid signature.");
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

        Ok(IdPakHeader {
            id,
            offset: u32::from_le_bytes(offset),
            size: u32::from_le_bytes(size),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_default() {
        let header = IdPakHeader::default();
        assert_eq!(header.id, PAK_SIG);
        assert_eq!(header.offset, HEADER_SIZE);
        assert_eq!(header.size, 0);
    }

    #[test]
    fn test_header_from_byte_slice() {
        let bytes: [u8; 12] = [
            0x50, 0x41, 0x43, 0x4B, // PACK
            0x57, 0x8A, 0x16, 0x01, // 18254423
            0xC0, 0x54, 0x00, 0x00, // 21696
        ];

        let header = IdPakHeader::try_from(&bytes[..]).unwrap();
        assert_eq!(header.id, PAK_SIG);
        assert_eq!(header.offset, 18254423);
        assert_eq!(header.size, 21696);
    }

    #[test]
    fn test_header_from_byte_slice_inv_sig() {
        let bytes: [u8; 12] = [
            0x50, 0x41, 0x43, 0x0, // PAC\0
            0x57, 0x8A, 0x16, 0x01, // 18254423
            0xC0, 0x54, 0x00, 0x00, // 21696
        ];

        let err = IdPakHeader::try_from(&bytes[..]).unwrap_err();
        assert_eq!(err, "Invalid signature.");
    }

    #[test]
    fn test_header_from_byte_slice_inv_size() {
        let bytes: [u8; 13] = [
            0x50, 0x41, 0x43, 0x4B, // PACK
            0x57, 0x8A, 0x16, 0x01, // 18254423
            0xC0, 0x54, 0x00, 0x00, 0x00,
        ];

        let err = IdPakHeader::try_from(&bytes[..]).unwrap_err();
        assert_eq!(err, "Invalid size.");
    }

    #[test]
    fn test_header_from_byte_slice_insuff_data() {
        let bytes: [u8; 10] = [
            0x50, 0x41, 0x43, 0x4B, // PACK
            0x57, 0x8A, 0x16, 0x01, // 18254423
            0xC0, 0x54,
        ];

        let err = IdPakHeader::try_from(&bytes[..]).unwrap_err();
        assert_eq!(err, "Not enough data for header.");
    }
}
