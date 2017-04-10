// The possible addressing modes of an instruction
// TODO: Put important details in comments above each
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
    page_delay: u32,

    // Number of cycles used by the instruction without any penalties
    cycles: u32,

    // Addressing mode used in memory lookup
    addr_mode: AddressingMode,

    // Actual size of the instruction besides the 1 byte opcode
    size: u32
}

// All possible instructions and their properties. This makes decoding as simple as an array lookup
pub const intructions: [Instruction; 256] = [
    Instruction{str_name: "BRK", page_delay: 4, cycles: 0, size: 7, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ORA", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "KIL", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SLO", page_delay: 4, cycles: 0, size: 8, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ORA", page_delay: 4, cycles: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ASL", page_delay: 4, cycles: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "SLO", page_delay: 4, cycles: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "PHP", page_delay: 4, cycles: 0, size: 3, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ORA", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "ASL", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Accumulator},
    Instruction{str_name: "ANC", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "ORA", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "ASL", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "SLO", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BPL", page_delay: 4, cycles: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "ORA", page_delay: 4, cycles: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SLO", page_delay: 4, cycles: 0, size: 8, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ORA", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ASL", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SLO", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "CLC", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ORA", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SLO", page_delay: 4, cycles: 0, size: 7, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ORA", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ASL", page_delay: 4, cycles: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "SLO", page_delay: 4, cycles: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "JSR", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "AND", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "KIL", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RLA", page_delay: 4, cycles: 0, size: 8, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "BIT", page_delay: 4, cycles: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "AND", page_delay: 4, cycles: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ROL", page_delay: 4, cycles: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "RLA", page_delay: 4, cycles: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "PLP", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "AND", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "ROL", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Accumulator},
    Instruction{str_name: "ANC", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "BIT", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "AND", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "ROL", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "RLA", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BMI", page_delay: 4, cycles: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "AND", page_delay: 4, cycles: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RLA", page_delay: 4, cycles: 0, size: 8, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "AND", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ROL", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "RLA", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SEC", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "AND", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RLA", page_delay: 4, cycles: 0, size: 7, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "AND", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ROL", page_delay: 4, cycles: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "RLA", page_delay: 4, cycles: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "RTI", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "EOR", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "KIL", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SRE", page_delay: 4, cycles: 0, size: 8, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "EOR", page_delay: 4, cycles: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "LSR", page_delay: 4, cycles: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "SRE", page_delay: 4, cycles: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "PHA", page_delay: 4, cycles: 0, size: 3, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "EOR", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "LSR", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Accumulator},
    Instruction{str_name: "ALR", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "JMP", page_delay: 4, cycles: 0, size: 3, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "EOR", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "LSR", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "SRE", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BVC", page_delay: 4, cycles: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "EOR", page_delay: 4, cycles: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SRE", page_delay: 4, cycles: 0, size: 8, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "EOR", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "LSR", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SRE", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "CLI", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "EOR", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SRE", page_delay: 4, cycles: 0, size: 7, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "EOR", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "LSR", page_delay: 4, cycles: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "SRE", page_delay: 4, cycles: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "RTS", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ADC", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "KIL", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RRA", page_delay: 4, cycles: 0, size: 8, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ADC", page_delay: 4, cycles: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ROR", page_delay: 4, cycles: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "RRA", page_delay: 4, cycles: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "PLA", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ADC", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "ROR", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Accumulator},
    Instruction{str_name: "ARR", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "JMP", page_delay: 4, cycles: 0, size: 5, addr_mode: AddressingMode::Indirect},
    Instruction{str_name: "ADC", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "ROR", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "RRA", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BVS", page_delay: 4, cycles: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "ADC", page_delay: 4, cycles: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RRA", page_delay: 4, cycles: 0, size: 8, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ADC", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ROR", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "RRA", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SEI", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ADC", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "RRA", page_delay: 4, cycles: 0, size: 7, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ADC", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ROR", page_delay: 4, cycles: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "RRA", page_delay: 4, cycles: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "STA", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "SAX", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "STY", page_delay: 4, cycles: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "STA", page_delay: 4, cycles: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "STX", page_delay: 4, cycles: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "SAX", page_delay: 4, cycles: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "DEY", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "TXA", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "XAA", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "STY", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "STA", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "STX", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "SAX", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BCC", page_delay: 4, cycles: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "STA", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "AHX", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "STY", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "STA", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "STX", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::ZeroPageY},
    Instruction{str_name: "SAX", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::ZeroPageY},
    Instruction{str_name: "TYA", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "STA", page_delay: 4, cycles: 0, size: 5, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "TXS", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "TAS", page_delay: 4, cycles: 0, size: 5, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "SHY", page_delay: 4, cycles: 0, size: 5, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "STA", page_delay: 4, cycles: 0, size: 5, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "SHX", page_delay: 4, cycles: 0, size: 5, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "AHX", page_delay: 4, cycles: 0, size: 5, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "LDY", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "LDA", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "LDX", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "LAX", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "LDY", page_delay: 4, cycles: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "LDA", page_delay: 4, cycles: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "LDX", page_delay: 4, cycles: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "LAX", page_delay: 4, cycles: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "TAY", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "LDA", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "TAX", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "LAX", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "LDY", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "LDA", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "LDX", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "LAX", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BCS", page_delay: 4, cycles: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "LDA", page_delay: 4, cycles: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "LAX", page_delay: 4, cycles: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "LDY", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "LDA", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "LDX", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::ZeroPageY},
    Instruction{str_name: "LAX", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::ZeroPageY},
    Instruction{str_name: "CLV", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "LDA", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "TSX", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "LAS", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "LDY", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "LDA", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "LDX", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "LAX", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "CPY", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "CMP", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "DCP", page_delay: 4, cycles: 0, size: 8, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "CPY", page_delay: 4, cycles: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "CMP", page_delay: 4, cycles: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "DEC", page_delay: 4, cycles: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "DCP", page_delay: 4, cycles: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "INY", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "CMP", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "DEX", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "AXS", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "CPY", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "CMP", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "DEC", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "DCP", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BNE", page_delay: 4, cycles: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "CMP", page_delay: 4, cycles: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "DCP", page_delay: 4, cycles: 0, size: 8, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "CMP", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "DEC", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "DCP", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "CLD", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "CMP", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "DCP", page_delay: 4, cycles: 0, size: 7, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "CMP", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "DEC", page_delay: 4, cycles: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "DCP", page_delay: 4, cycles: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "CPX", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "SBC", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "ISC", page_delay: 4, cycles: 0, size: 8, addr_mode: AddressingMode::IndexedIndirect},
    Instruction{str_name: "CPX", page_delay: 4, cycles: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "SBC", page_delay: 4, cycles: 0, size: 3, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "INC", page_delay: 4, cycles: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "ISC", page_delay: 4, cycles: 0, size: 5, addr_mode: AddressingMode::ZeroPage},
    Instruction{str_name: "INX", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SBC", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SBC", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Immediate},
    Instruction{str_name: "CPX", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "SBC", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "INC", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "ISC", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::Absolute},
    Instruction{str_name: "BEQ", page_delay: 4, cycles: 1, size: 2, addr_mode: AddressingMode::Relative},
    Instruction{str_name: "SBC", page_delay: 4, cycles: 1, size: 5, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "KIL", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ISC", page_delay: 4, cycles: 0, size: 8, addr_mode: AddressingMode::IndirectIndexed},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SBC", page_delay: 4, cycles: 0, size: 4, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "INC", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "ISC", page_delay: 4, cycles: 0, size: 6, addr_mode: AddressingMode::ZeroPageX},
    Instruction{str_name: "SED", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "SBC", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 0, size: 2, addr_mode: AddressingMode::Implicit},
    Instruction{str_name: "ISC", page_delay: 4, cycles: 0, size: 7, addr_mode: AddressingMode::AbsoluteY},
    Instruction{str_name: "NOP", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "SBC", page_delay: 4, cycles: 1, size: 4, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "INC", page_delay: 4, cycles: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
    Instruction{str_name: "ISC", page_delay: 4, cycles: 0, size: 7, addr_mode: AddressingMode::AbsoluteX},
];