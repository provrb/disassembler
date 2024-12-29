use pe_parser::section::SectionHeader;

// api packages
mod api {
    pub mod disassembly;
}

fn main() {
    let bs = api::disassembly::read_file_bytes("C:\\Users\\ethan\\source\\repos\\DLL\\DLL\\out\\x64\\Client\\ComputerDefaults.exe");
    
    match bs {
        Ok(vector) => {
            let header = api::disassembly::get_text_header(&vector);
            
        }
        Err(error) => {
            eprintln!("Error! {}", error);
        }
    }
    
    
}
