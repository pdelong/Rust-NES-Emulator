extern crate rand;

use std::rc::Rc;
use std::cell::RefCell;

use self::rand::Rng;

pub struct PPU {
    // The cycle number of the current scanline
    cycle: usize,

    // The current scanline number
    scanline: usize,

    // Current VRAM address
    vram_addr: u16,
    temp_addr: u16,
    x: u8,

    oddframe: bool,

    write_toggle: bool,

    memory: ::memory::PPUMemoryMap,

    pub oam: [u8; 256],

    pub pixeldata: [u8; 256*240*3],

    nametablebyte: u8,

    lowtilebyte: u8,
    hightilebyte: u8,

    attributebyte: u8,

    tiledata: u64,

    // $2000 - PPU Control Register 1
    flag_table_address: u8,
    flag_vertical_write: bool,
    flag_sprite_table_address: bool,
    flag_screen_table_address: bool,
    flag_sprite_size: bool,
    /* UNUSED BIT HERE */
    flag_vblank_enable: bool,

    // $2001 - PPU Control Register 2
    /* UNKNOWN BIT HERE */
    flag_image_mask: bool,
    flag_sprite_mask: bool,
    flag_screen_enable: bool,
    flag_sprites_enable: bool,
    flag_emphasize_red: bool,
    flag_emphasize_green: bool,
    flag_emphasize_blue: bool,

    // $2002 - PPU Status Register
    /* 5 UNKNOWN BITS HERE */
    flag_overflow: bool,
    flag_hit: bool,
    flag_vblank: bool,
    pub nmi: bool,

    // $2003 - OAMADDR
    pub oamaddr: u8,

    // $2004
    // TODO: Figure this out

    // $2005
    scroll_offset: u8,

    // $2006
    memory_address_lo: u8,
    memory_address_hi: u8,
    memory_address_select: bool,

    // $2007
    // Will be handled in code
}

impl PPU {
    pub fn new(cart: Rc<RefCell<::cartridge::Cartridge>>) -> PPU {
        PPU {
            cycle: 0,
            scanline: 0,

            oam: [0; 256],
            pixeldata: [0; 256*240*3],

            memory: ::memory::PPUMemoryMap::new(cart),

            lowtilebyte: 0,
            hightilebyte: 0,

            attributebyte: 0,

            vram_addr: 0,
            temp_addr: 0,
            x: 0,
            oddframe:false,

            write_toggle: false,
            nametablebyte: 0,

            tiledata: 0,

            // $2000 - PPU Control Register 1
            flag_table_address: 0,
            flag_vertical_write: false,
            flag_sprite_table_address: false,
            flag_screen_table_address: false,
            flag_sprite_size: false,
            /* Unused bit here */
            flag_vblank_enable: false,

            // $2001 - PPU Control Register 2
            flag_image_mask: false,
            flag_sprite_mask: false,
            flag_screen_enable: false,
            flag_sprites_enable: false,
            flag_emphasize_red: false,
            flag_emphasize_green: false,
            flag_emphasize_blue: false,

            // $2002 - PPU Status Register
            flag_overflow: false,
            flag_hit: false,
            flag_vblank: false,
            nmi: false,

            // $2003 - OAMADDR
            oamaddr: 0,

            // $2005
            scroll_offset: 0,

            // $2006
            memory_address_lo: 0,
            memory_address_hi: 0,
            memory_address_select: true,
        }
    }

    fn get_background_pixel(& self) -> u8 {
        	let data = ((self.tiledata >> 32) as u32) >> ((7 - self.x) * 4);
            (data & 0x0F) as u8
    }

    pub fn step(&mut self, cycles: u8) {
        for i in 0..cycles {
            self.cycle();
        }
    }

    fn increment_x(&mut self) {
        if (self.vram_addr & 0x001F) == 31 {
            // reset addr to 0
            self.vram_addr &= 0xFFE0;
            self.vram_addr ^= 0x4000;
        } else {
            self.vram_addr += 1;
        }
    }

