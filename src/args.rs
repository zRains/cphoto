use clap::Parser;

#[derive(Parser)]
pub struct EncodeCmdArgs {
    /// Input file path.
    #[arg(short)]
    pub file_path: String,

    /// Chunk type to append.
    #[arg(short)]
    pub chunk_type: String,

    /// Message to append.
    #[arg(short)]
    pub message: String,

    /// Output file path.
    #[arg(short)]
    pub output_file_path: Option<String>,
}

#[derive(Parser)]
pub struct DecodeCmdArgs {
    /// Input file path.
    #[arg(short)]
    pub file_path: String,

    /// Chunk type for decode.
    #[arg(short)]
    pub chunk_type: String,
}

#[derive(Parser)]
pub struct RemoveCmdArgs {
    /// Input file path.
    #[arg(short)]
    pub file_path: String,

    /// Chunk type for remove.
    #[arg(short)]
    pub chunk_type: String,
}

#[derive(Parser)]
pub struct PrintCmdArgs {
    /// Input file path.
    #[arg(short)]
    pub file_path: String,
}
