use anyhow::Result;
use parser::ParserError;
use parser::SupportedFormats;
use parser::converter::convert_streams::convert_streams;
use std::env;
use std::fs::File;
use std::path::PathBuf;

fn main() -> Result<(), ParserError> {
    let manifest_dir =
        env::var("CARGO_MANIFEST_DIR").map_err(|e| ParserError::GeneralError(e.to_string()))?;

    let input_file = PathBuf::from(manifest_dir.clone())
        .join("examples")
        .join("sample.mt940");

    let output_file = PathBuf::from(manifest_dir)
        .join("examples")
        .join("output.xml");

    let input_stream = Box::new(File::open(input_file)?);
    let output_stream = Box::new(File::create(output_file)?);

    let _result = convert_streams(
        input_stream,
        SupportedFormats::Mt940,
        output_stream,
        SupportedFormats::Xml,
    )?;

    println!("Conversion CAMT053 -> MT940 completed!");
    Ok(())
}
