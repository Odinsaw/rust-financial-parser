use anyhow::{Context, Result};
use clap::{Arg, ArgAction, Command};
use parser::{
    Camt053, FinancialDataRead, FinancialDataWrite, Mt940, ParserError, SupportedFormats,
};
use std::fs::File;
use std::io::{self, Write};

fn main() -> Result<()> {
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

    let input_path = matches.get_one::<String>("input").unwrap();
    let output_path = matches.get_one::<String>("output").unwrap();
    let in_format: SupportedFormats = matches
        .get_one::<String>("in-format")
        .unwrap()
        .parse()
        .unwrap();

    let out_format: SupportedFormats = matches
        .get_one::<String>("out-format")
        .map(|s| s.parse().unwrap())
        .unwrap_or(in_format.clone());
    let verbose = matches.get_flag("verbose");

    if verbose {
        eprintln!("Reading from: {}", input_path);
        eprintln!("Input format: {:?}", in_format);
        eprintln!("Output format: {:?}", out_format);
        eprintln!("Writing to: {}", output_path);
    }

    // Process conversion
    process_conversion(input_path, output_path, &in_format, &out_format, verbose)
        .context("Conversion failed")?;

    if verbose {
        eprintln!("Conversion completed successfully");
    }

    Ok(())
}

fn process_conversion(
    input_path: &str,
    output_path: &str,
    in_format: &SupportedFormats,
    out_format: &SupportedFormats,
    verbose: bool,
) -> Result<(), ParserError> {
    // If formats are the same, just copy the data
    if in_format == out_format {
        if verbose {
            eprintln!("Input and output formats are the same, copying data as-is");
        }
        copy_input_to_output(input_path, output_path)?;
        return Ok(());
    }

    match (in_format, out_format) {
        (SupportedFormats::Mt940, SupportedFormats::Camt053) => {
            if verbose {
                eprintln!("Converting MT940 to CAMT053");
            }
            convert_mt940_to_camt053(input_path, output_path)
        }
        (SupportedFormats::Camt053, SupportedFormats::Mt940) => {
            if verbose {
                eprintln!("Converting CAMT053 to MT940");
            }
            convert_camt053_to_mt940(input_path, output_path)
        }
        _ => unreachable!(),
    }
}

fn copy_input_to_output(input_path: &str, output_path: &str) -> Result<(), ParserError> {
    let input_reader = create_reader(input_path)?;
    let output_writer = create_writer(output_path)?;

    let mut input_buf = std::io::BufReader::new(input_reader);
    let mut output_buf = std::io::BufWriter::new(output_writer);

    std::io::copy(&mut input_buf, &mut output_buf)?;
    output_buf.flush()?;

    Ok(())
}

fn convert_mt940_to_camt053(input_path: &str, output_path: &str) -> Result<(), ParserError> {
    let input_reader = create_reader(input_path)?;
    let output_writer = create_writer(output_path)?;

    let mt940 = Mt940::from_read(input_reader)?;
    let camt053_result: Result<Camt053, ParserError> = From::from(&mt940);
    let camt053 = camt053_result?;

    camt053.write_to(output_writer)?;
    Ok(())
}

fn convert_camt053_to_mt940(input_path: &str, output_path: &str) -> Result<(), ParserError> {
    let input_reader = create_reader(input_path)?;
    let output_writer = create_writer(output_path)?;

    let camt053 = Camt053::from_read(input_reader)?;
    let mt940_vec_result: Result<Vec<Mt940>, ParserError> = From::from(&camt053);
    let mt940_vec = mt940_vec_result?;

    let mut buffered_writer = std::io::BufWriter::new(output_writer);

    for (i, mt940) in mt940_vec.into_iter().enumerate() {
        if i > 0 {
            buffered_writer.write_all(b"\n\n")?;
        }
        mt940.write_to(&mut buffered_writer)?;
    }

    buffered_writer.flush()?;
    Ok(())
}

fn create_reader(input_path: &str) -> Result<Box<dyn std::io::Read>, ParserError> {
    if input_path == "-" {
        Ok(Box::new(io::stdin()))
    } else {
        let file = File::open(input_path).map_err(|e| ParserError::Io(e.to_string()))?;
        Ok(Box::new(file))
    }
}

fn create_writer(output_path: &str) -> Result<Box<dyn std::io::Write>, ParserError> {
    if output_path == "-" {
        Ok(Box::new(io::stdout()))
    } else {
        let file = File::create(output_path).map_err(|e| ParserError::Io(e.to_string()))?;
        Ok(Box::new(file))
    }
}
