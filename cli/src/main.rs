#![warn(missing_docs)]

mod errors;

use anyhow::{Context, Result};
use clap::{Arg, ArgAction, Command};
use errors::CliError;
use parser::SupportedFormats;
use parser::converter::convert_streams::convert_streams;
use std::fs::File;
use std::io;

/// Entry point for the CLI application.
///
/// Parses command-line arguments, sets up input and output streams, and
/// performs conversion between supported financial formats (e.g., MT940, CAMT.053, CSV, XML).
///
/// # Command-line arguments
///
/// - `-i, --input <FILE>`: Input file (use `-` or omit for stdin). Default: `-`.
/// - `-o, --output <FILE>`: Output file (use `-` or omit for stdout). Default: `-`.
/// - `--in-format <FORMAT>`: Input format (required). Options: `"mt940"`, `"camt053"`, `"xml"`, `"csv"`.
/// - `--out-format <FORMAT>`: Output format. Defaults to the same as input format.
/// - `-v, --verbose`: Enable verbose output.
///
/// # Behavior
///
/// - Reads the input stream and parses it according to the input format.
/// - Converts the data to the requested output format.
/// - Writes the result to the output stream.
/// - If verbose mode is enabled, prints detailed information to stderr.
///
/// # Errors
///
/// Returns a [`CliError`] in the following cases:
/// - Invalid or missing command-line arguments (`ArgsError`).
/// - Input/output errors (`Io`).
/// - Parsing or conversion failures (`ParserError` or `ConversionError`).
fn main() -> Result<(), CliError> {
    let matches = Command::new("financial-parcer")
        .version("1.0")
        .about("CLI utility for converting between MT940 and CAMT053 formats")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Input file (omit or use - for stdin)")
                .default_value("-"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output file (omit or use - for stdout)")
                .default_value("-"),
        )
        .arg(
            Arg::new("in-format")
                .long("in-format")
                .value_name("FORMAT")
                .value_parser(["mt940", "camt053", "xml", "csv"])
                .required(true)
                .help("Input format"),
        )
        .arg(
            Arg::new("out-format")
                .long("out-format")
                .value_name("FORMAT")
                .value_parser(["mt940", "camt053", "xml", "csv"])
                .help("Output format (defaults to the same as input format)"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help("Enable verbose output"),
        )
        .get_matches();

    let input_path = matches
        .get_one::<String>("input")
        .ok_or_else(|| CliError::ArgsError("Failed to parse 'input' argument".to_string()))?;
    let output_path = matches
        .get_one::<String>("output")
        .ok_or_else(|| CliError::ArgsError("Failed to parse 'output' argument".to_string()))?;

    let in_format_str = matches
        .get_one::<String>("in-format")
        .ok_or_else(|| CliError::ArgsError("Missing 'in-format' argument".to_string()))?;
    let in_format: SupportedFormats = in_format_str
        .parse()
        .map_err(|e| CliError::ArgsError(format!("Invalid format: {}", e)))?;

    let out_format_str = matches
        .get_one::<String>("out-format")
        .unwrap_or(&in_format_str);
    let out_format: SupportedFormats = out_format_str
        .parse()
        .map_err(|e| CliError::ArgsError(format!("Invalid format: {}", e)))?;
    let verbose = matches.get_flag("verbose");

    if verbose {
        eprintln!("Reading from: {}", input_path);
        eprintln!("Input format: {:?}", in_format);
        eprintln!("Output format: {:?}", out_format);
        eprintln!("Writing to: {}", output_path);
    }

    let input_stream = create_reader(input_path)?;
    let output_stream = create_writer(output_path)?;
    // Process conversion
    convert_streams(input_stream, in_format, output_stream, out_format)
        .map_err(|e| CliError::ConversionError(e.to_string()))?;

    if verbose {
        eprintln!("Conversion completed successfully");
    }

    Ok(())
}

/// Creates a boxed reader from the specified input path.
///
/// If the input path is `"-"`, returns a reader for stdin; otherwise, opens the file.
///
/// # Errors
///
/// Returns a [`CliError::Io`] if the file cannot be opened.
fn create_reader(input_path: &str) -> Result<Box<dyn std::io::Read>, CliError> {
    if input_path == "-" {
        Ok(Box::new(io::stdin()))
    } else {
        let file = File::open(input_path).map_err(|e| CliError::Io(e))?;
        Ok(Box::new(file))
    }
}

/// Creates a boxed writer for the specified output path.
///
/// If the output path is `"-"`, returns a writer for stdout; otherwise, creates/truncates the file.
///
/// # Errors
///
/// Returns a [`CliError::Io`] if the file cannot be created.
fn create_writer(output_path: &str) -> Result<Box<dyn std::io::Write>, CliError> {
    if output_path == "-" {
        Ok(Box::new(io::stdout()))
    } else {
        let file = File::create(output_path).map_err(|e| CliError::Io(e))?;
        Ok(Box::new(file))
    }
}
