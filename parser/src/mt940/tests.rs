use crate::mt940::format::{BasicHeaderBlock, Mt940};
use crate::traits::{FinancialDataRead, FinancialDataWrite};

#[test]
fn test_basic_header_constructor() {
    let header_str = String::from("F01GSCRUS30XXXX3614000002");
    let header_str_bigger = String::from("F01GSCRUS30XXXX36140000021");
    let header_str_less = String::from("F01GSCRUS30XXXX361400000");

    let target = BasicHeaderBlock {
        application_identifier: "F".to_string(),
        service_identifier: "01".to_string(),
        lt_identifier: "GSCRUS30XXXX".to_string(),
        session_number: "3614".to_string(),
        sequence_number: "000002".to_string(),
    };

    assert_eq!(
        target,
        BasicHeaderBlock::from_string(&header_str).expect("Failed to parse header")
    );
    assert!(BasicHeaderBlock::from_string(&header_str_bigger).is_err());
    assert!(BasicHeaderBlock::from_string(&header_str_less).is_err());
}

use std::fs::File;
use std::path::Path;

#[test]
fn test_with_file() {
    let path = std::path::Path::new(r"test_data");
    let valid_case1 = File::open(path.join("valid1.mt940")).unwrap();
    let valid_case2 = File::open(path.join("valid2.mt940")).unwrap();
    let invalid_case1 = File::open(path.join("invalid1.mt940")).unwrap();
    let invalid_case2 = File::open(path.join("invalid2.mt940")).unwrap();

    let mt940_valid1 = Mt940::from_read(valid_case1);
    let mt940_valid2 = Mt940::from_read(valid_case2);
    let mt940_invalid1 = Mt940::from_read(invalid_case1);
    let mt940_invalid2 = Mt940::from_read(invalid_case2);

    assert!(mt940_valid1.is_ok());
    assert!(mt940_valid2.is_ok());
    assert!(mt940_invalid1.is_err());
    assert!(mt940_invalid2.is_err());
    assert_ne!(mt940_valid1.unwrap(), mt940_valid2.unwrap());
}

#[test]
fn test_read_write() {
    // file paths: new file that will be created and valid mt940 file to compare
    let new_file_path = Path::new(r"test_data\test_write.mt940");
    let target_file_path = Path::new(r"test_data\valid1.mt940");
    // files
    let new_file = File::create(new_file_path).unwrap();
    let target_file = File::open(target_file_path).unwrap();
    // load valid mt940 file to struct (read tests suggest this operation is correct)
    // then serialize and write to new file
    let mt940_valid = Mt940::from_read(target_file).unwrap();
    let _ = mt940_valid.write_to(new_file).unwrap();
    // load new file and check that deserialization is correct
    let new_file = File::open(new_file_path).unwrap();
    let read_from_new_file = Mt940::from_read(new_file).unwrap();
    std::fs::remove_file(new_file_path).unwrap();
    assert_eq!(read_from_new_file, mt940_valid);
}
