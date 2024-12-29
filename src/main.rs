// imports
use std::{fs::File, io::Write, os::raw};
use api::disassembly::convert_to_instructions;
use std::error::Error;
use std::{default, env};

// api packages
mod api {
    pub mod disassembly;
}

#[derive(Default)]
pub struct Settings {
    pub savePath: String,
    pub inputPath: String,
}

fn parse_args(args: Vec<String>) -> Result<Settings, Box<dyn Error>> {
    let mut settings = Settings::default();
    
    for (i, argument) in args.iter().enumerate() {
        if argument == "-out" && args.len() > i + 1 {
            settings.savePath = args[i + 1].clone();
        }
        else if argument == "-in" && args.len() > i + 1 {
            settings.inputPath = args[i + 1].clone();
        }
    }

    if !std::fs::exists(&settings.inputPath)? {
        panic!("Input file doesn't exist. Quitting.");
    }

    Ok(settings)
}

fn main() -> Result<(), Box<dyn Error>> {
    let settings         = parse_args(env::args().collect())?;
    let rawBytes      = api::disassembly::read_file_bytes(&settings.inputPath)?;
    let textHeader = api::disassembly::get_text_header(&rawBytes)?;
    let textBytes     = api::disassembly::extract_text_bytes(&textHeader, &rawBytes);
    let instructions   = api::disassembly::convert_to_instructions(textBytes, &textHeader);

    println!(".text header starts at {} and ends at {}", textHeader.start, textHeader.end);

    let mut saveFile = File::create(&settings.savePath)?;
    let mut written = saveFile.write(instructions.as_bytes());
    
    match written {
        Ok(size) => {
            println!("Wrote {} bytes of disassembled code to {}.", size, &settings.savePath);
            Ok(())
        }
        Err(error) => {
            Err(Box::new(error))
        }
    }
}
