use crate::chunk_type::ChunkType;
use crc::{Crc, CRC_32_ISO_HDLC};
use std::io::{Error, ErrorKind};

pub struct Chunk {
    pub c_length: u32,
    pub c_type: ChunkType,
    pub c_data: Vec<u8>,
    pub c_crc: u32,
}

fn calc_crc(bytes: &[u8]) -> u32 {
    let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    let mut digest = crc.digest();

    digest.update(bytes);

    digest.finalize()
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

        let c_type = ChunkType::try_from(TryInto::<[u8; 4]>::try_into(&value[4..8]).unwrap())?;
        let c_data = value[8..(value.len() - 4) as usize].to_vec();
        let c_crc =
            u32::from_be_bytes(TryInto::<[u8; 4]>::try_into(&value[value.len() - 4..]).unwrap());

        if calc_crc(&[&value[4..8], &c_data].concat()) != c_crc {
            return Err(Error::new(ErrorKind::Unsupported, "CRC check failed"));
        }

        Ok(Self {
            c_length,
            c_type,
            c_data,
            c_crc,
        })
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.c_data).unwrap())
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let c_crc = calc_crc(&[chunk_type.bytes().as_slice(), &data].concat());

        Self {
            c_length: data.len() as u32,
            c_type: chunk_type,
            c_data: data,
            c_crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.c_length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.c_type
    }

    pub fn data(&self) -> &[u8] {
        &self.c_data
    }

    pub fn crc(&self) -> u32 {
        self.c_crc
    }

    pub fn data_as_string(&self) -> Result<String, Error> {
        match std::str::from_utf8(&self.c_data) {
            Ok(s) => Ok(s.into()),
            Err(_) => Err(Error::new(
                ErrorKind::Unsupported,
                "Can not convert into string",
            )),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        [
            u32::to_be_bytes(self.c_length).as_slice(),
            self.c_type.bytes().as_slice(),
            &self.c_data,
            u32::to_be_bytes(self.c_crc).as_slice(),
        ]
        .concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