    fn increment_y(&mut self) {
        if self.vram_addr & 0x7000 != 0x7000 {
            self.vram_addr += 0x1000;
        } else {
            // fine Y = 0
            self.vram_addr &= 0x8FFF;
            // let y = coarse Y
            let mut y = (self.vram_addr & 0x03E0) >> 5;
            if y == 29 {
                // coarse Y = 0
                y = 0;
                // switch vertical nametable
                self.vram_addr ^= 0x0800;
            } else if y == 31 {
                // coarse Y = 0, nametable not switched
                y = 0;
            } else {
                // increment coarse Y
                y += 1;
            }
            // put coarse Y back into v
            self.vram_addr = (self.vram_addr & 0xFC1F) | (y << 5)
        }
    }

    fn copy_x(&mut self) {
        self.vram_addr = (self.vram_addr & 0xFBE0) | (self.temp_addr & 0x041F);
    }

    fn copy_y(&mut self) {
        self.vram_addr = (self.vram_addr & 0x841F) | (self.temp_addr & 0x7BE0);
    }

    fn tick(&mut self) {
        if self.flag_sprites_enable || self.flag_sprites_enable {
            if self.oddframe == true && self.scanline == 261 && self.cycle == 339 {
                self.cycle = 0;
                self.scanline = 0;
                self.oddframe = false;
                return;
            }
        }

        self.cycle += 1;

        if self.cycle > 340 {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline > 261 {
                self.scanline = 0;
                self.oddframe = if self.oddframe { false } else { true };
            }
        }
    }

    // Run one cycle
    pub fn cycle(&mut self) {
        self.tick();

        if self.scanline == 241 && self.cycle == 1 {
            // Trigger NMI
            self.flag_vblank = true;
            if (self.flag_vblank_enable) {
                self.nmi = true;
            }
        }

        if self.scanline == 261 && self.cycle == 1 {
            self.flag_vblank = false;
        }

        let enable_rendering = self.flag_screen_enable || self.flag_sprites_enable;

        let visible_line = self.scanline < 240;
        let pre_line = self.scanline == 261;
        let render_line = pre_line || visible_line;

        let visible_cycle = self.cycle >= 1 && self.cycle <= 256;
        let prefetch_cycle = self.cycle >= 321 && self.cycle <= 336;
        let fetch_cycle = prefetch_cycle || visible_cycle;

        if enable_rendering {

            if visible_cycle && visible_line {
                let offset:usize = self.scanline*256 + self.cycle - 1;
                //println!("scanline: {}, cycle: {}, offset: {}",self.scanline, self.cycle, offset);
                let mut index = self.get_background_pixel();

                self.pixeldata[offset*3 + 0] = PALETTE[(index*3) as usize + 0];
                self.pixeldata[offset*3 + 1] = PALETTE[(index*3) as usize + 1];
                self.pixeldata[offset*3 + 2] = PALETTE[(index*3) as usize + 2];
            }

            if render_line && fetch_cycle {
                self.tiledata <<= 4;
                match self.cycle % 8 {
                    0 => {
                        // Store tile data
                        let mut data: u32 = 0;
                        for i in 0..8 {
                            let a = self.attributebyte;
                            let p1 = (self.lowtilebyte & 0x80) >> 7;
                            let p2 = (self.hightilebyte & 0x80) >> 6;
                            self.lowtilebyte <<= 1;
                            self.hightilebyte <<= 1;
                            data <<= 4;
                            data |= ((a | p1 | p2) as u32);
                        }
                        self.tiledata = self.tiledata | (data as u64);
                    }

                    1 => {
                        // Fetch Name Table Byte
                        self.nametablebyte = self.memory.read(0x2000 | (self.vram_addr & 0xFFF));
                    }

                    3 => {
                        // Fetch Attribute Table Byte
                        let address = 0x23C0 | (self.vram_addr & 0x0C00) | ((self.vram_addr >> 4) & 0x38) | ((self.vram_addr >> 2) & 0x07);
                        let shift = ((self.vram_addr >> 4) & 4) | (self.vram_addr & 2);
                        self.attributebyte = ((self.memory.read(address) >> shift) & 3) << 2;

                    }

                    5 => {
                        // Fetch Low Tile Byte
                        let fine_y = (self.vram_addr >> 12) & 7;
                        let baseaddr = if self.flag_screen_table_address { 0x1000 } else { 0x0 };
                        let tile = self.nametablebyte as u16;
                        let address = baseaddr as u16 + tile*16 + fine_y;
                        self.lowtilebyte = self.memory.read(address);
                    }

                    7 => {
                        // Fetch High Tile Byte
                        let fine_y = (self.vram_addr >> 12) & 7;
                        let baseaddr = if self.flag_screen_table_address { 0x1000 } else { 0x0 };
                        let tile = self.nametablebyte as u16;
                        let address = baseaddr as u16 + tile*16 + fine_y;
                        self.hightilebyte = self.memory.read(address + 8);
                    }
                    _ => {}
                }
            }

            if pre_line && self.cycle >= 280 && self.cycle <= 304 {
                self.copy_y();
            }

            if render_line {
                if fetch_cycle && self.cycle % 8 == 0 {
                    self.increment_x();
                }
                if self.cycle == 256 {
                    self.increment_y();
                }
                if self.cycle == 257 {
                    self.copy_x();
                }
            }

            // TODO: Do sprites and stuff
        }
    }

