use crate::chunk_type::ChunkType;
use crc::{Crc, CRC_32_ISO_HDLC};
use std::io::{Error, ErrorKind};

struct Chunk {
    c_length: u32,
    c_type: ChunkType,
    c_data: Vec<u8>,
    c_crc: u32,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 12 {
            return Err(Error::new(
                ErrorKind::Unsupported,
                format!("At least 16 bytes, got {} byte(s)", value.len()),
            ));
        }

        let c_length = u32::from_be_bytes(value[..4].try_into().unwrap());

        if value.len() != (12 + c_length) as usize {
            return Err(Error::new(ErrorKind::Unsupported, "Invalid data"));
        }

        let c_type = ChunkType::try_from(TryInto::<[u8; 4]>::try_into(&value[..4]).unwrap());
        let c_data = value
            .iter()
            .skip(8)
            .take(c_length as usize)
            .map(|b| *b)
            .collect::<Vec<_>>();

        // let c_crc = value.iter().rev().take(4).collect();
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);

        let mut digest = crc.digest();

        Ok(todo!())
    }
}
