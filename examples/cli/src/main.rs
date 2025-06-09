use anyhow::{Context, Result};
use clap::Parser;
use lr_formatter_rs::{convert, formats::Format};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Parser)]
#[command(
    version = "1.0.0",
    author = "LRBSpec",
    about = "CLI for converting Line Rider file formats"
)]
struct Cli {
    input: String,
    from: String,
    to: String,
    output: Option<String>,
}

fn parse_format(format: &str) -> Result<Format> {
    match format.to_lowercase().as_str() {
        "trackjson" => Ok(Format::TrackJson),
        "lrb" => Ok(Format::LRB),
        "trk" => Ok(Format::TRK),
        "sol" => Ok(Format::SOL(Some(0))),
        _ => Err(anyhow::anyhow!(
            "Invalid format '{}'. Must be one of: trackjson, lrb, trk, sol",
            format
        )),
    }
}

fn run() -> Result<()> {
    let args = Cli::parse();

    let from_format = parse_format(&args.from).context("Failed to parse 'from' format")?;
    let to_format = parse_format(&args.to).context("Failed to parse 'to' format")?;

    let mut input_data = Vec::new();
    File::open(&args.input)
        .with_context(|| format!("Failed to open input file '{}'", &args.input))?
        .read_to_end(&mut input_data)
        .context("Failed to read input file")?;

    let converted_data =
        convert(&input_data, from_format, to_format).context("Conversion failed")?;

    if let Some(ref output) = args.output {
        File::create(output)
            .with_context(|| format!("Failed to create output file '{}'", output))?
            .write_all(&converted_data)
            .context("Failed to write output file")?;

        println!("Converted file saved to {}", output);
    } else {
        println!("{}", String::from_utf8_lossy(&converted_data));
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