    pub fn read_control_1(&self) -> u8 {
        panic!("Attempt to read from control 1. Probably a bug")
    }

    pub fn read_control_2(&self) -> u8 {
        panic!("Attempt to read from control 2. Probably a bug")
    }

    pub fn read_status(&self) -> u8 {
        (self.flag_overflow as u8) << 5 |
        (self.flag_hit as u8)      << 6 |
        (self.flag_vblank as u8)   << 7
    }

    pub fn read_oamaddr(&self) -> u8 {
        panic!("Attempt to read from oamaddr. Probably a bug")
    }

    pub fn read_oamdata(&self) -> u8 {
        panic!("Read from oamdata not yet implemented");
    }

    pub fn read_scroll_offset(&self) -> u8 {
        panic!("Attempt to read from scroll offset. Probably a bug")
    }

    pub fn read_addr_offset(&self) -> u8 {
        panic!("Attempt to read from addr offset. Probably a bug")
    }

    pub fn read_ppudata(&self) -> u8 {
        panic!("Reading from ppudata not yet implemented");
    }


    pub fn write_control_1(&mut self, data: u8) {
        self.flag_table_address        = (data >> 0) & 0b11;
        self.flag_vertical_write       = (data >> 2) & 0b1 == 1;
        self.flag_sprite_table_address = (data >> 3) & 0b1 == 1;
        self.flag_screen_table_address = (data >> 4) & 0b1 == 1;
        self.flag_sprite_size          = (data >> 5) & 0b1 == 1;
        self.flag_vblank_enable        = (data >> 7) & 0b1 == 1;
    }

    pub fn write_control_2(&mut self, data: u8) {
        self.flag_image_mask       = (data >> 1) & 0b1 == 1;
        self.flag_sprite_mask      = (data >> 2) & 0b1 == 1;
        self.flag_screen_enable    = (data >> 3) & 0b1 == 1;
        self.flag_sprites_enable   = (data >> 4) & 0b1 == 1;
        self.flag_emphasize_red    = (data >> 5) & 0b1 == 1;
        self.flag_emphasize_blue   = (data >> 6) & 0b1 == 1;
        self.flag_emphasize_green  = (data >> 7) & 0b1 == 1;
    }

