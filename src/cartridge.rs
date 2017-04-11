use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Cartridge {
    rom: Vec<u8>
}

impl Cartridge {
    pub fn new(filename: &String) -> Cartridge {
        let path = Path::new(filename);
        let display = path.display();

        let mut rom:Vec<u8> = vec![0; 0x8000];

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open {}: {}", display,
                               why.description()),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut i = 0;
        for byte in file.bytes() {
            match byte {
                Err(why) => panic!("couldn't read {}: {}", display,
                                   why.description()),
                Ok(byte) => { rom[i] = byte; i+=1; }
            }
        }
        Cartridge {rom: vec![0]}
    }
}
