use std::rc::Rc;
use std::cell::RefCell;

pub struct PPU {
    // PPU registers
    v: u16,  // current vram address (15 bit)
    t: u16,  // temporary vram address (15 bit)
    x: u8,   // fine x scroll (3 bit)
    w: u8,   // write toggle (1 bit)
    f: u8,   // even/odd frame flag (1 bit)

    register: u8,

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

    // $2003 - OAMADDR
    oamaddr: u8,

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
            v: 0,
            t: 0,
            x: 0,
            w: 0,
            f: 0,

            register: 0,

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
            flag_vblank: true,

            // $2003 - OAMADDR
            oamaddr: 0,

            // $2005
            scroll_offset: 0,

            // $2006
            memory_address_lo: 0,
            memory_address_hi: 0,
            memory_address_select: false,
        }
    }

    pub fn step(&mut self) {

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
        self.scroll_offset = data;
    }

    pub fn write_addr_offset(&mut self, data: u8) {
        // FIXME: Mirroring
        if self.memory_address_select {
            self.memory_address_select = false;
            self.memory_address_hi = data;
        } else {
            self.memory_address_select = true;
            self.memory_address_lo = data;
        }
    }

    pub fn write_ppudata(&mut self, data: u8) {
        panic!("Write PPU data not implemented yet");
    }
}
