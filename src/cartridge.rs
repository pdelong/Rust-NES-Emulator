pub struct Cartridge {
    pub ram: Box<[u8]>,
    pub rom: Vec<u8>,
}

impl Cartridge {
    pub fn new(rominfo: ::ines::INesInfo) -> Cartridge {
        Cartridge {rom: rominfo.prgrom, ram: Box::new([0;0x2000])}
    }

    pub fn read(&self, addr: u16) {

    }

    pub fn write(&mut self, data: u8, addr: u16) {

    }
}
