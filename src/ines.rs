use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct INesInfo {
    pub prgunits: u8,
    pub chrunits: u8,
    pub prgrom: Vec<u8>,
    pub chrrom: Vec<u8>,
}

impl INesInfo {
    pub fn new(filename: &String) -> INesInfo {
        let mut result = read_file(filename);
        if !check_header(&result) { panic!("Invalid nes file:"); }

        let prgunits = result[4];
        let chrunits = result[5];

        let prgsize = 0x4000 * prgunits as usize; 
        let chrsize = 0x2000 * chrunits as usize;

        let mut rest1 = result.split_off(16);
        let header = result;
        let mut rest2 = rest1.split_off(prgsize as usize);
        let prgrom = rest1;
        let chrrom = rest2;

        INesInfo{prgunits: prgunits, chrunits: chrunits, prgrom, chrrom: chrrom}
    }
}

fn check_header(file: &Vec<u8>) -> bool {
    file[0] == 0x4E && file[1] == 0x45 && file[2] == 0x53 && file[3] == 0x1A
}


fn read_file(filename: &String) -> Vec<u8> {
    let path = Path::new(filename);
    let display = path.display();

    let mut data:Vec<u8> = Vec::new();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
        Ok(file) => file,
    };

    match file.read_to_end(&mut data) {
        Err(why) => panic!("Couldn't read from {}: {}", display, why.description()),
        Ok(_) => data
    }

}
