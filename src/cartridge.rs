pub struct Cartridge {
    pub ram: Vec<u8>,
    pub rom: Vec<u8>,
}

impl Cartridge {
    pub fn new(rominfo: ::ines::INesInfo) -> Cartridge {
        Cartridge {rom: rominfo.prgrom, ram: vec![0;0x2000]}
    }
}
