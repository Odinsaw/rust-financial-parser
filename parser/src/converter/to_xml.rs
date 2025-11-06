use crate::ParserError;
use crate::camt053::format::*;
use crate::mt940::format::*;
use crate::xml::format::*;

use crate::converter::mt940xml_wrapper::*;
use quick_xml::se;
use quick_xml::se::to_string;

impl TryFrom<&Camt053> for XmlWrapper {
    type Error = ParserError;

    fn try_from(value: &Camt053) -> Result<Self, Self::Error> {
        let xml = se::to_string(value).map_err(|e| ParserError::Converter(e.to_string()))?;
        Ok(XmlWrapper(xml))
    }
}

impl TryFrom<&Mt940> for XmlWrapper {
    type Error = ParserError;

    fn try_from(mt940: &Mt940) -> Result<Self, Self::Error> {
        let statement_lines_xml: Vec<MT940StatementLineXml> = mt940
            .statement
            .statement_lines
            .iter()
            .map(|line| MT940StatementLineXml {
                field_61: line.field_61.clone(),
                field_86: line.field_86.clone(),
            })
            .collect();

        let statement_wrapper = MT940XmlStatement {
            field_20: mt940.statement.field_20.clone(),
            field_21: mt940.statement.field_21.clone(),
            field_25: mt940.statement.field_25.clone(),
            field_28c: mt940.statement.field_28c.clone(),
            field_60f: mt940.statement.field_60f.clone(),
            statement_lines: statement_lines_xml,
            field_62f: mt940.statement.field_62f.clone(),
            field_64: mt940.statement.field_64.clone(),
            field_65: mt940.statement.field_65.clone(),
        };

        let wrapper = Mt940Xml {
            basic_header: mt940.basic_header.clone(),
            application_header: mt940.application_header.clone(),
            user_header: mt940.user_header.clone(),
            statement: statement_wrapper,
            footer: mt940.footer.clone(),
        };

        to_string(&wrapper)
            .map(XmlWrapper)
            .map_err(|e| ParserError::Converter(format!("XML conversion error: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::FinancialDataRead;
    use std::env;
    use std::fs::File;
    use std::path::PathBuf;

    #[test]
    fn test_convert_camt053_to_xml() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = PathBuf::from(manifest_dir).join("test_data");
        let target_file_path = path.join("valid1.camt053");
        let target_file = File::open(target_file_path).unwrap();
        let camt053_valid = Camt053::from_read(target_file).unwrap();

        let result: Result<XmlWrapper, ParserError> = (&camt053_valid).try_into();
        let result = result.unwrap();
    }

    #[test]
    fn test_convert_mt940_to_xml() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = PathBuf::from(manifest_dir).join("test_data");
        let target_file_path = path.join("simple.mt940");
        let target_file = File::open(target_file_path).unwrap();
        let mt940_valid = Mt940::from_read(target_file).unwrap();

        let result: Result<XmlWrapper, ParserError> = (&mt940_valid).try_into();
        let result = result.unwrap();

        // Expected XML representation
        assert_eq!(result.0.clone(), "<Mt940Xml><basic_header>\
        F01BANKDEFFAXXX0000000000</basic_header><application_header>I940BANKNL2AXXXXN\
        </application_header><user_header/><statement><field20><reference>STAT202510210001</reference>\
        </field20><field21/><field25><authorisation>NL91ABNA0417164300</authorisation></field25><field28C>\
        <statement_number>1</statement_number><sequence_number>1</sequence_number></field28C><field60F><debit_credit_mark>C</debit_credit_mark>\
        <value_date>2025-10-20</value_date><currency>EUR</currency><amount>10000</amount>\
        </field60F><transactions><field61><value_date>2025-10-21</value_date><entry_date>1021</entry_date>\
        <debit_credit_mark>D</debit_credit_mark><funds_code/><amount>250</amount><transaction_type>NTRF</transaction_type>\
        <customer_reference>NONREF</customer_reference><bank_reference>BKNTRX0001</bank_reference><supplementary_details/>\
        </field61><field86><narrative>Payment to supplier Roga i Kopyta Inv 1001</narrative></field86></transactions>\
        <field62F><debit_credit_mark>C</debit_credit_mark><value_date>2025-10-21</value_date><currency>EUR</currency>\
        <amount>11239.5</amount></field62F><field64/><field65/></statement><footer/></Mt940Xml>".to_string());
    }
}
