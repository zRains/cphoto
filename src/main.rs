mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use clap::Parser;
use commands::{Cmd, Commands};
use std::io::Error;

fn main() -> Result<(), Error> {
    let cmds = Commands::parse();

    match cmds.cmd {
        Cmd::Encode(args) => Commands::encode(&args)?,
        Cmd::Decode(args) => println!("Decode message:\n{}", Commands::decode(&args)?),
        Cmd::Remove(args) => Commands::remove(&args)?,
        Cmd::Print(args) => println!("{}", Commands::print(&args)?),
    };

    Ok(())
}
