pub struct CPU {
    pub memory: Vec<u16>,
    cycles: u64,
    pc: u16, 
    sp: u8,   
    a: u8,   
    x: u8,   
    y: u8,   
    c: u8,   
    z: u8,   
    i: u8,   
    d: u8,   
    b: u8,   
    u: u8,   
    v: u8,   
    n: u8,   
    interrupt: u8,
    stall: u32
}

enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
}

struct Instruction {
    str_name: String,
    page_delay: u32,
    cycles: u32,
    addr_mode: AddressingMode,
    size: u32
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: vec![0;0x1000],
            cycles: 0,
            pc: 0, 
            sp: 0,   
            a: 0,   
            x: 0,   
            y: 0,   
            c: 0,   
            z: 0,   
            i: 0,   
            d: 0,   
            b: 0,   
            u: 0,   
            v: 0,   
            n: 0,   
            interrupt: 0,
            stall: 0
        }
    }

    pub fn step() {

    }

    pub fn step_num(cycles: u32) {

    }
}
