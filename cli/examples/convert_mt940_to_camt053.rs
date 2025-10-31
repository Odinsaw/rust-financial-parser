use anyhow::Result;
use parser::{Camt053, FinancialDataRead, FinancialDataWrite, Mt940, ParserError};
use std::env;
use std::path::PathBuf;

use std::fs::File;

fn main() -> Result<(), ParserError> {
    let manifest_dir =
        env::var("CARGO_MANIFEST_DIR").map_err(|e| ParserError::GeneralError(e.to_string()))?;

    let input_file = PathBuf::from(manifest_dir.clone())
        .join("examples")
        .join("sample.mt940");

    let output_file = PathBuf::from(manifest_dir)
        .join("examples")
        .join("output.camt053");

    let mt940 = Mt940::from_read(File::open(input_file)?)?;
    let camt053: Result<Camt053, ParserError> = From::from(&mt940);
    let camt053 = camt053?;

    camt053.write_to(File::create(output_file)?)?;

    println!("Conversion MT940 -> CAMT053 completed!");
    Ok(())
}
