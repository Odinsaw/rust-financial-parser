/// Module for stream-based conversion between supported financial statement formats.
///
/// Provides functions to convert data between formats such as MT940 and CAMT.053
/// directly from input/output streams without requiring intermediate files.
pub mod convert_streams;

/// Module for converting data to the **CAMT.053** format.
///
/// Contains utilities and implementations that transform supported input formats
/// (such as MT940) into CAMT.053 XML structures.
pub mod to_camt053;

/// Module for converting data to the **MT940** format.
///
/// Includes logic for transforming CAMT.053 or other supported formats
/// into SWIFT MT940 text statements.
pub mod to_mt940;

/// Module for converting data to the **Xml** format.
///
/// Includes logic for transforming CAMT.053 or other supported formats
/// into xml document.
pub mod to_xml;

/// Helper wrapper for mt940 - xml conversions
mod mt940xml_wrapper;
