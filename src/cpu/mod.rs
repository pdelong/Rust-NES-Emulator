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
            b: 0,   
            u: 0,   
            v: 0,   
            n: 0,   
            interrupt: 0,
            stall: 0
        }
    }

    pub fn step(&mut self) {
        let opcode = self.memory.read(self.pc);
        println!("{:?}", instructions[opcode as usize]);
    }

    pub fn step_num(cycles: u32) {

    }

    fn adc(address: u16, pc: u16, mode: AddressingMode){} 
    fn ahx(address: u16, pc: u16, mode: AddressingMode){}
    fn alr(address: u16, pc: u16, mode: AddressingMode){}
    fn anc(address: u16, pc: u16, mode: AddressingMode){}
    fn and(address: u16, pc: u16, mode: AddressingMode){}
    fn arr(address: u16, pc: u16, mode: AddressingMode){}
    fn asl(address: u16, pc: u16, mode: AddressingMode){}
    fn axs(address: u16, pc: u16, mode: AddressingMode){}
    fn bcc(address: u16, pc: u16, mode: AddressingMode){}
    fn bcs(address: u16, pc: u16, mode: AddressingMode){}
    fn beq(address: u16, pc: u16, mode: AddressingMode){}
    fn bit(address: u16, pc: u16, mode: AddressingMode){}
    fn bmi(address: u16, pc: u16, mode: AddressingMode){}
    fn bne(address: u16, pc: u16, mode: AddressingMode){}
    fn bpl(address: u16, pc: u16, mode: AddressingMode){}
    fn brk(address: u16, pc: u16, mode: AddressingMode){}
    fn bvc(address: u16, pc: u16, mode: AddressingMode){}
    fn bvs(address: u16, pc: u16, mode: AddressingMode){}
    fn clc(address: u16, pc: u16, mode: AddressingMode){}
    fn cld(address: u16, pc: u16, mode: AddressingMode){}
    fn cli(address: u16, pc: u16, mode: AddressingMode){}
    fn clv(address: u16, pc: u16, mode: AddressingMode){}
    fn cmp(address: u16, pc: u16, mode: AddressingMode){}
    fn cpx(address: u16, pc: u16, mode: AddressingMode){}
    fn cpy(address: u16, pc: u16, mode: AddressingMode){}
    fn dcp(address: u16, pc: u16, mode: AddressingMode){}
    fn dec(address: u16, pc: u16, mode: AddressingMode){}
    fn dex(address: u16, pc: u16, mode: AddressingMode){}
    fn dey(address: u16, pc: u16, mode: AddressingMode){}
    fn eor(address: u16, pc: u16, mode: AddressingMode){}
    fn inc(address: u16, pc: u16, mode: AddressingMode){}
    fn inx(address: u16, pc: u16, mode: AddressingMode){}
    fn iny(address: u16, pc: u16, mode: AddressingMode){}
    fn isc(address: u16, pc: u16, mode: AddressingMode){}
    fn jmp(address: u16, pc: u16, mode: AddressingMode){}
    fn jsr(address: u16, pc: u16, mode: AddressingMode){}
    fn kil(address: u16, pc: u16, mode: AddressingMode){}
    fn las(address: u16, pc: u16, mode: AddressingMode){}
    fn lax(address: u16, pc: u16, mode: AddressingMode){}
    fn lda(address: u16, pc: u16, mode: AddressingMode){}
    fn ldx(address: u16, pc: u16, mode: AddressingMode){}
    fn ldy(address: u16, pc: u16, mode: AddressingMode){}
    fn lsr(address: u16, pc: u16, mode: AddressingMode){}
    fn nop(address: u16, pc: u16, mode: AddressingMode){}
    fn ora(address: u16, pc: u16, mode: AddressingMode){}
    fn pha(address: u16, pc: u16, mode: AddressingMode){}
    fn php(address: u16, pc: u16, mode: AddressingMode){}
    fn pla(address: u16, pc: u16, mode: AddressingMode){}
    fn plp(address: u16, pc: u16, mode: AddressingMode){}
    fn rla(address: u16, pc: u16, mode: AddressingMode){}
    fn rol(address: u16, pc: u16, mode: AddressingMode){}
    fn ror(address: u16, pc: u16, mode: AddressingMode){}
    fn rra(address: u16, pc: u16, mode: AddressingMode){}
    fn rti(address: u16, pc: u16, mode: AddressingMode){}
    fn rts(address: u16, pc: u16, mode: AddressingMode){}
    fn sax(address: u16, pc: u16, mode: AddressingMode){}
    fn sbc(address: u16, pc: u16, mode: AddressingMode){}
    fn sec(address: u16, pc: u16, mode: AddressingMode){}
    fn sed(address: u16, pc: u16, mode: AddressingMode){}
    fn sei(address: u16, pc: u16, mode: AddressingMode){}
    fn shx(address: u16, pc: u16, mode: AddressingMode){}
    fn shy(address: u16, pc: u16, mode: AddressingMode){}
    fn slo(address: u16, pc: u16, mode: AddressingMode){}
    fn sre(address: u16, pc: u16, mode: AddressingMode){}
    fn sta(address: u16, pc: u16, mode: AddressingMode){}
    fn stx(address: u16, pc: u16, mode: AddressingMode){}
    fn sty(address: u16, pc: u16, mode: AddressingMode){}
    fn tas(address: u16, pc: u16, mode: AddressingMode){}
    fn tax(address: u16, pc: u16, mode: AddressingMode){}
    fn tay(address: u16, pc: u16, mode: AddressingMode){}
    fn tsx(address: u16, pc: u16, mode: AddressingMode){}
    fn txa(address: u16, pc: u16, mode: AddressingMode){}
    fn txs(address: u16, pc: u16, mode: AddressingMode){}
    fn tya(address: u16, pc: u16, mode: AddressingMode){}
    fn xaa(address: u16, pc: u16, mode: AddressingMode){}
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
#[derive(Debug)]
pub struct Instruction<'a> {
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
    fun: fn(u16, u16, AddressingMode)
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