    pub fn write_status(&mut self, data: u8) {
        panic!("Attempt to read from status. Probably a bug")
    }

    pub fn write_oamaddr(&mut self, data: u8) {
        self.oamaddr = data;
    }

    pub fn write_oamdata(&mut self, data: u8) {
        panic!("Oamdata writing not yet implemented");
    }

    pub fn write_scroll_offset(&mut self, data: u8) {
        if !self.write_toggle {
            self.temp_addr = (self.temp_addr & 0xFFE0) | ((data as u16) >> 3);
            self.x = data & 0x07;
            self.write_toggle = true;
        } else {
            self.temp_addr = (self.temp_addr & 0x8FFF) | (((data as u16) & 0x07) << 12);
            self.temp_addr = (self.temp_addr & 0xFC1F) | (((data as u16) & 0xF8) << 2);
            self.write_toggle = false;
        }
    }

    pub fn write_addr_offset(&mut self, data: u8) {
        if !self.write_toggle {
            self.temp_addr = (self.temp_addr & 0x80FF) | (((data as u16) & 0x3F) << 8);
            self.write_toggle = true;
        } else {
            self.temp_addr = (self.temp_addr & 0xFF00) | (data as u16);
            self.vram_addr = self.temp_addr;
            self.write_toggle = false;
        }
    }

    pub fn write_ppudata(&mut self, data: u8) {
        self.memory.write(data, self.vram_addr);
        self.vram_addr += if self.flag_vertical_write { 32 } else { 1 };
    }
}

const PALETTE: [u8; 192] = [
0x7C,0x7C,0x7C,
0x00,0x00,0xFC,
0x00,0x00,0xBC,
0x44,0x28,0xBC,
0x94,0x00,0x84,
0xA8,0x00,0x20,
0xA8,0x10,0x00,
0x88,0x14,0x00,
0x50,0x30,0x00,
0x00,0x78,0x00,
0x00,0x68,0x00,
0x00,0x58,0x00,
0x00,0x40,0x58,
0x00,0x00,0x00,
0x00,0x00,0x00,
0x00,0x00,0x00,
0xBC,0xBC,0xBC,
0x00,0x78,0xF8,
0x00,0x58,0xF8,
0x68,0x44,0xFC,
0xD8,0x00,0xCC,
0xE4,0x00,0x58,
0xF8,0x38,0x00,
0xE4,0x5C,0x10,
0xAC,0x7C,0x00,
0x00,0xB8,0x00,
0x00,0xA8,0x00,
0x00,0xA8,0x44,
0x00,0x88,0x88,
0x00,0x00,0x00,
0x00,0x00,0x00,
0x00,0x00,0x00,
0xF8,0xF8,0xF8,
0x3C,0xBC,0xFC,
0x68,0x88,0xFC,
0x98,0x78,0xF8,
0xF8,0x78,0xF8,
0xF8,0x58,0x98,
0xF8,0x78,0x58,
0xFC,0xA0,0x44,
0xF8,0xB8,0x00,
0xB8,0xF8,0x18,
0x58,0xD8,0x54,
0x58,0xF8,0x98,
0x00,0xE8,0xD8,
0x78,0x78,0x78,
0x00,0x00,0x00,
0x00,0x00,0x00,
0xFC,0xFC,0xFC,
0xA4,0xE4,0xFC,
0xB8,0xB8,0xF8,
0xD8,0xB8,0xF8,
0xF8,0xB8,0xF8,
0xF8,0xA4,0xC0,
0xF0,0xD0,0xB0,
0xFC,0xE0,0xA8,
0xF8,0xD8,0x78,
0xD8,0xF8,0x78,
0xB8,0xF8,0xB8,
0xB8,0xF8,0xD8,
0x00,0xFC,0xFC,
0xF8,0xD8,0xF8,
0x00,0x00,0x00,
0x00,0x00,0x00,
];
