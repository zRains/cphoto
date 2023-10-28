use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

use clap::{Parser, Subcommand};

use crate::{
    args::{DecodeCmdArgs, EncodeCmdArgs, PrintCmdArgs, RemoveCmdArgs},
    chunk::Chunk,
    chunk_type::ChunkType,
    png::Png,
};

fn read_from_file(path: &str) -> Result<Vec<u8>, Error> {
    Ok(std::fs::read(path)?)
}

#[derive(Parser)]
#[command(
    author = "zrain",
    version = "0.1.1",
    about = "Hide some message in photo",
    propagate_version = true
)]
pub struct Commands {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Encode photo with some message
    Encode(EncodeCmdArgs),

    /// Decode photo from chunk type
    Decode(DecodeCmdArgs),

    /// Remove a chunk
    Remove(RemoveCmdArgs),

    /// Remove a chunk
    Print(PrintCmdArgs),
}

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

    pub fn decode(args: &DecodeCmdArgs) -> Result<String, Error> {
        let png = Png::try_from(read_from_file(&args.file_path)?.as_slice())?;

        match png.chunk_by_type(&args.chunk_type) {
            Some(chunk) => Ok(format!("{}", chunk.data_as_string()?)),
            None => Err(Error::new(
                ErrorKind::NotFound,
                format!("No any chunk type is {}", args.chunk_type),
            )),
        }
    }

    pub fn remove(args: &RemoveCmdArgs) -> Result<(), Error> {
        let mut png = Png::try_from(read_from_file(&args.file_path)?.as_slice())?;

        png.remove_chunk(&args.chunk_type)?;

        Ok(())
    }

    pub fn print(args: &PrintCmdArgs) -> Result<String, Error> {
        let png = Png::try_from(read_from_file(&args.file_path)?.as_slice())?;

        Ok(png.to_string())
    }
}
