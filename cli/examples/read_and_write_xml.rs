use parser::{FinancialDataRead, FinancialDataWrite, ParserError, XmlWrapper};
use std::env;
use std::fs::File;
use std::path::PathBuf;

fn main() -> Result<(), ParserError> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let input_file = PathBuf::from(manifest_dir.clone())
        .join("examples")
        .join("sample.xml");

    let output_file = PathBuf::from(manifest_dir)
        .join("examples")
        .join("output.xml");

    let xml = XmlWrapper::from_read(File::open(input_file)?)?;
    xml.write_to(File::create(output_file)?)?;

    println!("File copied without format conversion.");
    Ok(())
}
