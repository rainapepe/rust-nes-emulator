use super::addres_mode::AddressMode;
use super::instruction::Instruction;
use super::Cpu6502;

use std::collections::HashMap;

const HEX_TABLE: &str = "0123456789ABCDEF";

fn to_hex(value: u32, hex_size: u8) -> String {
    let mut temp_value = value;
    let mut text = Vec::<u8>::new();

    // inicia o array com o tamanho do hex_size com zeros
    text.resize(hex_size as usize, 0);

    // começar do ultimo caracter para o primeiro
    for i in (0..hex_size).rev() {
        // identifica o caracter hex correpondente aos ultimos 4 bits
        let c = HEX_TABLE.as_bytes()[(temp_value & 0xF) as usize];
        text[i as usize] = c;

        // mover 4 bits para a direita para poder achar o valor do proximos 4 bits
        temp_value >>= 4;
    }

    String::from_utf8(text).unwrap()
}

impl Cpu6502 {
    pub fn complete(&self) -> bool {
        self.cycles == 0
    }

    pub fn disassemble_instruction(&mut self) -> String {
        let mut addr = self.pc as u32;
        let mut value: u8 = 0;
        let mut lo: u8 = 0;
        let mut hi: u8 = 0;

        let mut instruction_line = format!("${}: ", to_hex(addr, 4));
        let opcode = self.read(addr as u16);
        addr += 1;
        let instruction = Instruction::from(opcode);
        instruction_line += &format!("{} ", instruction.name);

        match instruction.addres_mode {
            AddressMode::IMP => {
                instruction_line += " {IMP}";
            }
            AddressMode::IMM => {
                value = self.bus_read(addr as u16, true);
                instruction_line += &format!("#${} {{IMM}}", to_hex(value as u32, 2));
            }
            AddressMode::ZP0 => {
                lo = self.bus_read(addr as u16, true);
                instruction_line += &format!("${} {{ZP0}}", to_hex(lo as u32, 2));
            }
            AddressMode::ZPX => {
                lo = self.bus_read(addr as u16, true);
                instruction_line += &format!("${}, X {{ZPX}}", to_hex(lo as u32, 2));
            }
            AddressMode::ZPY => {
                lo = self.bus_read(addr as u16, true);
                instruction_line += &format!("${}, Y {{ZPY}}", to_hex(lo as u32, 2));
            }
            AddressMode::IZX => {
                lo = self.bus_read(addr as u16, true);
                instruction_line += &format!("${}, X {{IZX}}", to_hex(lo as u32, 2));
            }
            AddressMode::IZY => {
                lo = self.bus_read(addr as u16, true);
                instruction_line += &format!("${}, Y {{IZY}}", to_hex(lo as u32, 2));
            }
            AddressMode::ABS => {
                lo = self.bus_read(addr as u16, true);
                addr += 1;
                hi = self.bus_read(addr as u16, true);
                instruction_line +=
                    &format!("${} {{ABS}}", to_hex(((hi as u32) << 8) | lo as u32, 4));
            }
            AddressMode::ABX => {
                lo = self.bus_read(addr as u16, true);
                addr += 1;
                hi = self.bus_read(addr as u16, true);
                instruction_line +=
                    &format!("${}, X {{ABX}}", to_hex(((hi as u32) << 8) | lo as u32, 2));
            }
            AddressMode::ABY => {
                lo = self.bus_read(addr as u16, true);
                addr += 1;
                hi = self.bus_read(addr as u16, true);
                instruction_line +=
                    &format!("${}, Y {{ABY}}", to_hex(((hi as u32) << 8) | lo as u32, 2));
            }
            AddressMode::IND => {
                lo = self.bus_read(addr as u16, true);
                addr += 1;
                hi = self.bus_read(addr as u16, true);
                instruction_line +=
                    &format!("(${}) {{IND}}", to_hex(((hi as u32) << 8) | lo as u32, 2));
            }
            AddressMode::REL => {
                value = self.bus_read(addr as u16, true);
                addr += 1;
                instruction_line += &format!(
                    "${} [${}] {{REL}}",
                    to_hex(value as u32, 2),
                    to_hex(addr + value as u32, 4)
                );
            }
            _ => {}
        };

        return instruction_line;
    }

