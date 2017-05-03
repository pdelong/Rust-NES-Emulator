use std::rc::Rc;
use std::cell::RefCell;

pub struct PPU {
    // The cycle number of the current scanline
    cycle: usize,

    // The current scanline number
    scanline: usize,

    // Current address
    addr: u16,

    memory: ::memory::PPUMemoryMap,

    pub oam: [u8; 256],

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

            memory: ::memory::PPUMemoryMap::new(cart),

            addr: 0,

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

    pub fn step(&mut self, cycles: u8) {
        for i in 0..cycles {
            self.cycle();
        }
    }

    // Run one cycle
    pub fn cycle(&mut self) {
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

        // We're done with one scanline and maybe a frame
        if self.cycle > 340 {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline > 261 {
                self.scanline = 0;
            }
        }

        self.cycle += 1;

        // Render a pixel

        // Fetch data from memory if necessary

        // TODO: Do sprites and stuff

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
        // FIXME: Need to write twice
        self.scroll_offset = data;
    }

    pub fn write_addr_offset(&mut self, data: u8) {
        // FIXME: Mirroring
        // FIXME: Also updating the correct internal registers
        if self.memory_address_select {
            self.memory_address_select = false;
            self.memory_address_hi = data;
        } else {
            self.memory_address_select = true;
            self.memory_address_lo = data;
            self.addr = ((self.memory_address_hi as u16) << 8) + (self.memory_address_lo as u16);
            println!("{}", self.addr);
        }
    }

    pub fn write_ppudata(&mut self, data: u8) {
        self.memory.write(data, self.addr);
        self.addr += if self.flag_vertical_write { 32 } else { 1 };
    }
}

 const PALETTE: [u8; 192] = [
    124,124,124,    0,0,252,        0,0,188,        68,40,188,
    148,0,132,      168,0,32,       168,16,0,       136,20,0,
    80,48,0,        0,120,0,        0,104,0,        0,88,0,
    0,64,88,        0,0,0,          0,0,0,          0,0,0,
    188,188,188,    0,120,248,      0,88,248,       104,68,252,
    216,0,204,      228,0,88,       248,56,0,       228,92,16,
    172,124,0,      0,184,0,        0,168,0,        0,168,68,
    0,136,136,      0,0,0,          0,0,0,          0,0,0,
    248,248,248,    60,188,252,     104,136,252,    152,120,248,
    248,120,248,    248,88,152,     248,120,88,     252,160,68,
    248,184,0,      184,248,24,     88,216,84,      88,248,152,
    0,232,216,      120,120,120,    0,0,0,          0,0,0,
    252,252,252,    164,228,252,    184,184,248,    216,184,248,
    248,184,248,    248,164,192,    240,208,176,    252,224,168,
    248,216,120,    216,248,120,    184,248,184,    184,248,216,
    0,252,252,      248,216,248,    0,0,0,          0,0,0
];
