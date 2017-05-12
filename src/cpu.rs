#![allow(unused_variables)]
#![allow(dead_code)]

pub struct CPU {
    pub memory: ::memory::CPUMemoryMap,
    // Number of cycles executed so far
    cycles: u64,

    // Cycles for this step
    this_cycles: u8,

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
    b: u8,  // This doesn't do anything
    u: u8,  // Unused
    v: u8,  // Overflow
    n: u8,  // Negative

    interrupt: u8,
    // stall: u32
}

pub enum Interrupt {
    IntNMI,
    IntIRQ,
    IntNone,
}


fn pages_differ(one: u16, two: u16) -> bool {
    one & 0xFF00 != two & 0xFF00
}

impl CPU {
    pub fn new(mem: ::memory::CPUMemoryMap) -> CPU {
        let pcval = {
            ((mem.read(0xFFFD) as u16) << 8) + (mem.read(0xFFFC) as u16)
        };

        CPU {
            memory: mem,
            cycles: 0,
            this_cycles: 0,
            pc: pcval,
            sp: 0xfd,
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
            //stall: 0
        }
    }

    fn push(&mut self, data: u8) {
        self.memory.write(data, self.sp as u16 + 0x100);
        self.sp -= 1;
    }

    fn push16(&mut self, data: u16) {
        self.push(((data >> 8) & 0xff) as u8);
        self.push((data & 0xff) as u8);
    }

    fn pop(&mut self) -> u8 {
        self.sp += 1;
        self.memory.read(self.sp as u16 + 0x100)
    }

    fn pop16(&mut self) -> u16 {
        let lo = self.pop();
        let hi = self.pop();

        ((hi as u16) << 8) | lo as u16
    }

    fn flags_str(&self) -> String {
        // NBICZ
        let mut status = "".to_string();
        if (self.n == 1) {
            status += "n";
        }
        if (self.b == 1) {
            status += "b";
        }
        if (self.i == 1) {
            status += "i";
        }
        if (self.c == 1) {
            status += "c";
        }
        if (self.z == 1) {
            status += "z";
        }

        status
    }


    pub fn step(&mut self, int: Interrupt) -> u8 {
        self.this_cycles = 0;

        match int {
            Interrupt::IntNMI => self.nmi(),
            Interrupt::IntIRQ => self.irq(),
            Interrupt::IntNone => {},
        }

        let (fun, address, addr_mode, size, str_name) = {
            let opcode = self.memory.read(self.pc);
            let instruction = &INSTRUCTIONS[opcode as usize];
            let mut page_crossed = false;
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
                        (self.pc + 2).wrapping_add((offset as i16) as u16)
                    } else {
                        (self.pc + 2).wrapping_add(offset as u16)
                    }
                },


                AddressingMode::Absolute => self.memory.read16(self.pc+1),

                AddressingMode::AbsoluteX => {
                    let addr = self.memory.read16(self.pc+1) + self.x as u16;
                    page_crossed = pages_differ(addr - self.x as u16, addr);
                    addr
                },

                AddressingMode::AbsoluteY => {
                    let addr = self.memory.read16(self.pc+1) + self.y as u16;
                    page_crossed = pages_differ(addr - self.y as u16, addr);
                    addr
                },

                AddressingMode::Indirect => {
                    self.memory.read16(self.memory.read16(self.pc+1))
                },

                AddressingMode::IndexedIndirect => {
                    self.memory.read16(self.memory.read(self.pc+1) as u16 + self.x as u16)
                },

                AddressingMode::IndirectIndexed => {
                    let addr = self.memory.read16(self.memory.read(self.pc+1) as u16) + self.y as u16;

                    page_crossed = pages_differ(addr - self.y as u16, addr);
                    addr
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

            self.this_cycles = instruction.cycles + if page_crossed { instruction.page_delay } else { 0 };
            (fun, address, instruction.addr_mode, instruction.size, instruction.str_name)
        };

