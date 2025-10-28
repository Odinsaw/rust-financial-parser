use anyhow::Result;
use parser::{Camt053, FinancialDataRead, FinancialDataWrite, Mt940, ParserError};
use std::env;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use std::fs::File;

fn main() -> Result<(), ParserError> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let input_file = PathBuf::from(manifest_dir.clone())
        .join("examples")
        .join("sample.camt053");

    let output_file = PathBuf::from(manifest_dir)
        .join("examples")
        .join("output.mt940");

    let camt053 = Camt053::from_read(File::open(input_file)?)?;
    let mt940_vec: Result<Vec<Mt940>, ParserError> = From::from(&camt053);
    let mt940_vec = mt940_vec.unwrap();

    let mut writer = BufWriter::new(File::create(output_file)?);
    for (i, mt940) in mt940_vec.into_iter().enumerate() {
        if i > 0 {
            writer.write_all(b"\n\n")?;
        }
        mt940.write_to(&mut writer)?;
    }

    println!("Conversion CAMT053 -> MT940 completed!");
    Ok(())
}
