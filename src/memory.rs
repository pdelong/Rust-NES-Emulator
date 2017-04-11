pub struct Memory<'a> {
    ram: Vec<u8>,
    cart: &'a ::cartridge::Cartridge,
}

impl<'a> Memory<'a> {
    pub fn new(cart: &'a ::cartridge::Cartridge) -> Memory<'a> {
        Memory{ram: vec![0; 2048], cart: &cart}
    }

    pub fn read16(&self, address: u16) -> u8 {
        match address {
            // 2k of ram repeated 4 times
            0 ... 0x1fff => {
                self.ram[address as usize % 0x800]
            },

            0x2000 ... 0x3fff => {
                0
            },

            0x4000 ... 0x4017 => {
                0
            },

            0x4018 ... 0x401F => {
                0
            },

            0x4020 ... 0xFFFF => {
                0
            },
            _ => 0
        }
    }

    pub fn write16(address: u16) {
    }
}
