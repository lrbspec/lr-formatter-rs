use clap::Parser;
use std::fs::File;
use std::io::{Read, Write};
use std::error::Error;
use lr_formatter_rs::{convert, format::Format};

#[derive(Parser)]
#[command(version = "1.0", author = "Your Name <your@email.com>", about = "CLI for converting Line Rider file formats")]
struct Cli {
    input: String,
    from: String,
    to: String,
    output: Option<String>,
}

fn parse_format(format: &str) -> Result<Format, Box<dyn Error>> {
    match format.to_lowercase().as_str() {
        "trackjson" => Ok(Format::TrackJson),
        "lrb" => Ok(Format::LRB),
        _ => Err("Unsupported format".into()),
    }
}

fn main() {
    let args = Cli::parse();

    let from_format = parse_format(&args.from).expect("Invalid from format");
    let to_format = parse_format(&args.to).expect("Invalid to format");

    let mut input_data = Vec::new();
    File::open(&args.input).expect("Failed to open input file").read_to_end(&mut input_data).expect("Failed to read input file");

    match convert(&input_data, from_format, to_format) {
        Ok(converted_data) => {
            if let Some(ref output) = args.output {
                File::create(output).expect("Failed to create output file").write_all(&converted_data).expect("Failed to write output file");
                println!("Converted file saved to {}", output);
            } else {
                println!("{}", String::from_utf8_lossy(&converted_data));
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}