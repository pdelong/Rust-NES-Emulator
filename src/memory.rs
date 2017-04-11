pub struct Memory<'a> {
    ram: Vec<u8>,
    cart: &'a ::cartridge::Cartridge,
}

// For now I'm just gonna assume NROM-128 because I'm just focusing on Donkey Kong
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
                panic!("0x2000 ... 0x3fff");
            },

            0x4000 ... 0x4017 => {
                panic!("0x4000 ... 0x4017");
            },

            0x4018 ... 0x401F => {
                panic!("0x4018 ... 0x401F");
            },

            // PRG-RAM
            0x6000 ... 0x7FFF => {
                panic!("PRG-RAM");
            },

            // First 16k of ROM
            0x8000 ... 0xBFFF => {
                self.cart.rom[address - 0x8000];
            }

            // Last 16k of ROM (just a mirror for NROM-128)
            0xC000 ... 0xFFFF => {
                // REMEMBER THIS IS A MIRROR OF THE PREVIOUS BECAUSE NROM 128
                self.cart.rom[address - 0xC000];
            }
            _ => 0
        }
    }

    pub fn write16(address: u16) {
    }
}
