use super::addres_mode::AddressMode;
use super::opcode::Opcode;
use super::Cpu6502;

#[derive(Copy, Clone)]
pub struct Instruct(&'static str, Opcode, AddressMode, u8);

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub addres_mode: AddressMode,
    pub cycles: u8,
    pub name: &'static str,
}

impl Instruction {
    pub fn new(opcode: Opcode, addres_mode: AddressMode) -> Instruction {
        Instruction {
            name: "test",
            addres_mode,
            cycles: 4,
            opcode,
        }
    }

    pub fn from(inst_code: u8) -> Instruction {
        match inst_code {
            /* *************** binary op ***************  */
            0x69 => Instruction::new(Opcode::ADC, AddressMode::IMM),
            0x65 => Instruction::new(Opcode::ADC, AddressMode::ZP0),
            0x75 => Instruction::new(Opcode::ADC, AddressMode::ZPX),
            0x6d => Instruction::new(Opcode::ADC, AddressMode::ABS),
            0x7d => Instruction::new(Opcode::ADC, AddressMode::ABX),
            0x79 => Instruction::new(Opcode::ADC, AddressMode::ABY),
            0x61 => Instruction::new(Opcode::ADC, AddressMode::IZX),
            0x71 => Instruction::new(Opcode::ADC, AddressMode::IZY),

            0xe9 => Instruction::new(Opcode::SBC, AddressMode::IMM),
            0xe5 => Instruction::new(Opcode::SBC, AddressMode::ZP0),
            0xf5 => Instruction::new(Opcode::SBC, AddressMode::ZPX),
            0xed => Instruction::new(Opcode::SBC, AddressMode::ABS),
            0xfd => Instruction::new(Opcode::SBC, AddressMode::ABX),
            0xf9 => Instruction::new(Opcode::SBC, AddressMode::ABY),
            0xe1 => Instruction::new(Opcode::SBC, AddressMode::IZX),
            0xf1 => Instruction::new(Opcode::SBC, AddressMode::IZY),

            0x29 => Instruction::new(Opcode::AND, AddressMode::IMM),
            0x25 => Instruction::new(Opcode::AND, AddressMode::ZP0),
            0x35 => Instruction::new(Opcode::AND, AddressMode::ZPX),
            0x2d => Instruction::new(Opcode::AND, AddressMode::ABS),
            0x3d => Instruction::new(Opcode::AND, AddressMode::ABX),
            0x39 => Instruction::new(Opcode::AND, AddressMode::ABY),
            0x21 => Instruction::new(Opcode::AND, AddressMode::IZX),
            0x31 => Instruction::new(Opcode::AND, AddressMode::IZY),

            0x49 => Instruction::new(Opcode::EOR, AddressMode::IMM),
            0x45 => Instruction::new(Opcode::EOR, AddressMode::ZP0),
            0x55 => Instruction::new(Opcode::EOR, AddressMode::ZPX),
            0x4d => Instruction::new(Opcode::EOR, AddressMode::ABS),
            0x5d => Instruction::new(Opcode::EOR, AddressMode::ABX),
            0x59 => Instruction::new(Opcode::EOR, AddressMode::ABY),
            0x41 => Instruction::new(Opcode::EOR, AddressMode::IZX),
            0x51 => Instruction::new(Opcode::EOR, AddressMode::IZY),

            0x09 => Instruction::new(Opcode::ORA, AddressMode::IMM),
            0x05 => Instruction::new(Opcode::ORA, AddressMode::ZP0),
            0x15 => Instruction::new(Opcode::ORA, AddressMode::ZPX),
            0x0d => Instruction::new(Opcode::ORA, AddressMode::ABS),
            0x1d => Instruction::new(Opcode::ORA, AddressMode::ABX),
            0x19 => Instruction::new(Opcode::ORA, AddressMode::ABY),
            0x01 => Instruction::new(Opcode::ORA, AddressMode::IZX),
            0x11 => Instruction::new(Opcode::ORA, AddressMode::IZY),

            /* *************** shift/rotate op ***************  */
            0x0a => Instruction::new(Opcode::ASL, AddressMode::ACC),
            0x06 => Instruction::new(Opcode::ASL, AddressMode::ZP0),
            0x16 => Instruction::new(Opcode::ASL, AddressMode::ZPX),
            0x0e => Instruction::new(Opcode::ASL, AddressMode::ABS),
            0x1e => Instruction::new(Opcode::ASL, AddressMode::ABX),

            0x4a => Instruction::new(Opcode::LSR, AddressMode::ACC),
            0x46 => Instruction::new(Opcode::LSR, AddressMode::ZP0),
            0x56 => Instruction::new(Opcode::LSR, AddressMode::ZPX),
            0x4e => Instruction::new(Opcode::LSR, AddressMode::ABS),
            0x5e => Instruction::new(Opcode::LSR, AddressMode::ABX),

            0x2a => Instruction::new(Opcode::ROL, AddressMode::ACC),
            0x26 => Instruction::new(Opcode::ROL, AddressMode::ZP0),
            0x36 => Instruction::new(Opcode::ROL, AddressMode::ZPX),
            0x2e => Instruction::new(Opcode::ROL, AddressMode::ABS),
            0x3e => Instruction::new(Opcode::ROL, AddressMode::ABX),

            0x6a => Instruction::new(Opcode::ROR, AddressMode::ACC),
            0x66 => Instruction::new(Opcode::ROR, AddressMode::ZP0),
            0x76 => Instruction::new(Opcode::ROR, AddressMode::ZPX),
            0x6e => Instruction::new(Opcode::ROR, AddressMode::ABS),
            0x7e => Instruction::new(Opcode::ROR, AddressMode::ABX),

            /* *************** inc/dec op ***************  */
            0xe6 => Instruction::new(Opcode::INC, AddressMode::ZP0),
            0xf6 => Instruction::new(Opcode::INC, AddressMode::ZPX),
            0xee => Instruction::new(Opcode::INC, AddressMode::ABS),
            0xfe => Instruction::new(Opcode::INC, AddressMode::ABX),

            0xe8 => Instruction::new(Opcode::INX, AddressMode::IMP),
            0xc8 => Instruction::new(Opcode::INY, AddressMode::IMP),

            0xc6 => Instruction::new(Opcode::DEC, AddressMode::ZP0),
            0xd6 => Instruction::new(Opcode::DEC, AddressMode::ZPX),
            0xce => Instruction::new(Opcode::DEC, AddressMode::ABS),
            0xde => Instruction::new(Opcode::DEC, AddressMode::ABX),

            0xca => Instruction::new(Opcode::DEX, AddressMode::IMP),
            0x88 => Instruction::new(Opcode::DEY, AddressMode::IMP),

            /* *************** load/store op ***************  */
            0xa9 => Instruction::new(Opcode::LDA, AddressMode::IMM),
            0xa5 => Instruction::new(Opcode::LDA, AddressMode::ZP0),
            0xb5 => Instruction::new(Opcode::LDA, AddressMode::ZPX),
            0xad => Instruction::new(Opcode::LDA, AddressMode::ABS),
            0xbd => Instruction::new(Opcode::LDA, AddressMode::ABX),
            0xb9 => Instruction::new(Opcode::LDA, AddressMode::ABY),
            0xa1 => Instruction::new(Opcode::LDA, AddressMode::IZX),
            0xb1 => Instruction::new(Opcode::LDA, AddressMode::IZY),

            0xa2 => Instruction::new(Opcode::LDX, AddressMode::IMM),
            0xa6 => Instruction::new(Opcode::LDX, AddressMode::ZP0),
            0xb6 => Instruction::new(Opcode::LDX, AddressMode::ZPY),
            0xae => Instruction::new(Opcode::LDX, AddressMode::ABS),
            0xbe => Instruction::new(Opcode::LDX, AddressMode::ABY),

            0xa0 => Instruction::new(Opcode::LDY, AddressMode::IMM),
            0xa4 => Instruction::new(Opcode::LDY, AddressMode::ZP0),
            0xb4 => Instruction::new(Opcode::LDY, AddressMode::ZPX),
            0xac => Instruction::new(Opcode::LDY, AddressMode::ABS),
            0xbc => Instruction::new(Opcode::LDY, AddressMode::ABX),

            0x85 => Instruction::new(Opcode::STA, AddressMode::ZP0),
            0x95 => Instruction::new(Opcode::STA, AddressMode::ZPX),
            0x8d => Instruction::new(Opcode::STA, AddressMode::ABS),
            0x9d => Instruction::new(Opcode::STA, AddressMode::ABX),
            0x99 => Instruction::new(Opcode::STA, AddressMode::ABY),
            0x81 => Instruction::new(Opcode::STA, AddressMode::IZX),
            0x91 => Instruction::new(Opcode::STA, AddressMode::IZY),

            0x86 => Instruction::new(Opcode::STX, AddressMode::ZP0),
            0x96 => Instruction::new(Opcode::STX, AddressMode::ZPY),
            0x8e => Instruction::new(Opcode::STX, AddressMode::ABS),

            0x84 => Instruction::new(Opcode::STY, AddressMode::ZP0),
            0x94 => Instruction::new(Opcode::STY, AddressMode::ZPX),
            0x8c => Instruction::new(Opcode::STY, AddressMode::ABS),

            /* *************** set/clear flag ***************  */
            0x38 => Instruction::new(Opcode::SEC, AddressMode::IMP),
            0xf8 => Instruction::new(Opcode::SED, AddressMode::IMP),
            0x78 => Instruction::new(Opcode::SEI, AddressMode::IMP),
            0x18 => Instruction::new(Opcode::CLC, AddressMode::IMP),
            0xd8 => Instruction::new(Opcode::CLD, AddressMode::IMP),
            0x58 => Instruction::new(Opcode::CLI, AddressMode::IMP),
            0xb8 => Instruction::new(Opcode::CLV, AddressMode::IMP),

            /* *************** compare ***************  */
            0xc9 => Instruction::new(Opcode::CMP, AddressMode::IMM),
            0xc5 => Instruction::new(Opcode::CMP, AddressMode::ZP0),
            0xd5 => Instruction::new(Opcode::CMP, AddressMode::ZPX),
            0xcd => Instruction::new(Opcode::CMP, AddressMode::ABS),
            0xdd => Instruction::new(Opcode::CMP, AddressMode::ABX),
            0xd9 => Instruction::new(Opcode::CMP, AddressMode::ABY),
            0xc1 => Instruction::new(Opcode::CMP, AddressMode::IZX),
            0xd1 => Instruction::new(Opcode::CMP, AddressMode::IZY),

            0xe0 => Instruction::new(Opcode::CPX, AddressMode::IMM),
            0xe4 => Instruction::new(Opcode::CPX, AddressMode::ZP0),
            0xec => Instruction::new(Opcode::CPX, AddressMode::ABS),

            0xc0 => Instruction::new(Opcode::CPY, AddressMode::IMM),
            0xc4 => Instruction::new(Opcode::CPY, AddressMode::ZP0),
            0xcc => Instruction::new(Opcode::CPY, AddressMode::ABS),

            /* *************** jump/return ***************  */
            0x4c => Instruction::new(Opcode::JMP, AddressMode::ABS),
            0x6c => Instruction::new(Opcode::JMP, AddressMode::IND),

            0x20 => Instruction::new(Opcode::JSR, AddressMode::ABS),

            0x40 => Instruction::new(Opcode::RTI, AddressMode::IMP),
            0x60 => Instruction::new(Opcode::RTS, AddressMode::IMP),

            /* *************** branch ***************  */
            0x90 => Instruction::new(Opcode::BCC, AddressMode::REL),
            0xb0 => Instruction::new(Opcode::BCS, AddressMode::REL),
            0xf0 => Instruction::new(Opcode::BEQ, AddressMode::REL),
            0xd0 => Instruction::new(Opcode::BNE, AddressMode::REL),
            0x30 => Instruction::new(Opcode::BMI, AddressMode::REL),
            0x10 => Instruction::new(Opcode::BPL, AddressMode::REL),
            0x50 => Instruction::new(Opcode::BVC, AddressMode::REL),
            0x70 => Instruction::new(Opcode::BVS, AddressMode::REL),

            /* *************** push/pop ***************  */
            0x48 => Instruction::new(Opcode::PHA, AddressMode::IMP),
            0x08 => Instruction::new(Opcode::PHP, AddressMode::IMP),
            0x68 => Instruction::new(Opcode::PLA, AddressMode::IMP),
            0x28 => Instruction::new(Opcode::PLP, AddressMode::IMP),

            /* *************** transfer ***************  */
            0xaa => Instruction::new(Opcode::TAX, AddressMode::IMP),
            0xa8 => Instruction::new(Opcode::TAY, AddressMode::IMP),
            0xba => Instruction::new(Opcode::TSX, AddressMode::IMP),
            0x8a => Instruction::new(Opcode::TXA, AddressMode::IMP),
            0x9a => Instruction::new(Opcode::TXS, AddressMode::IMP),
            0x98 => Instruction::new(Opcode::TYA, AddressMode::IMP),

            /* *************** other ***************  */
            0x00 => Instruction::new(Opcode::BRK, AddressMode::IMP),

            0x24 => Instruction::new(Opcode::BIT, AddressMode::ZP0),
            0x2c => Instruction::new(Opcode::BIT, AddressMode::ABS),

            0xea => Instruction::new(Opcode::NOP, AddressMode::IMP),

            /* *************** unofficial1 ***************  */
            // 0x4b => Instruction::new(Opcode::ALR, AddressMode::IMM),
            // 0x0b => Instruction::new(Opcode::ANC, AddressMode::IMM),
            // 0x6b => Instruction::new(Opcode::ARR, AddressMode::IMM),
            // 0xcb => Instruction::new(Opcode::AXS, AddressMode::IMM),

            // 0xa3 => Instruction::new(Opcode::LAX, AddressMode::IZX),
            // 0xa7 => Instruction::new(Opcode::LAX, AddressMode::ZP0),
            // 0xaf => Instruction::new(Opcode::LAX, AddressMode::ABS),
            // 0xb3 => Instruction::new(Opcode::LAX, AddressMode::IZY),
            // 0xb7 => Instruction::new(Opcode::LAX, AddressMode::ZPY),
            // 0xbf => Instruction::new(Opcode::LAX, AddressMode::ABY),

            // 0x83 => Instruction::new(Opcode::SAX, AddressMode::IZX),
            // 0x87 => Instruction::new(Opcode::SAX, AddressMode::ZP0),
            // 0x8f => Instruction::new(Opcode::SAX, AddressMode::ABS),
            // 0x97 => Instruction::new(Opcode::SAX, AddressMode::ZPY),

            // 0xc3 => Instruction::new(Opcode::DCP, AddressMode::IZX),
            // 0xc7 => Instruction::new(Opcode::DCP, AddressMode::ZP0),
            // 0xcf => Instruction::new(Opcode::DCP, AddressMode::ABS),
            // 0xd3 => Instruction::new(Opcode::DCP, AddressMode::IZY),
            // 0xd7 => Instruction::new(Opcode::DCP, AddressMode::ZPX),
            // 0xdb => Instruction::new(Opcode::DCP, AddressMode::ABY),
            // 0xdf => Instruction::new(Opcode::DCP, AddressMode::ABX),

            // 0xe3 => Instruction::new(Opcode::ISC, AddressMode::IZX),
            // 0xe7 => Instruction::new(Opcode::ISC, AddressMode::ZP0),
            // 0xef => Instruction::new(Opcode::ISC, AddressMode::ABS),
            // 0xf3 => Instruction::new(Opcode::ISC, AddressMode::IZY),
            // 0xf7 => Instruction::new(Opcode::ISC, AddressMode::ZPX),
            // 0xfb => Instruction::new(Opcode::ISC, AddressMode::ABY),
            // 0xff => Instruction::new(Opcode::ISC, AddressMode::ABX),

            // 0x23 => Instruction::new(Opcode::RLA, AddressMode::IZX),
            // 0x27 => Instruction::new(Opcode::RLA, AddressMode::ZP0),
            // 0x2f => Instruction::new(Opcode::RLA, AddressMode::ABS),
            // 0x33 => Instruction::new(Opcode::RLA, AddressMode::IZY),
            // 0x37 => Instruction::new(Opcode::RLA, AddressMode::ZPX),
            // 0x3b => Instruction::new(Opcode::RLA, AddressMode::ABY),
            // 0x3f => Instruction::new(Opcode::RLA, AddressMode::ABX),

            // 0x63 => Instruction::new(Opcode::RRA, AddressMode::IZX),
            // 0x67 => Instruction::new(Opcode::RRA, AddressMode::ZP0),
            // 0x6f => Instruction::new(Opcode::RRA, AddressMode::ABS),
            // 0x73 => Instruction::new(Opcode::RRA, AddressMode::IZY),
            // 0x77 => Instruction::new(Opcode::RRA, AddressMode::ZPX),
            // 0x7b => Instruction::new(Opcode::RRA, AddressMode::ABY),
            // 0x7f => Instruction::new(Opcode::RRA, AddressMode::ABX),

            // 0x03 => Instruction::new(Opcode::SLO, AddressMode::IZX),
            // 0x07 => Instruction::new(Opcode::SLO, AddressMode::ZP0),
            // 0x0f => Instruction::new(Opcode::SLO, AddressMode::ABS),
            // 0x13 => Instruction::new(Opcode::SLO, AddressMode::IZY),
            // 0x17 => Instruction::new(Opcode::SLO, AddressMode::ZPX),
            // 0x1b => Instruction::new(Opcode::SLO, AddressMode::ABY),
            // 0x1f => Instruction::new(Opcode::SLO, AddressMode::ABX),

            // 0x43 => Instruction::new(Opcode::SRE, AddressMode::IZX),
            // 0x47 => Instruction::new(Opcode::SRE, AddressMode::ZP0),
            // 0x4f => Instruction::new(Opcode::SRE, AddressMode::ABS),
            // 0x53 => Instruction::new(Opcode::SRE, AddressMode::IZY),
            // 0x57 => Instruction::new(Opcode::SRE, AddressMode::ZPX),
            // 0x5b => Instruction::new(Opcode::SRE, AddressMode::ABY),
            // 0x5f => Instruction::new(Opcode::SRE, AddressMode::ABX),

            // 0x80 => Instruction::new(Opcode::SKB, AddressMode::IMM),
            // 0x82 => Instruction::new(Opcode::SKB, AddressMode::IMM),
            // 0x89 => Instruction::new(Opcode::SKB, AddressMode::IMM),
            // 0xc2 => Instruction::new(Opcode::SKB, AddressMode::IMM),
            // 0xe2 => Instruction::new(Opcode::SKB, AddressMode::IMM),

            // 0x0c => Instruction::new(Opcode::IGN, AddressMode::ABS),

            // 0x1c => Instruction::new(Opcode::IGN, AddressMode::ABX),
            // 0x3c => Instruction::new(Opcode::IGN, AddressMode::ABX),
            // 0x5c => Instruction::new(Opcode::IGN, AddressMode::ABX),
            // 0x7c => Instruction::new(Opcode::IGN, AddressMode::ABX),
            // 0xdc => Instruction::new(Opcode::IGN, AddressMode::ABX),
            // 0xfc => Instruction::new(Opcode::IGN, AddressMode::ABX),

            // 0x04 => Instruction::new(Opcode::IGN, AddressMode::ZP0),
            // 0x44 => Instruction::new(Opcode::IGN, AddressMode::ZP0),
            // 0x64 => Instruction::new(Opcode::IGN, AddressMode::ZP0),

            // 0x14 => Instruction::new(Opcode::IGN, AddressMode::ZPX),
            // 0x34 => Instruction::new(Opcode::IGN, AddressMode::ZPX),
            // 0x54 => Instruction::new(Opcode::IGN, AddressMode::ZPX),
            // 0x74 => Instruction::new(Opcode::IGN, AddressMode::ZPX),
            // 0xd4 => Instruction::new(Opcode::IGN, AddressMode::ZPX),
            // 0xf4 => Instruction::new(Opcode::IGN, AddressMode::ZPX),

            /* *************** unofficial2(既存の命令) ***************  */
            0xeb => Instruction::new(Opcode::SBC, AddressMode::IMM),

            0x1a => Instruction::new(Opcode::NOP, AddressMode::IMP),
            0x3a => Instruction::new(Opcode::NOP, AddressMode::IMP),
            0x5a => Instruction::new(Opcode::NOP, AddressMode::IMP),
            0x7a => Instruction::new(Opcode::NOP, AddressMode::IMP),
            0xda => Instruction::new(Opcode::NOP, AddressMode::IMP),
            0xfa => Instruction::new(Opcode::NOP, AddressMode::IMP),

            _ => panic!("Invalid inst_code:{:08x}", inst_code),
        }
    }
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
    Instruct("BRK", Opcode::BRK, AddressMode::IMM, 7),
    Instruct("ORA", Opcode::ORA, AddressMode::IZX, 6),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 8),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 3),
    Instruct("ORA", Opcode::ORA, AddressMode::ZP0, 3),
    Instruct("ASL", Opcode::ASL, AddressMode::ZP0, 5),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 5),
    Instruct("PHP", Opcode::PHP, AddressMode::IMP, 3),
    Instruct("ORA", Opcode::ORA, AddressMode::IMM, 2),
    Instruct("ASL", Opcode::ASL, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 2),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 4),
    Instruct("ORA", Opcode::ORA, AddressMode::ABS, 4),
    Instruct("ASL", Opcode::ASL, AddressMode::ABS, 6),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 6),
    Instruct("BPL", Opcode::BPL, AddressMode::REL, 2),
    Instruct("ORA", Opcode::ORA, AddressMode::IZY, 5),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 8),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 4),
    Instruct("ORA", Opcode::ORA, AddressMode::ZPX, 4),
    Instruct("ASL", Opcode::ASL, AddressMode::ZPX, 6),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 6),
    Instruct("CLC", Opcode::CLC, AddressMode::IMP, 2),
    Instruct("ORA", Opcode::ORA, AddressMode::ABY, 4),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 7),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 4),
    Instruct("ORA", Opcode::ORA, AddressMode::ABX, 4),
    Instruct("ASL", Opcode::ASL, AddressMode::ABX, 7),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 7),
    Instruct("JSR", Opcode::JSR, AddressMode::ABS, 6),
    Instruct("AND", Opcode::AND, AddressMode::IZX, 6),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 8),
    Instruct("BIT", Opcode::BIT, AddressMode::ZP0, 3),
    Instruct("AND", Opcode::AND, AddressMode::ZP0, 3),
    Instruct("ROL", Opcode::ROL, AddressMode::ZP0, 5),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 5),
    Instruct("PLP", Opcode::PLP, AddressMode::IMP, 4),
    Instruct("AND", Opcode::AND, AddressMode::IMM, 2),
    Instruct("ROL", Opcode::ROL, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 2),
    Instruct("BIT", Opcode::BIT, AddressMode::ABS, 4),
    Instruct("AND", Opcode::AND, AddressMode::ABS, 4),
    Instruct("ROL", Opcode::ROL, AddressMode::ABS, 6),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 6),
    Instruct("BMI", Opcode::BMI, AddressMode::REL, 2),
    Instruct("AND", Opcode::AND, AddressMode::IZY, 5),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 8),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 4),
    Instruct("AND", Opcode::AND, AddressMode::ZPX, 4),
    Instruct("ROL", Opcode::ROL, AddressMode::ZPX, 6),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 6),
    Instruct("SEC", Opcode::SEC, AddressMode::IMP, 2),
    Instruct("AND", Opcode::AND, AddressMode::ABY, 4),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 7),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 4),
    Instruct("AND", Opcode::AND, AddressMode::ABX, 4),
    Instruct("ROL", Opcode::ROL, AddressMode::ABX, 7),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 7),
    Instruct("RTI", Opcode::RTI, AddressMode::IMP, 6),
    Instruct("EOR", Opcode::EOR, AddressMode::IZX, 6),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 8),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 3),
    Instruct("EOR", Opcode::EOR, AddressMode::ZP0, 3),
    Instruct("LSR", Opcode::LSR, AddressMode::ZP0, 5),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 5),
    Instruct("PHA", Opcode::PHA, AddressMode::IMP, 3),
    Instruct("EOR", Opcode::EOR, AddressMode::IMM, 2),
    Instruct("LSR", Opcode::LSR, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 2),
    Instruct("JMP", Opcode::JMP, AddressMode::ABS, 3),
    Instruct("EOR", Opcode::EOR, AddressMode::ABS, 4),
    Instruct("LSR", Opcode::LSR, AddressMode::ABS, 6),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 6),
    Instruct("BVC", Opcode::BVC, AddressMode::REL, 2),
    Instruct("EOR", Opcode::EOR, AddressMode::IZY, 5),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 8),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 4),
    Instruct("EOR", Opcode::EOR, AddressMode::ZPX, 4),
    Instruct("LSR", Opcode::LSR, AddressMode::ZPX, 6),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 6),
    Instruct("CLI", Opcode::CLI, AddressMode::IMP, 2),
    Instruct("EOR", Opcode::EOR, AddressMode::ABY, 4),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 7),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 4),
    Instruct("EOR", Opcode::EOR, AddressMode::ABX, 4),
    Instruct("LSR", Opcode::LSR, AddressMode::ABX, 7),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 7),
    Instruct("RTS", Opcode::RTS, AddressMode::IMP, 6),
    Instruct("ADC", Opcode::ADC, AddressMode::IZX, 6),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 8),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 3),
    Instruct("ADC", Opcode::ADC, AddressMode::ZP0, 3),
    Instruct("ROR", Opcode::ROR, AddressMode::ZP0, 5),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 5),
    Instruct("PLA", Opcode::PLA, AddressMode::IMP, 4),
    Instruct("ADC", Opcode::ADC, AddressMode::IMM, 2),
    Instruct("ROR", Opcode::ROR, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 2),
    Instruct("JMP", Opcode::JMP, AddressMode::IND, 5),
    Instruct("ADC", Opcode::ADC, AddressMode::ABS, 4),
    Instruct("ROR", Opcode::ROR, AddressMode::ABS, 6),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 6),
    Instruct("BVS", Opcode::BVS, AddressMode::REL, 2),
    Instruct("ADC", Opcode::ADC, AddressMode::IZY, 5),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 8),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 4),
    Instruct("ADC", Opcode::ADC, AddressMode::ZPX, 4),
    Instruct("ROR", Opcode::ROR, AddressMode::ZPX, 6),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 6),
    Instruct("SEI", Opcode::SEI, AddressMode::IMP, 2),
    Instruct("ADC", Opcode::ADC, AddressMode::ABY, 4),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 7),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 4),
    Instruct("ADC", Opcode::ADC, AddressMode::ABX, 4),
    Instruct("ROR", Opcode::ROR, AddressMode::ABX, 7),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 7),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 2),
    Instruct("STA", Opcode::STA, AddressMode::IZX, 6),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 6),
    Instruct("STY", Opcode::STY, AddressMode::ZP0, 3),
    Instruct("STA", Opcode::STA, AddressMode::ZP0, 3),
    Instruct("STX", Opcode::STX, AddressMode::ZP0, 3),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 3),
    Instruct("DEY", Opcode::DEY, AddressMode::IMP, 2),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 2),
    Instruct("TXA", Opcode::TXA, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 2),
    Instruct("STY", Opcode::STY, AddressMode::ABS, 4),
    Instruct("STA", Opcode::STA, AddressMode::ABS, 4),
    Instruct("STX", Opcode::STX, AddressMode::ABS, 4),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 4),
    Instruct("BCC", Opcode::BCC, AddressMode::REL, 2),
    Instruct("STA", Opcode::STA, AddressMode::IZY, 6),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 6),
    Instruct("STY", Opcode::STY, AddressMode::ZPX, 4),
    Instruct("STA", Opcode::STA, AddressMode::ZPX, 4),
    Instruct("STX", Opcode::STX, AddressMode::ZPY, 4),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 4),
    Instruct("TYA", Opcode::TYA, AddressMode::IMP, 2),
    Instruct("STA", Opcode::STA, AddressMode::ABY, 5),
    Instruct("TXS", Opcode::TXS, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 5),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 5),
    Instruct("STA", Opcode::STA, AddressMode::ABX, 5),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 5),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 5),
    Instruct("LDY", Opcode::LDY, AddressMode::IMM, 2),
    Instruct("LDA", Opcode::LDA, AddressMode::IZX, 6),
    Instruct("LDX", Opcode::LDX, AddressMode::IMM, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 6),
    Instruct("LDY", Opcode::LDY, AddressMode::ZP0, 3),
    Instruct("LDA", Opcode::LDA, AddressMode::ZP0, 3),
    Instruct("LDX", Opcode::LDX, AddressMode::ZP0, 3),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 3),
    Instruct("TAY", Opcode::TAY, AddressMode::IMP, 2),
    Instruct("LDA", Opcode::LDA, AddressMode::IMM, 2),
    Instruct("TAX", Opcode::TAX, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 2),
    Instruct("LDY", Opcode::LDY, AddressMode::ABS, 4),
    Instruct("LDA", Opcode::LDA, AddressMode::ABS, 4),
    Instruct("LDX", Opcode::LDX, AddressMode::ABS, 4),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 4),
    Instruct("BCS", Opcode::BCS, AddressMode::REL, 2),
    Instruct("LDA", Opcode::LDA, AddressMode::IZY, 5),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 5),
    Instruct("LDY", Opcode::LDY, AddressMode::ZPX, 4),
    Instruct("LDA", Opcode::LDA, AddressMode::ZPX, 4),
    Instruct("LDX", Opcode::LDX, AddressMode::ZPY, 4),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 4),
    Instruct("CLV", Opcode::CLV, AddressMode::IMP, 2),
    Instruct("LDA", Opcode::LDA, AddressMode::ABY, 4),
    Instruct("TSX", Opcode::TSX, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 4),
    Instruct("LDY", Opcode::LDY, AddressMode::ABX, 4),
    Instruct("LDA", Opcode::LDA, AddressMode::ABX, 4),
    Instruct("LDX", Opcode::LDX, AddressMode::ABY, 4),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 4),
    Instruct("CPY", Opcode::CPY, AddressMode::IMM, 2),
    Instruct("CMP", Opcode::CMP, AddressMode::IZX, 6),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 8),
    Instruct("CPY", Opcode::CPY, AddressMode::ZP0, 3),
    Instruct("CMP", Opcode::CMP, AddressMode::ZP0, 3),
    Instruct("DEC", Opcode::DEC, AddressMode::ZP0, 5),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 5),
    Instruct("INY", Opcode::INY, AddressMode::IMP, 2),
    Instruct("CMP", Opcode::CMP, AddressMode::IMM, 2),
    Instruct("DEX", Opcode::DEX, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 2),
    Instruct("CPY", Opcode::CPY, AddressMode::ABS, 4),
    Instruct("CMP", Opcode::CMP, AddressMode::ABS, 4),
    Instruct("DEC", Opcode::DEC, AddressMode::ABS, 6),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 6),
    Instruct("BNE", Opcode::BNE, AddressMode::REL, 2),
    Instruct("CMP", Opcode::CMP, AddressMode::IZY, 5),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 8),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 4),
    Instruct("CMP", Opcode::CMP, AddressMode::ZPX, 4),
    Instruct("DEC", Opcode::DEC, AddressMode::ZPX, 6),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 6),
    Instruct("CLD", Opcode::CLD, AddressMode::IMP, 2),
    Instruct("CMP", Opcode::CMP, AddressMode::ABY, 4),
    Instruct("NOP", Opcode::NOP, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 7),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 4),
    Instruct("CMP", Opcode::CMP, AddressMode::ABX, 4),
    Instruct("DEC", Opcode::DEC, AddressMode::ABX, 7),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 7),
    Instruct("CPX", Opcode::CPX, AddressMode::IMM, 2),
    Instruct("SBC", Opcode::SBC, AddressMode::IZX, 6),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 8),
    Instruct("CPX", Opcode::CPX, AddressMode::ZP0, 3),
    Instruct("SBC", Opcode::SBC, AddressMode::ZP0, 3),
    Instruct("INC", Opcode::INC, AddressMode::ZP0, 5),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 5),
    Instruct("INX", Opcode::INX, AddressMode::IMP, 2),
    Instruct("SBC", Opcode::SBC, AddressMode::IMM, 2),
    Instruct("NOP", Opcode::NOP, AddressMode::IMP, 2),
    Instruct("???", Opcode::SBC, AddressMode::IMP, 2),
    Instruct("CPX", Opcode::CPX, AddressMode::ABS, 4),
    Instruct("SBC", Opcode::SBC, AddressMode::ABS, 4),
    Instruct("INC", Opcode::INC, AddressMode::ABS, 6),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 6),
    Instruct("BEQ", Opcode::BEQ, AddressMode::REL, 2),
    Instruct("SBC", Opcode::SBC, AddressMode::IZY, 5),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 8),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 4),
    Instruct("SBC", Opcode::SBC, AddressMode::ZPX, 4),
    Instruct("INC", Opcode::INC, AddressMode::ZPX, 6),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 6),
    Instruct("SED", Opcode::SED, AddressMode::IMP, 2),
    Instruct("SBC", Opcode::SBC, AddressMode::ABY, 4),
    Instruct("NOP", Opcode::NOP, AddressMode::IMP, 2),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 7),
    Instruct("???", Opcode::NOP, AddressMode::IMP, 4),
    Instruct("SBC", Opcode::SBC, AddressMode::ABX, 4),
    Instruct("INC", Opcode::INC, AddressMode::ABX, 7),
    Instruct("???", Opcode::XXX, AddressMode::IMP, 7),
];

pub fn get_instruction_by_id(opcode: u8) -> Instruction {
    let result = &LIST[opcode as usize];

    result.get()
}

impl Cpu6502 {
    pub fn get_instruction(&self) -> Instruction {
        // get_instruction_by_id(self.opcode)
        Instruction::from(self.opcode)
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
        if AddressMode::IMP != self.get_instruction().addres_mode {
            self.fetched = self.read(self.addr_abs);
        }

        self.fetched
    }
}
