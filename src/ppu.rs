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
    flag_vertical_write: u8,
    flag_sprite_table_address: u8,
    flag_screen_table_address: u8,
    flag_sprite_size: u8,
    /* UNUSED BIT HERE */
    flag_vblank_enable: u8,

    // $2001 - PPU Control Register 2
    /* UNKNOWN BIT HERE */
    flag_image_mask: u8,
    flag_sprite_mask: u8,
    flag_screen_enable: u8,
    flag_sprites_enable: u8,
    flag_background_color: u8,

    // $2002 - PPU Status Register
    /* 5 UNKNOWN BITS HERE */
    flag_overflow: u8,
    flag_hit: u8,
    flag_vblank: u8,

    // $2003 - OAMADDR
    oamaddr: u8,

    // $2004
    // TODO: Figure this out

    // $2005
    scroll_offset: u8,

    // $2006
    memory_address_lo: u8,
    memory_address_hi: u8,

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

            flag_table_address: 0,
            flag_vertical_write: 0,
            flag_sprite_table_address: 0,
            flag_screen_table_address: 0,
            flag_sprite_size: 0,

            flag_vblank_enable: 0,

            // $2001 - PPU Control Register 2
            flag_image_mask: 0,
            flag_sprite_mask: 0,
            flag_screen_enable: 0,
            flag_sprites_enable: 0,
            flag_background_color: 0,

            // $2002 - PPU Status Register
            flag_overflow: 0,
            flag_hit: 0,
            flag_vblank: 0,

            // $2003 - OAMADDR
            oamaddr: 0,

            // $2005
            scroll_offset: 0,

            // $2006
            memory_address_lo: 0,
            memory_address_hi: 0,
        }
    }

    pub fn step(&mut self) {

    }

    pub fn read_control_1(&self) -> u8{
        0
    }

    pub fn read_control_2(&self) -> u8{
        0
    }

    pub fn read_status(&self) -> u8{
        0x80
    }

    pub fn read_oamaddr(&self) -> u8{
        0
    }

    pub fn read_unknown4(&self) -> u8{
        0
    }

    pub fn read_scroll_offset(&self) -> u8{
        0
    }

    pub fn read_addr_offset(&self) -> u8{
        0
    }

    pub fn read_unknown7(&self) -> u8{
        0
    }
}
