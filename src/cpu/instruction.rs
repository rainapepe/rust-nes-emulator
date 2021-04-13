use super::addres_mode::AddresMode;
use super::opcode::Opcode;
use super::Cpu6502;

#[derive(Copy, Clone)]
pub struct Instruct(&'static str, Opcode, AddresMode, u8);

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub addres_mode: AddresMode,
    pub cycles: u8,
    pub name: &'static str,
}

impl Instruct {
    fn get(&self) -> Instruction {
        Instruction {
            name: self.0,
            opcode: self.1,
            addres_mode: self.2,
            cycles: self.3,
        }
    }
}

// Tabela de instruções de 16x16 com 256 instruções
const LIST: [Instruct; 256] = [
    Instruct("BRK", Opcode::BRK, AddresMode::IMM, 7),
    Instruct("ORA", Opcode::ORA, AddresMode::IZX, 6),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 8),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 3),
    Instruct("ORA", Opcode::ORA, AddresMode::ZP0, 3),
    Instruct("ASL", Opcode::ASL, AddresMode::ZP0, 5),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 5),
    Instruct("PHP", Opcode::PHP, AddresMode::IMP, 3),
    Instruct("ORA", Opcode::ORA, AddresMode::IMM, 2),
    Instruct("ASL", Opcode::ASL, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 2),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 4),
    Instruct("ORA", Opcode::ORA, AddresMode::ABS, 4),
    Instruct("ASL", Opcode::ASL, AddresMode::ABS, 6),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 6),
    Instruct("BPL", Opcode::BPL, AddresMode::REL, 2),
    Instruct("ORA", Opcode::ORA, AddresMode::IZY, 5),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 8),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 4),
    Instruct("ORA", Opcode::ORA, AddresMode::ZPX, 4),
    Instruct("ASL", Opcode::ASL, AddresMode::ZPX, 6),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 6),
    Instruct("CLC", Opcode::CLC, AddresMode::IMP, 2),
    Instruct("ORA", Opcode::ORA, AddresMode::ABY, 4),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 7),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 4),
    Instruct("ORA", Opcode::ORA, AddresMode::ABX, 4),
    Instruct("ASL", Opcode::ASL, AddresMode::ABX, 7),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 7),
    Instruct("JSR", Opcode::JSR, AddresMode::ABS, 6),
    Instruct("AND", Opcode::AND, AddresMode::IZX, 6),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 8),
    Instruct("BIT", Opcode::BIT, AddresMode::ZP0, 3),
    Instruct("AND", Opcode::AND, AddresMode::ZP0, 3),
    Instruct("ROL", Opcode::ROL, AddresMode::ZP0, 5),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 5),
    Instruct("PLP", Opcode::PLP, AddresMode::IMP, 4),
    Instruct("AND", Opcode::AND, AddresMode::IMM, 2),
    Instruct("ROL", Opcode::ROL, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 2),
    Instruct("BIT", Opcode::BIT, AddresMode::ABS, 4),
    Instruct("AND", Opcode::AND, AddresMode::ABS, 4),
    Instruct("ROL", Opcode::ROL, AddresMode::ABS, 6),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 6),
    Instruct("BMI", Opcode::BMI, AddresMode::REL, 2),
    Instruct("AND", Opcode::AND, AddresMode::IZY, 5),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 8),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 4),
    Instruct("AND", Opcode::AND, AddresMode::ZPX, 4),
    Instruct("ROL", Opcode::ROL, AddresMode::ZPX, 6),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 6),
    Instruct("SEC", Opcode::SEC, AddresMode::IMP, 2),
    Instruct("AND", Opcode::AND, AddresMode::ABY, 4),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 7),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 4),
    Instruct("AND", Opcode::AND, AddresMode::ABX, 4),
    Instruct("ROL", Opcode::ROL, AddresMode::ABX, 7),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 7),
    Instruct("RTI", Opcode::RTI, AddresMode::IMP, 6),
    Instruct("EOR", Opcode::EOR, AddresMode::IZX, 6),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 8),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 3),
    Instruct("EOR", Opcode::EOR, AddresMode::ZP0, 3),
    Instruct("LSR", Opcode::LSR, AddresMode::ZP0, 5),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 5),
    Instruct("PHA", Opcode::PHA, AddresMode::IMP, 3),
    Instruct("EOR", Opcode::EOR, AddresMode::IMM, 2),
    Instruct("LSR", Opcode::LSR, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 2),
    Instruct("JMP", Opcode::JMP, AddresMode::ABS, 3),
    Instruct("EOR", Opcode::EOR, AddresMode::ABS, 4),
    Instruct("LSR", Opcode::LSR, AddresMode::ABS, 6),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 6),
    Instruct("BVC", Opcode::BVC, AddresMode::REL, 2),
    Instruct("EOR", Opcode::EOR, AddresMode::IZY, 5),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 8),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 4),
    Instruct("EOR", Opcode::EOR, AddresMode::ZPX, 4),
    Instruct("LSR", Opcode::LSR, AddresMode::ZPX, 6),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 6),
    Instruct("CLI", Opcode::CLI, AddresMode::IMP, 2),
    Instruct("EOR", Opcode::EOR, AddresMode::ABY, 4),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 7),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 4),
    Instruct("EOR", Opcode::EOR, AddresMode::ABX, 4),
    Instruct("LSR", Opcode::LSR, AddresMode::ABX, 7),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 7),
    Instruct("RTS", Opcode::RTS, AddresMode::IMP, 6),
    Instruct("ADC", Opcode::ADC, AddresMode::IZX, 6),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 8),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 3),
    Instruct("ADC", Opcode::ADC, AddresMode::ZP0, 3),
    Instruct("ROR", Opcode::ROR, AddresMode::ZP0, 5),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 5),
    Instruct("PLA", Opcode::PLA, AddresMode::IMP, 4),
    Instruct("ADC", Opcode::ADC, AddresMode::IMM, 2),
    Instruct("ROR", Opcode::ROR, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 2),
    Instruct("JMP", Opcode::JMP, AddresMode::IND, 5),
    Instruct("ADC", Opcode::ADC, AddresMode::ABS, 4),
    Instruct("ROR", Opcode::ROR, AddresMode::ABS, 6),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 6),
    Instruct("BVS", Opcode::BVS, AddresMode::REL, 2),
    Instruct("ADC", Opcode::ADC, AddresMode::IZY, 5),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 8),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 4),
    Instruct("ADC", Opcode::ADC, AddresMode::ZPX, 4),
    Instruct("ROR", Opcode::ROR, AddresMode::ZPX, 6),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 6),
    Instruct("SEI", Opcode::SEI, AddresMode::IMP, 2),
    Instruct("ADC", Opcode::ADC, AddresMode::ABY, 4),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 7),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 4),
    Instruct("ADC", Opcode::ADC, AddresMode::ABX, 4),
    Instruct("ROR", Opcode::ROR, AddresMode::ABX, 7),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 7),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 2),
    Instruct("STA", Opcode::STA, AddresMode::IZX, 6),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 6),
    Instruct("STY", Opcode::STY, AddresMode::ZP0, 3),
    Instruct("STA", Opcode::STA, AddresMode::ZP0, 3),
    Instruct("STX", Opcode::STX, AddresMode::ZP0, 3),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 3),
    Instruct("DEY", Opcode::DEY, AddresMode::IMP, 2),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 2),
    Instruct("TXA", Opcode::TXA, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 2),
    Instruct("STY", Opcode::STY, AddresMode::ABS, 4),
    Instruct("STA", Opcode::STA, AddresMode::ABS, 4),
    Instruct("STX", Opcode::STX, AddresMode::ABS, 4),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 4),
    Instruct("BCC", Opcode::BCC, AddresMode::REL, 2),
    Instruct("STA", Opcode::STA, AddresMode::IZY, 6),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 6),
    Instruct("STY", Opcode::STY, AddresMode::ZPX, 4),
    Instruct("STA", Opcode::STA, AddresMode::ZPX, 4),
    Instruct("STX", Opcode::STX, AddresMode::ZPY, 4),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 4),
    Instruct("TYA", Opcode::TYA, AddresMode::IMP, 2),
    Instruct("STA", Opcode::STA, AddresMode::ABY, 5),
    Instruct("TXS", Opcode::TXS, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 5),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 5),
    Instruct("STA", Opcode::STA, AddresMode::ABX, 5),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 5),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 5),
    Instruct("LDY", Opcode::LDY, AddresMode::IMM, 2),
    Instruct("LDA", Opcode::LDA, AddresMode::IZX, 6),
    Instruct("LDX", Opcode::LDX, AddresMode::IMM, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 6),
    Instruct("LDY", Opcode::LDY, AddresMode::ZP0, 3),
    Instruct("LDA", Opcode::LDA, AddresMode::ZP0, 3),
    Instruct("LDX", Opcode::LDX, AddresMode::ZP0, 3),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 3),
    Instruct("TAY", Opcode::TAY, AddresMode::IMP, 2),
    Instruct("LDA", Opcode::LDA, AddresMode::IMM, 2),
    Instruct("TAX", Opcode::TAX, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 2),
    Instruct("LDY", Opcode::LDY, AddresMode::ABS, 4),
    Instruct("LDA", Opcode::LDA, AddresMode::ABS, 4),
    Instruct("LDX", Opcode::LDX, AddresMode::ABS, 4),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 4),
    Instruct("BCS", Opcode::BCS, AddresMode::REL, 2),
    Instruct("LDA", Opcode::LDA, AddresMode::IZY, 5),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 5),
    Instruct("LDY", Opcode::LDY, AddresMode::ZPX, 4),
    Instruct("LDA", Opcode::LDA, AddresMode::ZPX, 4),
    Instruct("LDX", Opcode::LDX, AddresMode::ZPY, 4),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 4),
    Instruct("CLV", Opcode::CLV, AddresMode::IMP, 2),
    Instruct("LDA", Opcode::LDA, AddresMode::ABY, 4),
    Instruct("TSX", Opcode::TSX, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 4),
    Instruct("LDY", Opcode::LDY, AddresMode::ABX, 4),
    Instruct("LDA", Opcode::LDA, AddresMode::ABX, 4),
    Instruct("LDX", Opcode::LDX, AddresMode::ABY, 4),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 4),
    Instruct("CPY", Opcode::CPY, AddresMode::IMM, 2),
    Instruct("CMP", Opcode::CMP, AddresMode::IZX, 6),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 8),
    Instruct("CPY", Opcode::CPY, AddresMode::ZP0, 3),
    Instruct("CMP", Opcode::CMP, AddresMode::ZP0, 3),
    Instruct("DEC", Opcode::DEC, AddresMode::ZP0, 5),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 5),
    Instruct("INY", Opcode::INY, AddresMode::IMP, 2),
    Instruct("CMP", Opcode::CMP, AddresMode::IMM, 2),
    Instruct("DEX", Opcode::DEX, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 2),
    Instruct("CPY", Opcode::CPY, AddresMode::ABS, 4),
    Instruct("CMP", Opcode::CMP, AddresMode::ABS, 4),
    Instruct("DEC", Opcode::DEC, AddresMode::ABS, 6),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 6),
    Instruct("BNE", Opcode::BNE, AddresMode::REL, 2),
    Instruct("CMP", Opcode::CMP, AddresMode::IZY, 5),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 8),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 4),
    Instruct("CMP", Opcode::CMP, AddresMode::ZPX, 4),
    Instruct("DEC", Opcode::DEC, AddresMode::ZPX, 6),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 6),
    Instruct("CLD", Opcode::CLD, AddresMode::IMP, 2),
    Instruct("CMP", Opcode::CMP, AddresMode::ABY, 4),
    Instruct("NOP", Opcode::NOP, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 7),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 4),
    Instruct("CMP", Opcode::CMP, AddresMode::ABX, 4),
    Instruct("DEC", Opcode::DEC, AddresMode::ABX, 7),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 7),
    Instruct("CPX", Opcode::CPX, AddresMode::IMM, 2),
    Instruct("SBC", Opcode::SBC, AddresMode::IZX, 6),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 8),
    Instruct("CPX", Opcode::CPX, AddresMode::ZP0, 3),
    Instruct("SBC", Opcode::SBC, AddresMode::ZP0, 3),
    Instruct("INC", Opcode::INC, AddresMode::ZP0, 5),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 5),
    Instruct("INX", Opcode::INX, AddresMode::IMP, 2),
    Instruct("SBC", Opcode::SBC, AddresMode::IMM, 2),
    Instruct("NOP", Opcode::NOP, AddresMode::IMP, 2),
    Instruct("???", Opcode::SBC, AddresMode::IMP, 2),
    Instruct("CPX", Opcode::CPX, AddresMode::ABS, 4),
    Instruct("SBC", Opcode::SBC, AddresMode::ABS, 4),
    Instruct("INC", Opcode::INC, AddresMode::ABS, 6),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 6),
    Instruct("BEQ", Opcode::BEQ, AddresMode::REL, 2),
    Instruct("SBC", Opcode::SBC, AddresMode::IZY, 5),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 8),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 4),
    Instruct("SBC", Opcode::SBC, AddresMode::ZPX, 4),
    Instruct("INC", Opcode::INC, AddresMode::ZPX, 6),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 6),
    Instruct("SED", Opcode::SED, AddresMode::IMP, 2),
    Instruct("SBC", Opcode::SBC, AddresMode::ABY, 4),
    Instruct("NOP", Opcode::NOP, AddresMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 7),
    Instruct("???", Opcode::NOP, AddresMode::IMP, 4),
    Instruct("SBC", Opcode::SBC, AddresMode::ABX, 4),
    Instruct("INC", Opcode::INC, AddresMode::ABX, 7),
    Instruct("???", Opcode::XXX, AddresMode::IMP, 7),
];

pub fn get_instruction_by_id(opcode: u8) -> Instruction {
    let result = &LIST[opcode as usize];

    result.get()
}

impl Cpu6502 {
    pub fn get_instruction(&self) -> Instruction {
        get_instruction_by_id(self.opcode)
    }

    /** This function sources the data used by the instruction into
        a convenient numeric variable. Some instructions dont have to
        fetch data as the source is implied by the instruction. For example
        "INX" increments the X register. There is no additional data
        required. For all other addressing modes, the data resides at
        the location held within addr_abs, so it is read from there.
        Immediate adress mode exploits this slightly, as that has
        set addr_abs = pc + 1, so it fetches the data from the
        next byte for example "LDA $FF" just loads the accumulator with
        256, i.e. no far reaching memory fetch is required. "fetched"
        is a variable global to the CPU, and is set by cal
    */
    pub fn fetch(&mut self) -> u8 {
        if AddresMode::IMP != self.get_instruction().addres_mode {
            self.fetched = self.read(self.addr_abs);
        }

        self.fetched
    }
}
