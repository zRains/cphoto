use std::{io::Error, str::FromStr};

use crate::{args::EncodeCmdArgs, chunk::Chunk, chunk_type::ChunkType, png::Png};

fn read_from_file(path: &str) -> Result<Vec<u8>, Error> {
    Ok(std::fs::read(path)?)
}

pub struct Commands;

impl Commands {
    pub fn encode(args: &EncodeCmdArgs) -> Result<(), Error> {
        let mut png = Png::try_from(read_from_file(&args.file_path)?.as_slice())?;

        png.append_chunk(Chunk::new(
            ChunkType::from_str(&args.chunk_type)?,
            args.message.as_bytes().to_vec(),
        ));

        std::fs::write(
            match &args.output_file_path {
                Some(path) => path.clone(),
                None => format!(
                    "./{:?}.png",
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                ),
            },
            png.as_bytes(),
        )?;

        Ok(())
    }
}
