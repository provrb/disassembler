use pe_parser::pe::{parse_portable_executable, PortableExecutable};
use pe_parser::section::{self, SectionHeader};
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Error;
use std::slice::Chunks;
use Vec;
use capstone::{prelude::*, Insn, Instructions};
use std::io::copy;

#[derive(Default)]
pub struct TextHeader {
    pub header: SectionHeader,
    pub start:  u32,
    pub end:    u32,
}

/*
    Read file into a byte string
*/
pub fn read_file_bytes(path: &str) -> Result<Vec<u8>, Error> {
    let mut file = File::open(path)?;
    let mut bytestring = Vec::new();

    file.read_to_end(&mut bytestring)?;
    Ok(bytestring)
}

/**
 * Retrieve the '.text' section header
 * from a portable executables raw byte data
 */
pub fn get_text_header(bytestring: &Vec<u8>) -> Result<TextHeader, pe_parser::Error> {
    let pe = parse_portable_executable(bytestring)?;
    let mut info: TextHeader = TextHeader::default();
    
    for section in pe.section_table {
        // check if the section is .text
        if let Ok(section_name) = std::str::from_utf8(&section.name)  {
            if section_name.contains(".text") {
                info.start = section.pointer_to_raw_data;
                info.end = info.start + section.size_of_raw_data;
                info.header = section;
                return Ok(info)
            }
        }
    }

    Err(pe_parser::Error::MissingCoffHeader)
}

pub fn extract_text_bytes(textHeader: &TextHeader, rawBytes: &Vec<u8>) -> Vec<u8> {    
    let remainder = rawBytes[textHeader.start as usize..textHeader.end as usize].to_vec();
    return remainder;
}

pub fn convert_to_instructions(rawBytes: Vec<u8>, textHeader: &TextHeader) -> String {
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .syntax(arch::x86::ArchSyntax::Masm)
        .build()
        .expect("Disassembly failed.");

    let disassem = cs.disasm_all(&rawBytes, 0x0).expect("Disassembly failed.");
    
    println!("Found {} instructions", disassem.len());

    return disassem.to_string();
}

pub fn run() {
    println!("Run Disassembler");
}