#![warn(missing_docs)]
#![deny(unreachable_pub)]

//! # Financial Statement Format Converter
//!
//! This crate provides tools for **reading, writing, and converting** financial
//! statement data between multiple formats, including **MT940**, **CAMT.053**, **XML**, and **CSV**.
//!
//! ## Overview
//!
//! The library offers a unified API for handling different statement formats,
//! enabling conversion pipelines and consistent error handling.
//!
//! Supported conversions include:
//!
//! - MT940 ↔ CAMT.053
//! - MT940 → XML
//! - CAMT.053 → XML
//!
//! Each format is implemented in its own module and provides parsing and
//! serialization through shared traits.
//!
//! ## Architecture
//!
//! The crate is organized around three main layers:
//!
//! - **Format modules** (`mt940`, `camt053`, `xml`, `csv`)
//!   Each defines a format-specific struct implementing
//!   [`FinancialDataRead`] and [`FinancialDataWrite`].
//!
//! - **Core traits** ([`traits`])
//!   Define the generic read/write interfaces for all supported formats.
//!
//! - **Converter logic** ([`converter`])
//!   Implements high-level conversion between formats, working directly
//!   with input/output streams.
//!
//! Errors across all modules are represented by the unified [`ParserError`] type.
//!
//! ## Extending the crate
//!
//! To add support for a new format:
//!
//! 1. Create a new module implementing [`FinancialDataRead`] and [`FinancialDataWrite`].
//! 2. Add conversions (`TryFrom` implementations) to or from existing formats.
//! 3. Register the format in [`SupportedFormats`] and extend the converter logic.

pub(crate) mod camt053;
pub(crate) mod csv;
pub(crate) mod mt940;
pub(crate) mod xml;

/// Core conversion utilities and logic shared by format converters.
///
/// Defines the main conversion flow and high-level orchestration between
/// different format handlers.
pub mod converter;

/// Module defining all error types used throughout the library.
///
/// Contains the [`ParserError`] enum and related conversions for handling
/// parsing, I/O, and format conversion errors in a consistent way.
pub mod errors;

/// Traits defining the core read/write interfaces for financial data formats.
///
/// Provides the [`FinancialDataRead`] and [`FinancialDataWrite`] traits used
/// by all supported formats to implement uniform serialization and parsing.
pub mod traits;

// Structs for internal use
pub(crate) use camt053::format::Camt053;
pub(crate) use csv::format::CsvWrapper;
pub(crate) use mt940::format::Mt940;
pub(crate) use xml::format::XmlWrapper;

pub use errors::ParserError;
pub use traits::{FinancialDataRead, FinancialDataWrite};

/// Enumeration of supported statement formats.
///
/// Defines the file formats that the library can handle when reading or exporting
/// financial statement data. Typically used to select the appropriate parser or serializer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SupportedFormats {
    /// **CSV** format — a simple text-based format using delimiters such as commas or semicolons.
    Csv,

    /// **XML** format — a structured markup format commonly used for data exchange.
    Xml,

    /// **CAMT.053** format — an ISO 20022 XML-based standard for electronic bank statements.
    Camt053,

    /// **MT940** format — a SWIFT standard format for bank statements widely used in international banking.
    Mt940,
}

impl std::str::FromStr for SupportedFormats {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mt940" => Ok(SupportedFormats::Mt940),
            "camt053" => Ok(SupportedFormats::Camt053),
            "xml" => Ok(SupportedFormats::Xml),
            "csv" => Ok(SupportedFormats::Csv),
            _ => Err(format!(
                "Unknown format: {}. Use 'mt940', 'camt053', 'xml' or 'csv'",
                s
            )),
        }
    }
}

impl SupportedFormats {
    fn to_string(&self) -> String {
        match self {
            SupportedFormats::Mt940 => "mt940".to_string(),
            SupportedFormats::Camt053 => "camt053".to_string(),
            SupportedFormats::Xml => "xml".to_string(),
            SupportedFormats::Csv => "csv".to_string(),
        }
    }
}
