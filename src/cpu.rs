pub struct CPU<'a> {
    pub memory: &'a ::memory::Memory<'a>,
    // Number of cycles executed so far
    cycles: u64,

    // Program Counter
    pc: u16, 

    // Stack Pointer
    sp: u8,   

    // Accumulator
    a: u8,   

    // General Purpose Registers
    x: u8,   
    y: u8,   

    /*** Flags ***/
    c: u8,  // Carry
    z: u8,  // Zero
    i: u8,  // Interrupt Mask
    d: u8,  // Decimal (NEVER USED)
    v: u8,  // Overflow
    n: u8,  // Negative

    interrupt: u8,
    stall: u32
}

impl<'a> CPU<'a> {
    pub fn new(mem: &'a ::memory::Memory<'a>) -> CPU<'a> {
        CPU {
            memory: mem,
            cycles: 0,
            pc: ((mem.read(0xFFFD) as u16) << 8) + (mem.read(0xFFFC) as u16),
            sp: 0,   
            a: 0,   
            x: 0,   
            y: 0,   
            c: 0,   
            z: 0,   
            i: 0,   
            d: 0,   
            v: 0,   
            n: 0,   
            interrupt: 0,
            stall: 0
        }
    }

    pub fn step(&mut self) {
        let opcode = self.memory.read(self.pc);
        let instruction = &INSTRUCTIONS[opcode as usize];
        let address:u16 = match instruction.addr_mode {
            AddressingMode::Implicit => 0,
            AddressingMode::Accumulator => 0,
            AddressingMode::Immediate => self.pc + 1,
            AddressingMode::ZeroPage => self.memory.read(self.pc + 1) as u16,
            AddressingMode::ZeroPageX => (self.memory.read(self.pc + 1) + self.x) as u16,
            AddressingMode::ZeroPageY => (self.memory.read(self.pc + 1) + self.y) as u16,
            AddressingMode::Relative => {
                let offset:i8 = self.memory.read(self.pc + 1) as i8;
                if offset < 0 {
                    self.pc + 2 + (-offset as u16)
                } else {
                    self.pc + 2 + (offset as u16)
                }
            },

            AddressingMode::Absolute => ((self.memory.read(self.pc+1) as u16) << 8) + (self.memory.read(self.pc+2) as u16),

            AddressingMode::AbsoluteX => ((self.memory.read(self.pc+1) as u16) << 8) + (self.memory.read(self.pc+2) as u16) + self.x as u16,

            AddressingMode::AbsoluteY => ((self.memory.read(self.pc+1) as u16) << 8) + (self.memory.read(self.pc+2) as u16) + self.y as u16,

            AddressingMode::Indirect => {
                let addr:u16 = ((self.memory.read(self.pc+1) as u16) << 8) + (self.memory.read(self.pc+2) as u16);
                ((self.memory.read(addr+self.pc+1) as u16) << 8) + (self.memory.read(addr+self.pc+2) as u16)
            },

            AddressingMode::IndexedIndirect => {
                let addr:u16 = ((self.memory.read(self.pc+1) as u16) << 8) + (self.memory.read(self.pc+2) as u16);
                ((self.memory.read(addr+self.pc+1+self.x as u16) as u16) << 8) + (self.memory.read(addr+self.pc+2+self.x as u16) as u16)
            },

            AddressingMode::IndirectIndexed => {
                let addr:u16 = ((self.memory.read(self.pc+1) as u16) << 8) + (self.memory.read(self.pc+2) as u16) + self.x as u16;
                ((self.memory.read(addr+self.pc+1) as u16) << 8) + (self.memory.read(addr+self.pc+2) as u16)
            },
        };

        let fun = match opcode {
            0x00 => CPU::brk,
            0x01 => CPU::ora,
            0x02 => CPU::kil,
            0x03 => CPU::slo,
            0x04 => CPU::nop,
            0x05 => CPU::ora,
            0x06 => CPU::asl,
            0x07 => CPU::slo,
            0x08 => CPU::php,
            0x09 => CPU::ora,
            0x0a => CPU::asl,
            0x0b => CPU::anc,
            0x0c => CPU::nop,
            0x0d => CPU::ora,
            0x0e => CPU::asl,
            0x0f => CPU::slo,
            0x10 => CPU::bpl,
            0x11 => CPU::ora,
            0x12 => CPU::kil,
            0x13 => CPU::slo,
            0x14 => CPU::nop,
            0x15 => CPU::ora,
            0x16 => CPU::asl,
            0x17 => CPU::slo,
            0x18 => CPU::clc,
            0x19 => CPU::ora,
            0x1a => CPU::nop,
            0x1b => CPU::slo,
            0x1c => CPU::nop,
            0x1d => CPU::ora,
            0x1e => CPU::asl,
            0x1f => CPU::slo,
            0x20 => CPU::jsr,
            0x21 => CPU::and,
            0x22 => CPU::kil,
            0x23 => CPU::rla,
            0x24 => CPU::bit,
            0x25 => CPU::and,
            0x26 => CPU::rol,
            0x27 => CPU::rla,
            0x28 => CPU::plp,
            0x29 => CPU::and,
            0x2a => CPU::rol,
            0x2b => CPU::anc,
            0x2c => CPU::bit,
            0x2d => CPU::and,
            0x2e => CPU::rol,
            0x2f => CPU::rla,
            0x30 => CPU::bmi,
            0x31 => CPU::and,
            0x32 => CPU::kil,
            0x33 => CPU::rla,
            0x34 => CPU::nop,
            0x35 => CPU::and,
            0x36 => CPU::rol,
            0x37 => CPU::rla,
            0x38 => CPU::sec,
            0x39 => CPU::and,
            0x3a => CPU::nop,
            0x3b => CPU::rla,
            0x3c => CPU::nop,
            0x3d => CPU::and,
            0x3e => CPU::rol,
            0x3f => CPU::rla,
            0x40 => CPU::rti,
            0x41 => CPU::eor,
            0x42 => CPU::kil,
            0x43 => CPU::sre,
            0x44 => CPU::nop,
            0x45 => CPU::eor,
            0x46 => CPU::lsr,
            0x47 => CPU::sre,
            0x48 => CPU::pha,
            0x49 => CPU::eor,
            0x4a => CPU::lsr,
            0x4b => CPU::alr,
            0x4c => CPU::jmp,
            0x4d => CPU::eor,
            0x4e => CPU::lsr,
            0x4f => CPU::sre,
            0x50 => CPU::bvc,
            0x51 => CPU::eor,
            0x52 => CPU::kil,
            0x53 => CPU::sre,
            0x54 => CPU::nop,
            0x55 => CPU::eor,
            0x56 => CPU::lsr,
            0x57 => CPU::sre,
            0x58 => CPU::cli,
            0x59 => CPU::eor,
            0x5a => CPU::nop,
            0x5b => CPU::sre,
            0x5c => CPU::nop,
            0x5d => CPU::eor,
            0x5e => CPU::lsr,
            0x5f => CPU::sre,
            0x60 => CPU::rts,
            0x61 => CPU::adc,
            0x62 => CPU::kil,
            0x63 => CPU::rra,
            0x64 => CPU::nop,
            0x65 => CPU::adc,
            0x66 => CPU::ror,
            0x67 => CPU::rra,
            0x68 => CPU::pla,
            0x69 => CPU::adc,
            0x6a => CPU::ror,
            0x6b => CPU::arr,
            0x6c => CPU::jmp,
            0x6d => CPU::adc,
            0x6e => CPU::ror,
            0x6f => CPU::rra,
            0x70 => CPU::bvs,
            0x71 => CPU::adc,
            0x72 => CPU::kil,
            0x73 => CPU::rra,
            0x74 => CPU::nop,
            0x75 => CPU::adc,
            0x76 => CPU::ror,
            0x77 => CPU::rra,
            0x78 => CPU::sei,
            0x79 => CPU::adc,
            0x7a => CPU::nop,
            0x7b => CPU::rra,
            0x7c => CPU::nop,
            0x7d => CPU::adc,
            0x7e => CPU::ror,
            0x7f => CPU::rra,
            0x80 => CPU::nop,
            0x81 => CPU::sta,
            0x82 => CPU::nop,
            0x83 => CPU::sax,
            0x84 => CPU::sty,
            0x85 => CPU::sta,
            0x86 => CPU::stx,
            0x87 => CPU::sax,
            0x88 => CPU::dey,
            0x89 => CPU::nop,
            0x8a => CPU::txa,
            0x8b => CPU::xaa,
            0x8c => CPU::sty,
            0x8d => CPU::sta,
            0x8e => CPU::stx,
            0x8f => CPU::sax,
            0x90 => CPU::bcc,
            0x91 => CPU::sta,
            0x92 => CPU::kil,
            0x93 => CPU::ahx,
            0x94 => CPU::sty,
            0x95 => CPU::sta,
            0x96 => CPU::stx,
            0x97 => CPU::sax,
            0x98 => CPU::tya,
            0x99 => CPU::sta,
            0x9a => CPU::txs,
            0x9b => CPU::tas,
            0x9c => CPU::shy,
            0x9d => CPU::sta,
            0x9e => CPU::shx,
            0x9f => CPU::ahx,
            0xa0 => CPU::ldy,
            0xa1 => CPU::lda,
            0xa2 => CPU::ldx,
            0xa3 => CPU::lax,
            0xa4 => CPU::ldy,
            0xa5 => CPU::lda,
            0xa6 => CPU::ldx,
            0xa7 => CPU::lax,
            0xa8 => CPU::tay,
            0xa9 => CPU::lda,
            0xaa => CPU::tax,
            0xab => CPU::lax,
            0xac => CPU::ldy,
            0xad => CPU::lda,
            0xae => CPU::ldx,
            0xaf => CPU::lax,
            0xb0 => CPU::bcs,
            0xb1 => CPU::lda,
            0xb2 => CPU::kil,
            0xb3 => CPU::lax,
            0xb4 => CPU::ldy,
            0xb5 => CPU::lda,
            0xb6 => CPU::ldx,
            0xb7 => CPU::lax,
            0xb8 => CPU::clv,
            0xb9 => CPU::lda,
            0xba => CPU::tsx,
            0xbb => CPU::las,
            0xbc => CPU::ldy,
            0xbd => CPU::lda,
            0xbe => CPU::ldx,
            0xbf => CPU::lax,
            0xc0 => CPU::cpy,
            0xc1 => CPU::cmp,
            0xc2 => CPU::nop,
            0xc3 => CPU::dcp,
            0xc4 => CPU::cpy,
            0xc5 => CPU::cmp,
            0xc6 => CPU::dec,
            0xc7 => CPU::dcp,
            0xc8 => CPU::iny,
            0xc9 => CPU::cmp,
            0xca => CPU::dex,
            0xcb => CPU::axs,
            0xcc => CPU::cpy,
            0xcd => CPU::cmp,
            0xce => CPU::dec,
            0xcf => CPU::dcp,
            0xd0 => CPU::bne,
            0xd1 => CPU::cmp,
            0xd2 => CPU::kil,
            0xd3 => CPU::dcp,
            0xd4 => CPU::nop,
            0xd5 => CPU::cmp,
            0xd6 => CPU::dec,
            0xd7 => CPU::dcp,
            0xd8 => CPU::cld,
            0xd9 => CPU::cmp,
            0xda => CPU::nop,
            0xdb => CPU::dcp,
            0xdc => CPU::nop,
            0xdd => CPU::cmp,
            0xde => CPU::dec,
            0xdf => CPU::dcp,
            0xe0 => CPU::cpx,
            0xe1 => CPU::sbc,
            0xe2 => CPU::nop,
            0xe3 => CPU::isc,
            0xe4 => CPU::cpx,
            0xe5 => CPU::sbc,
            0xe6 => CPU::inc,
            0xe7 => CPU::isc,
            0xe8 => CPU::inx,
            0xe9 => CPU::sbc,
            0xea => CPU::nop,
            0xeb => CPU::sbc,
            0xec => CPU::cpx,
            0xed => CPU::sbc,
            0xee => CPU::inc,
            0xef => CPU::isc,
            0xf0 => CPU::beq,
            0xf1 => CPU::sbc,
            0xf2 => CPU::kil,
            0xf3 => CPU::isc,
            0xf4 => CPU::nop,
            0xf5 => CPU::sbc,
            0xf6 => CPU::inc,
            0xf7 => CPU::isc,
            0xf8 => CPU::sed,
            0xf9 => CPU::sbc,
            0xfa => CPU::nop,
            0xfb => CPU::isc,
            0xfc => CPU::nop,
            0xfd => CPU::sbc,
            0xfe => CPU::inc,
            0xff => CPU::isc, 
            _ => panic!("Byte holding larger than 0xff"),
        };

        fun(self, address);
        println!("{}: {}", self.pc, instruction.str_name);
        self.pc += instruction.size as u16;
    }

    fn adc(&mut self, address: u16){} 
    fn ahx(&mut self, address: u16){}
    fn alr(&mut self, address: u16){}
    fn anc(&mut self, address: u16){}
    fn and(&mut self, address: u16){}
    fn arr(&mut self, address: u16){}
    fn asl(&mut self, address: u16){}
    fn axs(&mut self, address: u16){}
    fn bcc(&mut self, address: u16){}
    fn bcs(&mut self, address: u16){}
    fn beq(&mut self, address: u16){}
    fn bit(&mut self, address: u16){}
    fn bmi(&mut self, address: u16){}
    fn bne(&mut self, address: u16){}
    fn bpl(&mut self, address: u16){}
    fn brk(&mut self, address: u16){}
    fn bvc(&mut self, address: u16){}
    fn bvs(&mut self, address: u16){}

    fn clc(&mut self, address: u16) {
        self.c = 0;
    }

    fn cld(&mut self, address: u16) {
        self.d = 0;
    }

    fn cli(&mut self, address: u16) {
        self.i = 0;
    }

    fn clv(&mut self, address: u16) {
        self.v = 0;
    }

    fn cmp(&mut self, address: u16){}
    fn cpx(&mut self, address: u16){}
    fn cpy(&mut self, address: u16){}
    fn dcp(&mut self, address: u16){}
    fn dec(&mut self, address: u16){}

    fn dex(&mut self, address: u16) {
        self.x -= 1;
        self.z = if self.x == 0 { 1 } else { 0 };
        self.n = if self.x & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn dey(&mut self, address: u16){
        self.y -= 1;
        self.z = if self.y == 0 { 1 } else { 0 };
        self.n = if self.y & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn eor(&mut self, address: u16){}
    fn inc(&mut self, address: u16){}

    fn inx(&mut self, address: u16) {
        self.x += 1;
        self.z = if self.x == 0 { 1 } else { 0 };
        self.n = if self.x & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn iny(&mut self, address: u16) {
        self.y += 1;
        self.z = if self.y == 0 { 1 } else { 0 };
        self.n = if self.y & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn isc(&mut self, address: u16){}
    fn jmp(&mut self, address: u16){}
    fn jsr(&mut self, address: u16){}
    fn kil(&mut self, address: u16){}
    fn las(&mut self, address: u16){}
    fn lax(&mut self, address: u16){}
    fn lda(&mut self, address: u16){}
    fn ldx(&mut self, address: u16){}
    fn ldy(&mut self, address: u16){}
    fn lsr(&mut self, address: u16){}
    fn nop(&mut self, address: u16){}
    fn ora(&mut self, address: u16){}
    fn pha(&mut self, address: u16){}
    fn php(&mut self, address: u16){}
    fn pla(&mut self, address: u16){}
    fn plp(&mut self, address: u16){}
    fn rla(&mut self, address: u16){}
    fn rol(&mut self, address: u16){}
    fn ror(&mut self, address: u16){}
    fn rra(&mut self, address: u16){}
    fn rti(&mut self, address: u16){}
    fn rts(&mut self, address: u16){}
    fn sax(&mut self, address: u16){}
    fn sbc(&mut self, address: u16){}

    fn sec(&mut self, address: u16) {
        self.c = 1;
    }

    fn sed(&mut self, address: u16) {
        self.d = 1;
    }

    fn sei(&mut self, address: u16) {
        self.i = 1;
    }

    fn shx(&mut self, address: u16){}
    fn shy(&mut self, address: u16){}
    fn slo(&mut self, address: u16){}
    fn sre(&mut self, address: u16){}
    fn sta(&mut self, address: u16){}
    fn stx(&mut self, address: u16){}
    fn sty(&mut self, address: u16){}
    fn tas(&mut self, address: u16){}

    fn tax(&mut self, address: u16) {
        self.x = self.a;
        self.z = if self.x == 0 { 1 } else { 0 };
        self.n = if self.x & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn tay(&mut self, address: u16) {
        self.y = self.a;
        self.z = if self.y == 0 { 1 } else { 0 };
        self.n = if self.y & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn tsx(&mut self, address: u16) {
        self.x = self.sp;
    }

    fn txa(&mut self, address: u16) {
        self.a = self.x;
        self.z = if self.a == 0 { 1 } else { 0 };
        self.n = if self.a & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn txs(&mut self, address: u16) {
        self.sp = self.x;
    }

    fn tya(&mut self, address: u16) {
        self.a = self.y;
        self.z = if self.a == 0 { 1 } else { 0 };
        self.n = if self.a & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn xaa(&mut self, address: u16){}
}

// The possible addressing modes of an instruction
// TODO: Put important details in comments above each
#[derive(Debug)]
pub enum AddressingMode {
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

// Format of instructions
pub struct Instruction<'a, 'b> {
    // The name of the instruction for informational purposes
    str_name: &'a str,

    // The cycle delay encountered when crossing page boundary
    page_delay: u32,

    // Number of cycles used by the instruction without any penalties
    cycles: u32,

    // Addressing mode used in memory lookup
    addr_mode: AddressingMode,

    // Actual size of the instruction besides the 1 byte opcode
    size: u32,
}

impl<'a, 'b> Instruction<'a, 'b> {
    fn print(&self) {
        println!("{}", self.str_name);
    }
}

// All possible instructions and their properties. This makes decoding as simple as an array lookup
const INSTRUCTIONS: [Instruction; 256] = [
    Instruction{str_name: "BRK", cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ORA", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "KIL", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SLO", cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ORA", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ASL", cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "SLO", cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "PHP", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ORA", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "ASL", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Accumulator},
    Instruction{str_name: "ANC", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "ORA", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "ASL", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "SLO", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BPL", cycles: 4, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "ORA", cycles: 4, page_delay: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SLO", cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ORA", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ASL", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SLO", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "CLC", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ORA", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SLO", cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ORA", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ASL", cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "SLO", cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "JSR", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "AND", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "KIL", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RLA", cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "BIT", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "AND", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ROL", cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "RLA", cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "PLP", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "AND", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "ROL", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Accumulator},
    Instruction{str_name: "ANC", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "BIT", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "AND", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "ROL", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "RLA", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BMI", cycles: 4, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "AND", cycles: 4, page_delay: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RLA", cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "AND", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ROL", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "RLA", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SEC", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "AND", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RLA", cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "AND", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ROL", cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "RLA", cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "RTI", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "EOR", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "KIL", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SRE", cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "EOR", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "LSR", cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "SRE", cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "PHA", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "EOR", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "LSR", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Accumulator},
    Instruction{str_name: "ALR", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "JMP", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "EOR", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "LSR", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "SRE", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BVC", cycles: 4, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "EOR", cycles: 4, page_delay: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SRE", cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "EOR", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "LSR", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SRE", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "CLI", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "EOR", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SRE", cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "EOR", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "LSR", cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "SRE", cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "RTS", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ADC", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "KIL", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RRA", cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ADC", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ROR", cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "RRA", cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "PLA", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ADC", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "ROR", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Accumulator},
    Instruction{str_name: "ARR", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "JMP", cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::Indirect},
    Instruction{str_name: "ADC", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "ROR", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "RRA", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BVS", cycles: 4, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "ADC", cycles: 4, page_delay: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RRA", cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ADC", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ROR", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "RRA", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SEI", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ADC", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RRA", cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ADC", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ROR", cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "RRA", cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "STA", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "SAX", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "STY", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "STA", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "STX", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "SAX", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "DEY", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "TXA", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "XAA", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "STY", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "STA", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "STX", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "SAX", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BCC", cycles: 4, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "STA", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "AHX", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "STY", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "STA", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "STX", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageY},
    Instruction{str_name: "SAX", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageY},
    Instruction{str_name: "TYA", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "STA", cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "TXS", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "TAS", cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "SHY", cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "STA", cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "SHX", cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "AHX", cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "LDY", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "LDA", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "LDX", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "LAX", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "LDY", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "LDA", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "LDX", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "LAX", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "TAY", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "LDA", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "TAX", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "LAX", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "LDY", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "LDA", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "LDX", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "LAX", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BCS", cycles: 4, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "LDA", cycles: 4, page_delay: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "LAX", cycles: 4, page_delay: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "LDY", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "LDA", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "LDX", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageY},
    Instruction{str_name: "LAX", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageY},
    Instruction{str_name: "CLV", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "LDA", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "TSX", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "LAS", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "LDY", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "LDA", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "LDX", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "LAX", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "CPY", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "CMP", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "DCP", cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "CPY", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "CMP", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "DEC", cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "DCP", cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "INY", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "CMP", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "DEX", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "AXS", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "CPY", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "CMP", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "DEC", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "DCP", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BNE", cycles: 4, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "CMP", cycles: 4, page_delay: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "DCP", cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "CMP", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "DEC", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "DCP", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "CLD", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "CMP", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "DCP", cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "CMP", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "DEC", cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "DCP", cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "CPX", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "SBC", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "ISC", cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "CPX", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "SBC", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "INC", cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ISC", cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "INX", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SBC", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SBC", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "CPX", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "SBC", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "INC", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "ISC", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BEQ", cycles: 4, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "SBC", cycles: 4, page_delay: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ISC", cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SBC", cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "INC", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ISC", cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SED", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SBC", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ISC", cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "SBC", cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "INC", cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ISC", cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    ];
