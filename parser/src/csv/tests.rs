use crate::*;

use std::env;
use std::fs::File;
use std::path::PathBuf;

#[test]
fn test_read_write() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = PathBuf::from(manifest_dir).join("test_data");
    let new_file_path = path.join("new_file.csv");

    let csv_data = "name,age\nAlice,30\nBob,25\nCharlie,40";
    let csv = CsvWrapper::from_string(csv_data).unwrap();

    let write_csv_file = File::create(&new_file_path).unwrap();
    let _ = csv.write_to(write_csv_file).unwrap();

    let read_csv_file = File::open(&new_file_path).unwrap();
    let read_csv = CsvWrapper::from_read(read_csv_file).unwrap();
    std::fs::remove_file(&new_file_path).unwrap();
    assert_eq!(csv, read_csv);
}