    // This is the disassembly function. Its workings are not required for emulation.
    // It is merely a convenience function to turn the binary instruction code into
    // human readable form. Its included as part of the emulator because it can take
    // advantage of many of the CPUs internal operations to do this.
    pub fn disassemble(&mut self, start: u16, stop: u16) -> HashMap<u16, String> {
        let mut addr: u32 = start as u32;
        let mut value: u8 = 0;
        let mut lo: u8 = 0;
        let mut hi: u8 = 0;

        // criar map
        let mut map_lines = HashMap::<u16, String>::new();
        let mut line_addr: u16 = 0;

        while addr <= stop as u32 {
            line_addr = addr as u16;

            let mut s_inst = format!("${}: ", to_hex(addr, 4));

            let opcode = self.read(addr as u16);
            addr += 1;
            let instruction = Instruction::from(opcode);
            s_inst += &format!("{} ", instruction.name);

            match instruction.addres_mode {
                AddressMode::IMP => {
                    s_inst += " {IMP}";
                }
                AddressMode::IMM => {
                    value = self.bus_read(addr as u16, true);
                    addr += 1;
                    s_inst += &format!("#${} {{IMM}}", to_hex(value as u32, 2));
                }
                AddressMode::ZP0 => {
                    lo = self.bus_read(addr as u16, true);
                    addr += 1;
                    hi = 0x00;
                    s_inst += &format!("${} {{ZP0}}", to_hex(lo as u32, 2));
                }
                AddressMode::ZPX => {
                    lo = self.bus_read(addr as u16, true);
                    addr += 1;
                    hi = 0x00;
                    s_inst += &format!("${}, X {{ZPX}}", to_hex(lo as u32, 2));
                }
                AddressMode::ZPY => {
                    lo = self.bus_read(addr as u16, true);
                    addr += 1;
                    hi = 0x00;
                    s_inst += &format!("${}, Y {{ZPY}}", to_hex(lo as u32, 2));
                }
                AddressMode::IZX => {
                    lo = self.bus_read(addr as u16, true);
                    addr += 1;
                    hi = 0x00;
                    s_inst += &format!("${}, X {{IZX}}", to_hex(lo as u32, 2));
                }
                AddressMode::IZY => {
                    lo = self.bus_read(addr as u16, true);
                    addr += 1;
                    hi = 0x00;
                    s_inst += &format!("${}, Y {{IZY}}", to_hex(lo as u32, 2));
                }
                AddressMode::ABS => {
                    lo = self.bus_read(addr as u16, true);
                    addr += 1;
                    hi = self.bus_read(addr as u16, true);
                    addr += 1;
                    s_inst += &format!("${} {{ABS}}", to_hex(((hi as u32) << 8) | lo as u32, 2));
                }
                AddressMode::ABX => {
                    lo = self.bus_read(addr as u16, true);
                    addr += 1;
                    hi = self.bus_read(addr as u16, true);
                    addr += 1;
                    s_inst += &format!("${}, X {{ABX}}", to_hex(((hi as u32) << 8) | lo as u32, 2));
                }
                AddressMode::ABY => {
                    lo = self.bus_read(addr as u16, true);
                    addr += 1;
                    hi = self.bus_read(addr as u16, true);
                    addr += 1;
                    s_inst += &format!("${}, Y {{ABY}}", to_hex(((hi as u32) << 8) | lo as u32, 2));
                }
                AddressMode::IND => {
                    lo = self.bus_read(addr as u16, true);
                    addr += 1;
                    hi = self.bus_read(addr as u16, true);
                    addr += 1;
                    s_inst += &format!("(${}) {{IND}}", to_hex(((hi as u32) << 8) | lo as u32, 2));
                }
                AddressMode::REL => {
                    value = self.bus_read(addr as u16, true);
                    addr += 1;
                    s_inst += &format!(
                        "${} [${}] {{REL}}",
                        to_hex(value as u32, 2),
                        to_hex(addr + value as u32, 4)
                    );
                }
                _ => {}
            };

            map_lines.insert(line_addr, s_inst);
        }

        map_lines
    }
}
