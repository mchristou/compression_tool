use clap::Parser;
use compression_tool::{decoder, encoder, DecodeOpt, EncodeOpt};

#[derive(Parser, Debug)]
#[clap(name = "compression tool", about = "Encodes/Decodes input file")]
enum Args {
    #[clap(short_flag = 'e')]
    /// Encode the input file
    Encode(EncodeOpt),

    #[clap(short_flag = 'd')]
    /// Decode the input file
    Decode(DecodeOpt),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args {
        Args::Encode(opt) => encoder::encode(opt)?,
        Args::Decode(opt) => decoder::decode(opt)?,
    };

    Ok(())
}
