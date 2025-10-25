use crate::FinancialDataWrite;
use crate::ParserError;
use crate::XmlWrapper;
use quick_xml::se::to_string;

impl XmlWrapper {
    pub fn to_string(&self) -> Result<String, ParserError> {
        let xml_text = to_string(&self.0).map_err(|e| ParserError::Xml(e.to_string()))?;
        Ok(xml_text)
    }
}

impl FinancialDataWrite for XmlWrapper {
    fn write_to<W: std::io::Write>(&self, writer: W) -> Result<(), ParserError> {
        let data = self.to_string()?;
        Self::write_string(writer, &data)?;
        Ok(())
    }
}
