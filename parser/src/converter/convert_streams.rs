use crate::Camt053;
use crate::Mt940;
use crate::ParserError;
use crate::SupportedFormats;
use crate::traits::FinancialDataRead;
use crate::traits::FinancialDataWrite;

use std::io::Write;

pub fn convert_streams(
    input_stream: Box<dyn std::io::Read>,
    input_format: SupportedFormats,
    output_stream: Box<dyn std::io::Write>,
    output_format: SupportedFormats,
) -> Result<(), ParserError> {
    // If formats are the same, just copy the data
    if input_format == output_format {
        copy_input_to_output(input_stream, output_stream)?;
        return Ok(());
    }

    match (input_format, output_format) {
        (SupportedFormats::Mt940, SupportedFormats::Camt053) => {
            convert_mt940_to_camt053(input_stream, output_stream)
        }
        (SupportedFormats::Camt053, SupportedFormats::Mt940) => {
            convert_camt053_to_mt940(input_stream, output_stream)
        }
        _ => Err(ParserError::Converter(format!(
            "Unsupported format conversion: {} to {}",
            input_format.to_string(),
            output_format.to_string(),
        ))),
    }
}

pub fn convert_mt940_to_camt053(
    input_stream: Box<dyn std::io::Read>,
    output_stream: Box<dyn std::io::Write>,
) -> Result<(), ParserError> {
    let mt940 = Mt940::from_read(input_stream)?;
    let camt053_result: Result<Camt053, ParserError> = TryFrom::try_from(&mt940);
    let camt053 = camt053_result?;

    camt053.write_to(output_stream)?;
    Ok(())
}

pub fn convert_camt053_to_mt940(
    input_stream: Box<dyn std::io::Read>,
    output_stream: Box<dyn std::io::Write>,
) -> Result<(), ParserError> {
    let camt053 = Camt053::from_read(input_stream)?;
    let mt940_vec_result: Result<Vec<Mt940>, ParserError> = TryFrom::try_from(&camt053);
    let mt940_vec = mt940_vec_result?;

    let mut buffered_writer = std::io::BufWriter::new(output_stream);

    for (i, mt940) in mt940_vec.into_iter().enumerate() {
        if i > 0 {
            buffered_writer.write_all(b"\n\n")?;
        }
        mt940.write_to(&mut buffered_writer)?;
    }

    buffered_writer.flush()?;
    Ok(())
}

fn copy_input_to_output(
    input_stream: Box<dyn std::io::Read>,
    output_stream: Box<dyn std::io::Write>,
) -> Result<(), ParserError> {
    let mut input_buf = std::io::BufReader::new(input_stream);
    let mut output_buf = std::io::BufWriter::new(output_stream);

    std::io::copy(&mut input_buf, &mut output_buf)?;
    output_buf.flush()?;

    Ok(())
}
