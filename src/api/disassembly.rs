use pe_parser::pe::{parse_portable_executable, PortableExecutable};
use pe_parser::section::{self, SectionHeader};
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Error;
use Vec;

/*
    Read file into a byte string
*/
pub fn read_file_bytes(path: &str) -> Result<Vec<u8>, Error> {
    let mut file = File::open(path)?;
    let mut bytestring = Vec::new();

    file.read_to_end(&mut bytestring)?;
    Ok(bytestring)
}

pub fn get_text_header(bytestring: &Vec<u8>) -> Result<SectionHeader, pe_parser::Error> {
    let pe = parse_portable_executable(bytestring)?;
    let mut headerResult: Option<SectionHeader> = None;

    for section in pe.section_table {
        if let Ok(section_name) = std::str::from_utf8(&section.name)  {
            if section_name.contains(".text") {
                headerResult = Some(section);
                break;
            }
        }
    }
    
    match headerResult {
        Some(textSection) => {
            println!("{}", textSection);
            return Ok(textSection)
        }
        None => {
            println!("No .text section found. :(");
        }
    }

    Err(pe_parser::Error::MissingCoffHeader)
}

pub fn parse_raw_text(textHeader: &SectionHeader, rawBytes: &Vec<u8>) -> Vec<u8> {
    let rawDataStart = textHeader.pointer_to_raw_data;
    
    // slice
    let remainder = rawBytes[rawDataStart as usize..].to_vec();
    return remainder;
}

pub fn run() {
    println!("Run Disassembler");
}