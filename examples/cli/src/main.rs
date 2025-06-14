use anyhow::{Context, Result};
use clap::Parser;
use lr_formatter_rs::{convert, formats::Format};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Parser)]
#[command(
    version = "1.1.0",
    author = "LRBSpec",
    about = "CLI for converting Line Rider file formats"
)]
struct Cli {
    /// Path of the file to convert
    input_file: String,
    /// Format to convert to (json, lrb, sol)
    output_format: String,
    /// Optional output file path
    output_file: Option<String>,
}

fn parse_format(format: &str) -> Result<Format> {
    match format.to_lowercase().as_str() {
        "json" => Ok(Format::TrackJson),
        "lrb" => Ok(Format::LRB),
        "trk" => Ok(Format::TRK),
        "sol" => Ok(Format::SOL(Some(0))),
        _ => Err(anyhow::anyhow!(
            "Invalid format '{}'. Must be one of: json, lrb, trk, sol",
            format
        )),
    }
}

fn run() -> Result<()> {
    let args = Cli::parse();

    let input_path = Path::new(&args.input_file);
    let input_extension = input_path
        .extension()
        .and_then(|e| e.to_str())
        .context("Failed to parse file extension")?;
    let input_name = input_path
        .file_stem()
        .and_then(|e| e.to_str())
        .context("Failed to parse file name")?;
    let input_format = parse_format(input_extension).context("Failed to parse input format")?;
    let mut input_data = Vec::new();

    File::open(&args.input_file)
        .with_context(|| format!("Failed to open input file '{}'", &args.input_file))?
        .read_to_end(&mut input_data)
        .context("Failed to read input file")?;

    let output_format = parse_format(&args.output_format).context("Failed to parse output format")?;
    let output_extension = match output_format {
        Format::LRB => ".lrb",
        Format::SOL(_) => ".sol",
        Format::TRK => ".trk",
        Format::TrackJson => ".track.json",
    };
    let output_file_name = &args.output_file.unwrap_or(input_name.to_string() + " (Converted)" + output_extension);
    let output_data =
        &convert(&input_data, input_format, output_format).context("Conversion failed")?;

    File::create(output_file_name)
        .with_context(|| format!("Failed to create output file '{}'", output_file_name))?
        .write_all(output_data)
        .context("Failed to write output file")?;

    println!("Converted file saved to {}", output_file_name);

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{:?}", err);
        std::process::exit(1);
    }
}
