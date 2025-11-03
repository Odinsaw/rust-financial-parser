use crate::Camt053;
use crate::Mt940;
use crate::ParserError;
use crate::SupportedFormats;
use crate::traits::FinancialDataRead;
use crate::traits::FinancialDataWrite;

use std::io::Write;

/// Converts data between supported financial statement formats using input and output streams.
///
/// This function reads data from the given `input_stream` in the specified `input_format`,
/// converts it to the desired `output_format`, and writes the result to the provided `output_stream`.
///
/// # Arguments
///
/// * `input_stream` — Input data source implementing [`std::io::Read`].
/// * `input_format` — The format of the input data (e.g., `SupportedFormats::Mt940`).
/// * `output_stream` — Output destination implementing [`std::io::Write`].
/// * `output_format` — The desired output format (e.g., `SupportedFormats::Camt053`).
///
/// # Behavior
///
/// - If the input and output formats are identical, the data is copied directly.
/// - If a supported conversion path exists (e.g., MT940 → CAMT.053 or vice versa),
///   the data is parsed and re-serialized.
/// - For unsupported format combinations, an error of type [`ParserError::Converter`] is returned.
///
/// # Errors
///
/// Returns a [`ParserError`] if any parsing, I/O, or conversion error occurs.
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

/// Converts a stream of **MT940** data into **CAMT.053** format.
///
/// Parses MT940 data from the input stream, converts it to a [`Camt053`] structure,
/// and writes the resulting XML to the output stream.
///
/// # Errors
///
/// Returns a [`ParserError`] if the MT940 data cannot be parsed or if writing fails.
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

/// Converts a stream of **CAMT.053** data into **MT940** format.
///
/// Parses CAMT.053 XML from the input stream, converts it to one or more [`Mt940`] records,
/// and writes them to the output stream.
///
/// # Behavior
///
/// - Multiple MT940 statements may be generated from a single CAMT.053 file.
/// - Each MT940 record is separated by two newline characters for readability.
///
/// # Errors
///
/// Returns a [`ParserError`] if the CAMT.053 data cannot be parsed, converted, or written.
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

/// Copies raw data from the input stream to the output stream.
///
/// This function is used when no conversion between formats is necessary.
///
/// # Errors
///
/// Returns a [`ParserError`] if an I/O error occurs during reading or writing.
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
