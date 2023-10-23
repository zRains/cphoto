use std::fs;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let png = fs::read("./avatar.png").unwrap();

    println!(
        "{:?}",
        png.into_iter()
            .skip(8)
            .take(20)
            .map(|f| char::from_u32(f as u32))
            .collect::<Vec<_>>()
    );

    Ok(())
}
