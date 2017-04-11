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

impl<'a, 'b> CPU<'a> {
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
        let instruction = &instructions[opcode as usize];
        println!("{}: {}", self.pc, instruction.str_name);
        self.pc += instruction.size as u16;
    }

    pub fn step_num(cycles: u32) {

    }

    fn adc(&mut self, address: u16, pc: u16, mode: AddressingMode){} 
    fn ahx(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn alr(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn anc(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn and(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn arr(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn asl(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn axs(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn bcc(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn bcs(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn beq(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn bit(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn bmi(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn bne(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn bpl(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn brk(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn bvc(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn bvs(&mut self, address: u16, pc: u16, mode: AddressingMode){}

    fn clc(&mut self, address: u16, pc: u16, mode: AddressingMode) {
        self.c = 0;
    }

    fn cld(&mut self, address: u16, pc: u16, mode: AddressingMode) {
        self.d = 0;
    }

    fn cli(&mut self, address: u16, pc: u16, mode: AddressingMode) {
        self.i = 0;
    }

    fn clv(&mut self, address: u16, pc: u16, mode: AddressingMode) {
        self.v = 0;
    }

    fn cmp(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn cpx(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn cpy(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn dcp(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn dec(&mut self, address: u16, pc: u16, mode: AddressingMode){}

    fn dex(&mut self, address: u16, pc: u16, mode: AddressingMode) {
        self.x -= 1;
        self.z = if self.x == 0 { 1 } else { 0 };
        self.n = if self.x & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn dey(&mut self, address: u16, pc: u16, mode: AddressingMode){
        self.y -= 1;
        self.z = if self.y == 0 { 1 } else { 0 };
        self.n = if self.y & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn eor(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn inc(&mut self, address: u16, pc: u16, mode: AddressingMode){}

    fn inx(&mut self, address: u16, pc: u16, mode: AddressingMode) {
        self.x += 1;
        self.z = if self.x == 0 { 1 } else { 0 };
        self.n = if self.x & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn iny(&mut self, address: u16, pc: u16, mode: AddressingMode) {
        self.y += 1;
        self.z = if self.y == 0 { 1 } else { 0 };
        self.n = if self.y & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn isc(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn jmp(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn jsr(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn kil(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn las(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn lax(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn lda(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn ldx(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn ldy(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn lsr(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn nop(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn ora(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn pha(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn php(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn pla(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn plp(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn rla(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn rol(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn ror(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn rra(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn rti(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn rts(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn sax(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn sbc(&mut self, address: u16, pc: u16, mode: AddressingMode){}

    fn sec(&mut self, address: u16, pc: u16, mode: AddressingMode) {
        self.c = 1;
    }

    fn sed(&mut self, address: u16, pc: u16, mode: AddressingMode) {
        self.d = 1;
    }

    fn sei(&mut self, address: u16, pc: u16, mode: AddressingMode) {
        self.i = 1;
    }

    fn shx(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn shy(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn slo(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn sre(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn sta(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn stx(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn sty(&mut self, address: u16, pc: u16, mode: AddressingMode){}
    fn tas(&mut self, address: u16, pc: u16, mode: AddressingMode){}

    fn tax(&mut self, address: u16, pc: u16, mode: AddressingMode) {
        self.x = self.a;
        self.z = if self.x == 0 { 1 } else { 0 };
        self.n = if self.x & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn tay(&mut self, address: u16, pc: u16, mode: AddressingMode) {
        self.y = self.a;
        self.z = if self.y == 0 { 1 } else { 0 };
        self.n = if self.y & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn tsx(&mut self, address: u16, pc: u16, mode: AddressingMode) {
        self.x = self.sp;
    }

    fn txa(&mut self, address: u16, pc: u16, mode: AddressingMode) {
        self.a = self.x;
        self.z = if self.a == 0 { 1 } else { 0 };
        self.n = if self.a & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn txs(&mut self, address: u16, pc: u16, mode: AddressingMode) {
        self.sp = self.x;
    }

    fn tya(&mut self, address: u16, pc: u16, mode: AddressingMode) {
        self.a = self.y;
        self.z = if self.a == 0 { 1 } else { 0 };
        self.n = if self.a & 0b10000000 != 0 { 1 } else { 0 };
    }

    fn xaa(&mut self, address: u16, pc: u16, mode: AddressingMode){}
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

    // Pointer to function that implements this instruction
    fun: fn(&mut CPU<'b>, u16, u16, AddressingMode)
}

impl<'a, 'b> Instruction<'a, 'b> {
    fn print(&self) {
        println!("{}", self.str_name);
    }
}

// All possible instructions and their properties. This makes decoding as simple as an array lookup
const instructions: [Instruction; 256] = [
    Instruction{str_name: "BRK", fun: CPU::brk, cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ORA", fun: CPU::ora, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "KIL", fun: CPU::kil, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SLO", fun: CPU::slo, cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ORA", fun: CPU::ora, cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ASL", fun: CPU::asl, cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "SLO", fun: CPU::slo, cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "PHP", fun: CPU::php, cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ORA", fun: CPU::ora, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "ASL", fun: CPU::asl, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Accumulator},
    Instruction{str_name: "ANC", fun: CPU::anc, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "ORA", fun: CPU::ora, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "ASL", fun: CPU::asl, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "SLO", fun: CPU::slo, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BPL", fun: CPU::bpl, cycles: 4, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "ORA", fun: CPU::ora, cycles: 4, page_delay: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", fun: CPU::kil, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SLO", fun: CPU::slo, cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ORA", fun: CPU::ora, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ASL", fun: CPU::asl, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SLO", fun: CPU::slo, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "CLC", fun: CPU::clc, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ORA", fun: CPU::ora, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SLO", fun: CPU::slo, cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ORA", fun: CPU::ora, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ASL", fun: CPU::asl, cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "SLO", fun: CPU::slo, cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "JSR", fun: CPU::jsr, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "AND", fun: CPU::and, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "KIL", fun: CPU::kil, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RLA", fun: CPU::rla, cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "BIT", fun: CPU::bit, cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "AND", fun: CPU::and, cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ROL", fun: CPU::rol, cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "RLA", fun: CPU::rla, cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "PLP", fun: CPU::plp, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "AND", fun: CPU::and, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "ROL", fun: CPU::rol, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Accumulator},
    Instruction{str_name: "ANC", fun: CPU::anc, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "BIT", fun: CPU::bit, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "AND", fun: CPU::and, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "ROL", fun: CPU::rol, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "RLA", fun: CPU::rla, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BMI", fun: CPU::bmi, cycles: 4, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "AND", fun: CPU::and, cycles: 4, page_delay: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", fun: CPU::kil, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RLA", fun: CPU::rla, cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "AND", fun: CPU::and, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ROL", fun: CPU::rol, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "RLA", fun: CPU::rla, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SEC", fun: CPU::sec, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "AND", fun: CPU::and, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RLA", fun: CPU::rla, cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "AND", fun: CPU::and, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ROL", fun: CPU::rol, cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "RLA", fun: CPU::rla, cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "RTI", fun: CPU::rti, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "EOR", fun: CPU::eor, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "KIL", fun: CPU::kil, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SRE", fun: CPU::sre, cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "EOR", fun: CPU::eor, cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "LSR", fun: CPU::lsr, cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "SRE", fun: CPU::sre, cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "PHA", fun: CPU::pha, cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "EOR", fun: CPU::eor, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "LSR", fun: CPU::lsr, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Accumulator},
    Instruction{str_name: "ALR", fun: CPU::alr, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "JMP", fun: CPU::jmp, cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "EOR", fun: CPU::eor, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "LSR", fun: CPU::lsr, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "SRE", fun: CPU::sre, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BVC", fun: CPU::bvc, cycles: 4, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "EOR", fun: CPU::eor, cycles: 4, page_delay: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", fun: CPU::kil, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SRE", fun: CPU::sre, cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "EOR", fun: CPU::eor, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "LSR", fun: CPU::lsr, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SRE", fun: CPU::sre, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "CLI", fun: CPU::cli, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "EOR", fun: CPU::eor, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SRE", fun: CPU::sre, cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "EOR", fun: CPU::eor, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "LSR", fun: CPU::lsr, cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "SRE", fun: CPU::sre, cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "RTS", fun: CPU::rts, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ADC", fun: CPU::adc, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "KIL", fun: CPU::kil, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RRA", fun: CPU::rra, cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ADC", fun: CPU::adc, cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ROR", fun: CPU::ror, cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "RRA", fun: CPU::rra, cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "PLA", fun: CPU::pla, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ADC", fun: CPU::adc, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "ROR", fun: CPU::ror, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Accumulator},
    Instruction{str_name: "ARR", fun: CPU::arr, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "JMP", fun: CPU::jmp, cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::Indirect},
    Instruction{str_name: "ADC", fun: CPU::adc, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "ROR", fun: CPU::ror, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "RRA", fun: CPU::rra, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BVS", fun: CPU::bvs, cycles: 4, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "ADC", fun: CPU::adc, cycles: 4, page_delay: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", fun: CPU::kil, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RRA", fun: CPU::rra, cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ADC", fun: CPU::adc, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ROR", fun: CPU::ror, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "RRA", fun: CPU::rra, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SEI", fun: CPU::sei, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ADC", fun: CPU::adc, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RRA", fun: CPU::rra, cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ADC", fun: CPU::adc, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ROR", fun: CPU::ror, cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "RRA", fun: CPU::rra, cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "STA", fun: CPU::sta, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "SAX", fun: CPU::sax, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "STY", fun: CPU::sty, cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "STA", fun: CPU::sta, cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "STX", fun: CPU::stx, cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "SAX", fun: CPU::sax, cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "DEY", fun: CPU::dey, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "TXA", fun: CPU::txa, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "XAA", fun: CPU::xaa, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "STY", fun: CPU::sty, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "STA", fun: CPU::sta, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "STX", fun: CPU::stx, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "SAX", fun: CPU::sax, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BCC", fun: CPU::bcc, cycles: 4, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "STA", fun: CPU::sta, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", fun: CPU::kil, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "AHX", fun: CPU::ahx, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "STY", fun: CPU::sty, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "STA", fun: CPU::sta, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "STX", fun: CPU::stx, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageY},
    Instruction{str_name: "SAX", fun: CPU::sax, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageY},
    Instruction{str_name: "TYA", fun: CPU::tya, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "STA", fun: CPU::sta, cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "TXS", fun: CPU::txs, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "TAS", fun: CPU::tas, cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "SHY", fun: CPU::shy, cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "STA", fun: CPU::sta, cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "SHX", fun: CPU::shx, cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "AHX", fun: CPU::ahx, cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "LDY", fun: CPU::ldy, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "LDA", fun: CPU::lda, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "LDX", fun: CPU::ldx, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "LAX", fun: CPU::lax, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "LDY", fun: CPU::ldy, cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "LDA", fun: CPU::lda, cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "LDX", fun: CPU::ldx, cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "LAX", fun: CPU::lax, cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "TAY", fun: CPU::tay, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "LDA", fun: CPU::lda, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "TAX", fun: CPU::tax, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "LAX", fun: CPU::lax, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "LDY", fun: CPU::ldy, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "LDA", fun: CPU::lda, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "LDX", fun: CPU::ldx, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "LAX", fun: CPU::lax, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BCS", fun: CPU::bcs, cycles: 4, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "LDA", fun: CPU::lda, cycles: 4, page_delay: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", fun: CPU::kil, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "LAX", fun: CPU::lax, cycles: 4, page_delay: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "LDY", fun: CPU::ldy, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "LDA", fun: CPU::lda, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "LDX", fun: CPU::ldx, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageY},
    Instruction{str_name: "LAX", fun: CPU::lax, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageY},
    Instruction{str_name: "CLV", fun: CPU::clv, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "LDA", fun: CPU::lda, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "TSX", fun: CPU::tsx, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "LAS", fun: CPU::las, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "LDY", fun: CPU::ldy, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "LDA", fun: CPU::lda, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "LDX", fun: CPU::ldx, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "LAX", fun: CPU::lax, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "CPY", fun: CPU::cpy, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "CMP", fun: CPU::cmp, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "DCP", fun: CPU::dcp, cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "CPY", fun: CPU::cpy, cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "CMP", fun: CPU::cmp, cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "DEC", fun: CPU::dec, cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "DCP", fun: CPU::dcp, cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "INY", fun: CPU::iny, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "CMP", fun: CPU::cmp, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "DEX", fun: CPU::dex, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "AXS", fun: CPU::axs, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "CPY", fun: CPU::cpy, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "CMP", fun: CPU::cmp, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "DEC", fun: CPU::dec, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "DCP", fun: CPU::dcp, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BNE", fun: CPU::bne, cycles: 4, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "CMP", fun: CPU::cmp, cycles: 4, page_delay: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", fun: CPU::kil, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "DCP", fun: CPU::dcp, cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "CMP", fun: CPU::cmp, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "DEC", fun: CPU::dec, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "DCP", fun: CPU::dcp, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "CLD", fun: CPU::cld, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "CMP", fun: CPU::cmp, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "DCP", fun: CPU::dcp, cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "CMP", fun: CPU::cmp, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "DEC", fun: CPU::dec, cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "DCP", fun: CPU::dcp, cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "CPX", fun: CPU::cpx, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "SBC", fun: CPU::sbc, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "ISC", fun: CPU::isc, cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "CPX", fun: CPU::cpx, cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "SBC", fun: CPU::sbc, cycles: 4, page_delay: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "INC", fun: CPU::inc, cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ISC", fun: CPU::isc, cycles: 4, page_delay: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "INX", fun: CPU::inx, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SBC", fun: CPU::sbc, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SBC", fun: CPU::sbc, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "CPX", fun: CPU::cpx, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "SBC", fun: CPU::sbc, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "INC", fun: CPU::inc, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "ISC", fun: CPU::isc, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BEQ", fun: CPU::beq, cycles: 4, page_delay: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "SBC", fun: CPU::sbc, cycles: 4, page_delay: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", fun: CPU::kil, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ISC", fun: CPU::isc, cycles: 4, page_delay: 0, size: 8, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SBC", fun: CPU::sbc, cycles: 4, page_delay: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "INC", fun: CPU::inc, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ISC", fun: CPU::isc, cycles: 4, page_delay: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SED", fun: CPU::sed, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SBC", fun: CPU::sbc, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ISC", fun: CPU::isc, cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", fun: CPU::nop, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "SBC", fun: CPU::sbc, cycles: 4, page_delay: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "INC", fun: CPU::inc, cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ISC", fun: CPU::isc, cycles: 4, page_delay: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
];
