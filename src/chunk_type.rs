use std::{io::Error, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    pub ancillary_bit: u8,
    pub private_bit: u8,
    pub reserved_bit: u8,
    pub stc_bit: u8,
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let [ancillary_bit, private_bit, reserved_bit, stc_bit] = value;

        Ok(ChunkType {
            ancillary_bit,
            private_bit,
            reserved_bit,
            stc_bit,
        })
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match <&[u8] as TryInto<[u8; 4]>>::try_into(s.as_bytes()) {
            Ok(bytes) => bytes.try_into(),
            Err(err) => Err(Error::new(std::io::ErrorKind::Unsupported, err)),
        }
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_valid() {
            return writeln!(
                f,
                "{}",
                std::str::from_utf8(self.bytes().as_slice()).unwrap()
            );
        }

        return writeln!(f, "ChunkType is invalid");
    }
}

impl ChunkType {
    pub fn bit5_is_0(byte: u8) -> bool {
        byte & 0x20 == 0
    }

    pub fn is_valid_symbol(s: u8) -> bool {
        (65 <= s && s <= 90) || (97 <= s && s <= 122)
    }

    pub fn bytes(&self) -> [u8; 4] {
        [
            self.ancillary_bit,
            self.private_bit,
            self.reserved_bit,
            self.stc_bit,
        ]
    }

    pub fn is_valid(&self) -> bool {
        let is_uppercase = |n| (97 <= n && n <= 122);

        if !self
            .bytes()
            .as_slice()
            .into_iter()
            .all(|&n| ChunkType::is_valid_symbol(n))
        {
            return false;
        }

        let [a, b, c, d] = self.bytes();

        is_uppercase(c)
            && ChunkType::bit5_is_0(c)
            && [a, b, d]
                .into_iter()
                .all(|x| match (is_uppercase(x), ChunkType::bit5_is_0(x)) {
                    (true, true) | (false, false) => true,
                    _ => false,
                })
    }

    pub fn is_critical(&self) -> bool {
        self.ancillary_bit & 0x20 == 0
    }

    pub fn is_public(&self) -> bool {
        self.private_bit & 0x20 == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.reserved_bit & 0x20 == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.stc_bit & 0x20 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
