use crate::*;

use std::fs::File;
use std::path::Path;

#[test]
fn test_read_write() {
    let new_file_path = Path::new(r"test_data\new_file.xml");

    let xml_data = r#"
<company>
    <department name="Engineering">
        <employee id="1">
            <name>Alice</name>
            <age>30</age>
            <position>Software Engineer</position>
        </employee>
        <employee id="2">
            <name>Bob</name>
            <age>25</age>
            <position>DevOps Engineer</position>
        </employee>
    </department>
    <department name="Sales">
        <employee id="3">
            <name>Charlie</name>
            <age>40</age>
            <position>Sales Manager</position>
        </employee>
        <employee id="4">
            <name>Diana</name>
            <age>28</age>
            <position>Account Executive</position>
        </employee>
    </department>
</company>
"#
    .to_string();
    let xml = CsvWrapper::from_string(&xml_data).unwrap();

    let write_xml_file = File::create(new_file_path).unwrap();
    let _ = xml.write_to(write_xml_file).unwrap();

    let read_xml_file = File::open(new_file_path).unwrap();
    let read_xml = CsvWrapper::from_read(read_xml_file).unwrap();
    std::fs::remove_file(new_file_path).unwrap();
    assert_eq!(xml, read_xml);
}
