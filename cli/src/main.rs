mod errors;

use anyhow::{Context, Result};
use clap::{Arg, ArgAction, Command};
use errors::CliError;
use parser::SupportedFormats;
use parser::converter::convert_streams::convert_streams;
use std::fs::File;
use std::io;

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

fn create_reader(input_path: &str) -> Result<Box<dyn std::io::Read>, CliError> {
    if input_path == "-" {
        Ok(Box::new(io::stdin()))
    } else {
        let file = File::open(input_path).map_err(|e| CliError::Io(e))?;
        Ok(Box::new(file))
    }
}

fn create_writer(output_path: &str) -> Result<Box<dyn std::io::Write>, CliError> {
    if output_path == "-" {
        Ok(Box::new(io::stdout()))
    } else {
        let file = File::create(output_path).map_err(|e| CliError::Io(e))?;
        Ok(Box::new(file))
    }
}