        //println!("{:x}\t{:x}\t{}\tA: {:x}\tX: {:x}\tY: {:x}\tP: {}\tSP: {:x}", self.pc, address, str_name, self.a, self.x, self.y, self.flags_str(), self.sp);
        fun(self, address, addr_mode);
        self.pc += size as u16;

        self.this_cycles
    }

    fn nmi(&mut self) {
        let pc = self.pc;

        self.push16(pc);
        self.php(0, AddressingMode::Implicit);
        self.pc = self.memory.read16(0xFFFA);
        self.i = 1;
        self.this_cycles += 7;

        //println!("NMI Occured");
    }

    fn flags(&self) -> u8 {
        let mut flags:u8 = 0;;
        flags |= self.c << 0;
        flags |= self.z << 1;
        flags |= self.i << 2;
        flags |= self.d << 3;
        flags |= self.b << 4;
        flags |= self.u << 5;
        flags |= self.v << 6;
        flags |= self.n << 7;

        flags
    }

    fn set_flags(&mut self, flags:u8) {
        self.c = (flags & 1 << 0) >> 0;
        self.z = (flags & 1 << 1) >> 1;
        self.i = (flags & 1 << 2) >> 2;
        self.d = (flags & 1 << 3) >> 3;
        self.b = (flags & 1 << 4) >> 4;
        self.u = (flags & 1 << 5) >> 5;
        self.v = (flags & 1 << 6) >> 6;
        self.n = (flags & 1 << 7) >> 7;
    }

    fn irq(&mut self) {

    }

    fn adc(&mut self, address: u16, mode: AddressingMode) {
        let mem = self.memory.read(address);
        let (res1, overflow1) = mem.overflowing_add(self.a);
        let (res2, overflow2) = res1.overflowing_add(self.c);

        self.z = if res2 == 0 { 1 } else { 0 };
        self.n = if res2 & 0b10000000 != 0 { 1 } else { 0 };
        self.c = if overflow1 || overflow2 { 1 } else { 0 };
        self.v = if (self.a ^ res2) & 0b10000000 != 0 && (mem ^ self.a) & 0b10000000 == 0 { 1 } else { 0 };

        self.a = res2;
    }

    fn ahx(&mut self, address: u16, mode: AddressingMode){}
    fn alr(&mut self, address: u16, mode: AddressingMode){}
    fn anc(&mut self, address: u16, mode: AddressingMode){}

    fn and(&mut self, address: u16, mode: AddressingMode) {
        self.a = self.a & self.memory.read(address);
        self.z = if self.a == 0 { 1 } else { 0 };
        self.n = if self.a & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn arr(&mut self, address: u16, mode: AddressingMode){}

    fn asl(&mut self, address: u16, mode: AddressingMode) {
        match mode {
            AddressingMode::Accumulator => {
                let res = self.a << 1;
                let overflow = self.a & 0b10000000 != 0;

                self.a = res;

                self.z = if self.a == 0 { 1 } else { 0 };
                self.n = if self.a & 0b10000000 != 0 { 1 } else { 0 };
                self.c = if overflow { 1 } else { 0 };
            }
            _ => {
                let mem = self.memory.read(address);
                let res = mem << 1;
                let overflow = mem & 0b10000000 != 0;

                self.z = if res == 0 { 1 } else { 0 };
                self.n = if res & 0b10000000 != 0 { 1 } else { 0 };
                self.c = if overflow { 1 } else { 0 };

                self.memory.write(res as u8, address);
            }
        }
    }

    fn axs(&mut self, address: u16, mode: AddressingMode){}

    /* All the branch instruction */
    /* FIXME: address subtraction by 2 is not safe and should be fixed at some point although it
     * probably won't cause issues because who would ever be executing in ram */
    fn bcc(&mut self, address: u16, mode: AddressingMode) {
        if self.c == 0 {
            self.pc = address - 2;
            self.this_cycles += if pages_differ(self.pc, address) { 1 } else { 0 };
        }
    }

    fn bcs(&mut self, address: u16, mode: AddressingMode) {
        if self.c == 1 {
            self.pc = address - 2;
            self.this_cycles += if pages_differ(self.pc, address) { 1 } else { 0 };
        }
    }

    fn beq(&mut self, address: u16, mode: AddressingMode) {
        if self.z == 1{
            self.pc = address - 2;
            self.this_cycles += if pages_differ(self.pc, address) { 1 } else { 0 };
        }
    }

    fn bit(&mut self, address: u16, mode: AddressingMode) {
        panic!("Not implemented!");
    }

    fn bmi(&mut self, address: u16, mode: AddressingMode) {
        if self.n == 1{
            self.pc = address - 2;
            self.this_cycles += if pages_differ(self.pc, address) { 1 } else { 0 };
        }
    }

    fn bne(&mut self, address: u16, mode: AddressingMode) {
        if self.z == 0 {
            self.pc = address - 2;
            self.this_cycles += if pages_differ(self.pc, address) { 1 } else { 0 };
        }
    }

    fn bpl(&mut self, address: u16, mode: AddressingMode) {
        if self.n == 0 {
            self.pc = address - 2;
            self.this_cycles += if pages_differ(self.pc, address) { 1 } else { 0 };
        }
    }

    fn brk(&mut self, address: u16, mode: AddressingMode) {
        panic!("Not implemented!");
    }

    fn bvc(&mut self, address: u16, mode: AddressingMode) {
        if self.v == 0 {
            self.pc = address - 2;
            self.this_cycles += if pages_differ(self.pc, address) { 1 } else { 0 };
        }
    }

    fn bvs(&mut self, address: u16, mode: AddressingMode) {
        if self.v == 1 {
            self.pc = address - 2;
            self.this_cycles += if pages_differ(self.pc, address) { 1 } else { 0 };
        }
    }

    fn clc(&mut self, address: u16, mode: AddressingMode) {
        self.c = 0;
    }

    fn cld(&mut self, address: u16, mode: AddressingMode) {
        self.d = 0;
    }

    fn cli(&mut self, address: u16, mode: AddressingMode) {
        self.i = 0;
    }

    fn clv(&mut self, address: u16, mode: AddressingMode) {
        self.v = 0;
    }

    fn cmp(&mut self, address: u16, mode: AddressingMode) {
        let mem = self.memory.read(address);

        self.c = if self.a >= mem { 1 } else { 0 };
        self.z = if self.a == mem { 1 } else { 0 };
        self.v = if self.a < mem { 1 } else { 0 };
    }

    fn cpx(&mut self, address: u16, mode: AddressingMode) {
        let mem = self.memory.read(address);

        self.c = if self.x >= mem { 1 } else { 0 };
        self.z = if self.x == mem { 1 } else { 0 };
        self.v = if self.x < mem { 1 } else { 0 };
    }

    fn cpy(&mut self, address: u16, mode: AddressingMode) {
        let mem = self.memory.read(address);

        self.c = if self.y >= mem { 1 } else { 0 };
        self.z = if self.y == mem { 1 } else { 0 };
        self.v = if self.y < mem { 1 } else { 0 };
    }

    fn dcp(&mut self, address: u16, mode: AddressingMode) {
        panic!("Not implemented!");
    }

    fn dec(&mut self, address: u16, mode: AddressingMode) {
        let mem = self.memory.read(address).wrapping_sub(1);

        self.z = if mem == 0 { 1 } else { 0 };
        self.n = if mem & 0b10000000 != 0 { 1 } else { 0 };

        self.memory.write(mem, address);
    }

    fn dex(&mut self, address: u16, mode: AddressingMode) {
        self.x = self.x.wrapping_sub(1);
        self.z = if self.x == 0 { 1 } else { 0 };
        self.n = if self.x & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn dey(&mut self, address: u16, mode: AddressingMode){
        self.y = self.y.wrapping_sub(1);
        self.z = if self.y == 0 { 1 } else { 0 };
        self.n = if self.y & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn eor(&mut self, address: u16, mode: AddressingMode) {
        let mem = self.memory.read(address);

        self.a = self.a ^ mem;

        self.z = if self.y == 0 { 1 } else { 0 };
        self.n = if self.y & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn inc(&mut self, address: u16, mode: AddressingMode) {
        let mem = self.memory.read(address).wrapping_add(1);

        self.z = if mem == 0 { 1 } else { 0 };
        self.n = if mem & 0b10000000 != 0 { 1 } else { 0 };

        self.memory.write(mem, address);
    }

    fn inx(&mut self, address: u16, mode: AddressingMode) {
        self.x = self.x.wrapping_add(1);
        self.z = if self.x == 0 { 1 } else { 0 };
        self.n = if self.x & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn iny(&mut self, address: u16, mode: AddressingMode) {
        self.y = self.y.wrapping_add(1);
        self.z = if self.y == 0 { 1 } else { 0 };
        self.n = if self.y & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn isc(&mut self, address: u16, mode: AddressingMode) {
        panic!("Not implemented!");
    }

    fn jmp(&mut self, address: u16, mode: AddressingMode) {
        self.pc = address - 3;
    }
    fn jsr(&mut self, address: u16, mode: AddressingMode) {
        let pcval = self.pc + 3 - 1;

        self.push16(pcval);

        self.pc = address - 3;
    }

    fn kil(&mut self, address: u16, mode: AddressingMode) {
        panic!("Not implemented!");
    }
    fn las(&mut self, address: u16, mode: AddressingMode) {
        panic!("Not implemented!");
    }
    fn lax(&mut self, address: u16, mode: AddressingMode) {
        panic!("Not implemented!");
    }

    fn lda(&mut self, address: u16, mode: AddressingMode) {
        self.a = self.memory.read(address);

        self.z = if self.a == 0 { 1 } else { 0 };
        self.n = if self.a & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn ldx(&mut self, address: u16, mode: AddressingMode) {
        self.x = self.memory.read(address);

        self.z = if self.x == 0 { 1 } else { 0 };
        self.n = if self.x & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn ldy(&mut self, address: u16, mode: AddressingMode) {
        self.y = self.memory.read(address);

        self.z = if self.y == 0 { 1 } else { 0 };
        self.n = if self.y & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn lsr(&mut self, address: u16, mode: AddressingMode) {
        match mode {
            AddressingMode::Accumulator => {
                let res = self.a >> 1;
                let overflow = self.a & 1 != 0;

                self.a = res;

                self.c = if overflow { 1 } else { 0 };
                self.z = if self.a == 0 { 1 } else { 0 };
                self.n = if self.a & 0b10000000 != 0 { 1 } else { 0 };
            }

            _ => {
                let mem = self.memory.read(address);
                let res = mem >> 1;
                let overflow = mem & 1 != 0;

                self.c = if overflow { 1 } else { 0 };
                self.z = if mem == 0 { 1 } else { 0 };
                self.n = if mem & 0b10000000 != 0 { 1 } else { 0 };

                self.memory.write(res, address);
            }
        }
    }

    fn nop(&mut self, address: u16, mode: AddressingMode) {}

    fn ora(&mut self, address: u16, mode: AddressingMode) {
        self.a = self.a | self.memory.read(address);
        self.z = if self.a == 0 { 1 } else { 0 };
        self.n = if self.a & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn pha(&mut self, address: u16, mode: AddressingMode) {
        let a = self.a;
        self.push(a);
    }

    fn php(&mut self, address: u16, mode: AddressingMode) {
        let flags = self.flags();
        self.push(flags);
    }

    fn pla(&mut self, address: u16, mode: AddressingMode) {
        self.a = self.pop();

        self.z = if self.a == 0 { 1 } else { 0 };
        self.n = if self.a & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn plp(&mut self, address: u16, mode: AddressingMode) {
        panic!("Not implemented!");
    }


    fn rla(&mut self, address: u16, mode: AddressingMode) {
        panic!("Not implemented!");
    }

    fn rol(&mut self, address: u16, mode: AddressingMode) {
        match mode {
            AddressingMode::Accumulator => {
                let (mut newval, overflow) = self.a.overflowing_shl(1);
                newval |= self.c;
                self.a = newval;
                self.c = if overflow { 1 } else { 0 };
                self.z = if self.a == 0 { 1 } else { 0 };
                self.n = if self.a & 0b10000000 != 0 { 1 } else { 0 };
            }

            _ => {
                let (mut newval, overflow) = {
                    self.memory.read(address).overflowing_shl(1)
                };

                newval |= self.c;
                self.memory.write(newval, address);
                self.c = if overflow { 1 } else { 0 };
                self.z = if self.a == 0 { 1 } else { 0 };
                self.n = if self.a & 0b10000000 != 0 { 1 } else { 0 };
            }
        }
    }

    fn ror(&mut self, address: u16, mode: AddressingMode) {
        match mode {
            AddressingMode::Accumulator => {
                let (mut newval, overflow) = self.a.overflowing_shr(1);
                newval |= self.c << 7;
                self.a = newval;
                self.c = if overflow { 1 } else { 0 };
                self.z = if self.a == 0 { 1 } else { 0 };
                self.n = if self.a & 0b10000000 != 0 { 1 } else { 0 };
            }

            _ => {
                let (mut newval, overflow) = {
                    self.memory.read(address).overflowing_shr(1)
                };

                newval |= self.c << 7;
                self.memory.write(newval, address);
                self.c = if overflow { 1 } else { 0 };
                self.z = if self.a == 0 { 1 } else { 0 };
                self.n = if self.a & 0b10000000 != 0 { 1 } else { 0 };
            }
        }
    }

    fn rra(&mut self, address: u16, mode: AddressingMode) {
        panic!("Not implemented!");
    }

    fn rti(&mut self, address: u16, mode: AddressingMode) {
        let flags = self.pop();
        let pc = self.pop16();

        self.set_flags(flags);
        //Fixme SUCH A HACK OMG PLS
        self.pc = pc-1;
    }

    fn rts(&mut self, address: u16, mode: AddressingMode) {
        self.pc = self.pop16();
    }

    fn sax(&mut self, address: u16, mode: AddressingMode) {
        panic!("Not implemented!");
    }

    fn sbc(&mut self, address: u16, mode: AddressingMode) {
        let olda = self.a;
        let mem = self.memory.read(address);
        self.a = self.a.wrapping_sub(mem).wrapping_sub(1 - self.c);
        let res16:u16 = (olda as u16).wrapping_sub(mem as u16).wrapping_sub((1 - self.c) as u16);

        self.z = if self.a == 0 { 1 } else { 0 };
        self.n = if self.a & 0b10000000 != 0 { 1 } else { 0 };
        self.c = if res16 > 0xFF { 1 } else { 0 };
        self.v = if (olda ^ self.a) & 0b10000000 != 0 && (mem ^ olda) & 0b10000000 == 0 { 1 } else { 0 };
    }

    fn sec(&mut self, address: u16, mode: AddressingMode) {
        self.c = 1;
    }

    fn sed(&mut self, address: u16, mode: AddressingMode) {
        self.d = 1;
    }

    fn sei(&mut self, address: u16, mode: AddressingMode) {
        self.i = 1;
    }

    fn shx(&mut self, address: u16, mode: AddressingMode) {
        panic!("Not implemented!");
    }
    fn shy(&mut self, address: u16, mode: AddressingMode) {
        panic!("Not implemented!");
    }
    fn slo(&mut self, address: u16, mode: AddressingMode) {
        panic!("Not implemented!");
    }
    fn sre(&mut self, address: u16, mode: AddressingMode) {
        panic!("Not implemented!");
    }

    fn sta(&mut self, address: u16, mode: AddressingMode) {
        self.memory.write(self.a, address);
    }

    fn stx(&mut self, address: u16, mode: AddressingMode) {
        self.memory.write(self.x, address);
    }

    fn sty(&mut self, address: u16, mode: AddressingMode) {
        self.memory.write(self.y, address);
    }

    fn tas(&mut self, address: u16, mode: AddressingMode) {
        panic!("Not implemented!");
    }

    fn tax(&mut self, address: u16, mode: AddressingMode) {
        self.x = self.a;
        self.z = if self.x == 0 { 1 } else { 0 };
        self.n = if self.x & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn tay(&mut self, address: u16, mode: AddressingMode) {
        self.y = self.a;
        self.z = if self.y == 0 { 1 } else { 0 };
        self.n = if self.y & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn tsx(&mut self, address: u16, mode: AddressingMode) {
        self.x = self.sp;
    }

    fn txa(&mut self, address: u16, mode: AddressingMode) {
        self.a = self.x;
        self.z = if self.a == 0 { 1 } else { 0 };
        self.n = if self.a & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn txs(&mut self, address: u16, mode: AddressingMode) {
        self.sp = self.x;
    }

    fn tya(&mut self, address: u16, mode: AddressingMode) {
        self.a = self.y;
        self.z = if self.a == 0 { 1 } else { 0 };
        self.n = if self.a & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn xaa(&mut self, address: u16, mode: AddressingMode) {
        panic!("Not implemented!");
    }
}

// The possible addressing modes of an instruction
// TODO: Put important details in comments above each
#[derive(Debug, Clone, Copy)]
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
pub struct Instruction<'a> {
    // The name of the instruction for informational purposes
    str_name: &'a str,

    // The cycle delay encountered when crossing page boundary
    page_delay: u8,

    // Number of cycles used by the instruction without any penalties
    cycles: u8,

    // Addressing mode used in memory lookup
    addr_mode: AddressingMode,

    // Actual size of the instruction besides the 1 byte opcode
    size: u32,
}

// All possible instructions and their properties. This makes decoding as simple as an array lookup
const INSTRUCTIONS: [Instruction; 256] = [
    Instruction{str_name: "BRK", cycles: 7, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ORA", cycles: 6, page_delay: 0, size: 2, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "KIL", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SLO", cycles: 8, page_delay: 0, size: 0, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", cycles: 3, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ORA", cycles: 3, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ASL", cycles: 5, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "SLO", cycles: 5, page_delay: 0, size: 0, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "PHP", cycles: 3, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ORA", cycles: 2, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "ASL", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Accumulator},
    Instruction{str_name: "ANC", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "ORA", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "ASL", cycles: 6, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "SLO", cycles: 6, page_delay: 0, size: 0, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BPL", cycles: 2, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "ORA", cycles: 5, page_delay: 1, size: 2, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SLO", cycles: 8, page_delay: 0, size: 0, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ORA", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ASL", cycles: 6, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SLO", cycles: 6, page_delay: 0, size: 0, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "CLC", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ORA", cycles: 4, page_delay: 1, size: 3, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SLO", cycles: 7, page_delay: 0, size: 0, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 1, size: 3, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ORA", cycles: 4, page_delay: 1, size: 3, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ASL", cycles: 7, page_delay: 0, size: 3, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "SLO", cycles: 7, page_delay: 0, size: 0, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "JSR", cycles: 6, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "AND", cycles: 6, page_delay: 0, size: 2, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "KIL", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RLA", cycles: 8, page_delay: 0, size: 0, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "BIT", cycles: 3, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "AND", cycles: 3, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ROL", cycles: 5, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "RLA", cycles: 5, page_delay: 0, size: 0, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "PLP", cycles: 4, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "AND", cycles: 2, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "ROL", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Accumulator},
    Instruction{str_name: "ANC", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "BIT", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "AND", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "ROL", cycles: 6, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "RLA", cycles: 6, page_delay: 0, size: 0, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BMI", cycles: 2, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "AND", cycles: 5, page_delay: 1, size: 2, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RLA", cycles: 8, page_delay: 0, size: 0, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "AND", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ROL", cycles: 6, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "RLA", cycles: 6, page_delay: 0, size: 0, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SEC", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "AND", cycles: 4, page_delay: 1, size: 3, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RLA", cycles: 7, page_delay: 0, size: 0, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 1, size: 3, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "AND", cycles: 4, page_delay: 1, size: 3, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ROL", cycles: 7, page_delay: 0, size: 3, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "RLA", cycles: 7, page_delay: 0, size: 0, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "RTI", cycles: 6, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "EOR", cycles: 6, page_delay: 0, size: 2, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "KIL", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SRE", cycles: 8, page_delay: 0, size: 0, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", cycles: 3, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "EOR", cycles: 3, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "LSR", cycles: 5, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "SRE", cycles: 5, page_delay: 0, size: 0, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "PHA", cycles: 3, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "EOR", cycles: 2, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "LSR", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Accumulator},
    Instruction{str_name: "ALR", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "JMP", cycles: 3, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "EOR", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "LSR", cycles: 6, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "SRE", cycles: 6, page_delay: 0, size: 0, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BVC", cycles: 2, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "EOR", cycles: 5, page_delay: 1, size: 2, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SRE", cycles: 8, page_delay: 0, size: 0, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "EOR", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "LSR", cycles: 6, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SRE", cycles: 6, page_delay: 0, size: 0, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "CLI", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "EOR", cycles: 4, page_delay: 1, size: 3, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SRE", cycles: 7, page_delay: 0, size: 0, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 1, size: 3, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "EOR", cycles: 4, page_delay: 1, size: 3, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "LSR", cycles: 7, page_delay: 0, size: 3, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "SRE", cycles: 7, page_delay: 0, size: 0, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "RTS", cycles: 6, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ADC", cycles: 6, page_delay: 0, size: 2, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "KIL", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RRA", cycles: 8, page_delay: 0, size: 0, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", cycles: 3, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ADC", cycles: 3, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ROR", cycles: 5, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "RRA", cycles: 5, page_delay: 0, size: 0, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "PLA", cycles: 4, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ADC", cycles: 2, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "ROR", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Accumulator},
    Instruction{str_name: "ARR", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "JMP", cycles: 5, page_delay: 0, size: 3, addr_mode: AddressingMode::Indirect},
    Instruction{str_name: "ADC", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "ROR", cycles: 6, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "RRA", cycles: 6, page_delay: 0, size: 0, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BVS", cycles: 2, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "ADC", cycles: 5, page_delay: 1, size: 2, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RRA", cycles: 8, page_delay: 0, size: 0, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ADC", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ROR", cycles: 6, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "RRA", cycles: 6, page_delay: 0, size: 0, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SEI", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ADC", cycles: 4, page_delay: 1, size: 3, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RRA", cycles: 7, page_delay: 0, size: 0, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 1, size: 3, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ADC", cycles: 4, page_delay: 1, size: 3, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ROR", cycles: 7, page_delay: 0, size: 3, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "RRA", cycles: 7, page_delay: 0, size: 0, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "NOP", cycles: 2, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "STA", cycles: 6, page_delay: 0, size: 2, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "SAX", cycles: 6, page_delay: 0, size: 0, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "STY", cycles: 3, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "STA", cycles: 3, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "STX", cycles: 3, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "SAX", cycles: 3, page_delay: 0, size: 0, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "DEY", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "NOP", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "TXA", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "XAA", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "STY", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "STA", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "STX", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "SAX", cycles: 4, page_delay: 0, size: 0, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BCC", cycles: 2, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "STA", cycles: 6, page_delay: 0, size: 2, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "AHX", cycles: 6, page_delay: 0, size: 0, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "STY", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "STA", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "STX", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageY},
    Instruction{str_name: "SAX", cycles: 4, page_delay: 0, size: 0, addr_mode: AddressingMode::ZeroPageY},
    Instruction{str_name: "TYA", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "STA", cycles: 5, page_delay: 0, size: 3, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "TXS", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "TAS", cycles: 5, page_delay: 0, size: 0, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "SHY", cycles: 5, page_delay: 0, size: 0, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "STA", cycles: 5, page_delay: 0, size: 3, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "SHX", cycles: 5, page_delay: 0, size: 0, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "AHX", cycles: 5, page_delay: 0, size: 0, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "LDY", cycles: 2, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "LDA", cycles: 6, page_delay: 0, size: 2, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "LDX", cycles: 2, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "LAX", cycles: 6, page_delay: 0, size: 0, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "LDY", cycles: 3, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "LDA", cycles: 3, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "LDX", cycles: 3, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "LAX", cycles: 3, page_delay: 0, size: 0, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "TAY", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "LDA", cycles: 2, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "TAX", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "LAX", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "LDY", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "LDA", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "LDX", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "LAX", cycles: 4, page_delay: 0, size: 0, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BCS", cycles: 2, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "LDA", cycles: 5, page_delay: 1, size: 2, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "LAX", cycles: 5, page_delay: 1, size: 0, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "LDY", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "LDA", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "LDX", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageY},
    Instruction{str_name: "LAX", cycles: 4, page_delay: 0, size: 0, addr_mode: AddressingMode::ZeroPageY},
    Instruction{str_name: "CLV", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "LDA", cycles: 4, page_delay: 1, size: 3, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "TSX", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "LAS", cycles: 4, page_delay: 1, size: 0, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "LDY", cycles: 4, page_delay: 1, size: 3, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "LDA", cycles: 4, page_delay: 1, size: 3, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "LDX", cycles: 4, page_delay: 1, size: 3, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "LAX", cycles: 4, page_delay: 1, size: 0, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "CPY", cycles: 2, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "CMP", cycles: 6, page_delay: 0, size: 2, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "DCP", cycles: 8, page_delay: 0, size: 0, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "CPY", cycles: 3, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "CMP", cycles: 3, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "DEC", cycles: 5, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "DCP", cycles: 5, page_delay: 0, size: 0, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "INY", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "CMP", cycles: 2, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "DEX", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "AXS", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "CPY", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "CMP", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "DEC", cycles: 6, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "DCP", cycles: 6, page_delay: 0, size: 0, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BNE", cycles: 2, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "CMP", cycles: 5, page_delay: 1, size: 2, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "DCP", cycles: 8, page_delay: 0, size: 0, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "CMP", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "DEC", cycles: 6, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "DCP", cycles: 6, page_delay: 0, size: 0, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "CLD", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "CMP", cycles: 4, page_delay: 1, size: 3, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "DCP", cycles: 7, page_delay: 0, size: 0, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 1, size: 3, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "CMP", cycles: 4, page_delay: 1, size: 3, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "DEC", cycles: 7, page_delay: 0, size: 3, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "DCP", cycles: 7, page_delay: 0, size: 0, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "CPX", cycles: 2, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "SBC", cycles: 6, page_delay: 0, size: 2, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "ISC", cycles: 8, page_delay: 0, size: 0, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "CPX", cycles: 3, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "SBC", cycles: 3, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "INC", cycles: 5, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ISC", cycles: 5, page_delay: 0, size: 0, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "INX", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SBC", cycles: 2, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "NOP", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SBC", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "CPX", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "SBC", cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "INC", cycles: 6, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "ISC", cycles: 6, page_delay: 0, size: 0, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BEQ", cycles: 2, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "SBC", cycles: 5, page_delay: 1, size: 2, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", cycles: 2, page_delay: 0, size: 0, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ISC", cycles: 8, page_delay: 0, size: 0, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SBC", cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "INC", cycles: 6, page_delay: 0, size: 2, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ISC", cycles: 6, page_delay: 0, size: 0, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SED", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SBC", cycles: 4, page_delay: 1, size: 3, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 2, page_delay: 0, size: 1, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ISC", cycles: 7, page_delay: 0, size: 0, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", cycles: 4, page_delay: 1, size: 3, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "SBC", cycles: 4, page_delay: 1, size: 3, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "INC", cycles: 7, page_delay: 0, size: 3, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ISC", cycles: 7, page_delay: 0, size: 0, addr_mode: AddressingMode::AbsoluteX},
    ];
