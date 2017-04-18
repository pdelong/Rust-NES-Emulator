pub struct Memory<'a> {
    ram: Vec<u8>,
    cart: &'a ::cartridge::Cartridge,
}

// For now I'm just gonna assume NROM-128 because I'm just focusing on Donkey Kong
impl<'a> Memory<'a> {
    pub fn new(cart: &'a ::cartridge::Cartridge) -> Memory<'a> {
        Memory{ram: vec![0; 2048], cart: &cart}
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            // 2k of ram repeated 4 times
            0 ... 0x1fff => {
                let data = self.ram[address as usize % 0x800];
                //println!("Read {} from {}", data, address as usize % 0x800);

                data
            },

            0x2000 ... 0x3fff => {
                let modaddr = address % 8;
                println!("Read from PPU register: {}", modaddr);
                match modaddr {
                    0 => 0,
                    1 => 0,
                    2 => 0x80,
                    3 => 0,
                    4 => 0,
                    5 => 0,
                    6 => 0,
                    7 => 0,
                    _ => panic!("Mod 8 cannot return anything greater than 7 but somehow it did")
                }
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
                self.cart.rom[address as usize - 0x8000]
            }

            // Last 16k of ROM (just a mirror for NROM-128)
            0xC000 ... 0xFFFF => {
                // REMEMBER THIS IS A MIRROR OF THE PREVIOUS BECAUSE NROM 128
                self.cart.rom[address as usize - 0xC000]
            }
            _ => 0
        }
    }

    pub fn write(&mut self, data: u8, address: u16) {
        match address {
            // 2k of ram repeated 4 times
            0 ... 0x1fff => {
                //println!("Writing {} at {}", data, address);
                self.ram[address as usize % 0x800] = data;
                //println!("Wrote {} at {}", self.ram[address as usize % 0x800], address as usize % 0x800);
            },

            0x2000 ... 0x3fff => {
                let modaddr = address % 7;
                //println!("Write to PPU register: {}", modaddr);
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
                panic!("Attempt to write to ROM");
            }

            // Last 16k of ROM (just a mirror for NROM-128)
            0xC000 ... 0xFFFF => {
                // REMEMBER THIS IS A MIRROR OF THE PREVIOUS BECAUSE NROM 128
                panic!("Attempt to write to ROM");
            }
            _ => panic!("Whelp shit")
        }
    }
}
