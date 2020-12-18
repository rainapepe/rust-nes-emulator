use super::addres_mode::AddresMode;
use super::opcode::Opcode;
use super::Cpu6502;

// A convenient utility to convert variables into
// hex strings because "modern C++"'s method with
// streams is atrocious
// auto hex = [](uint32_t n, uint8_t d)
// {
//     std::string s(d, '0');
//     for (int i = d - 1; i >= 0; i--, n >>= 4)
//         s[i] = "0123456789ABCDEF"[n & 0xF];
//     return s;
// };

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

    // This is the disassembly function. Its workings are not required for emulation.
    // It is merely a convenience function to turn the binary instruction code into
    // human readable form. Its included as part of the emulator because it can take
    // advantage of many of the CPUs internal operations to do this.
    pub fn disassemble(start: u16, stop: u16) {
        let addr: u32 = start as u32;
        let value: u8 = 0;
        let lo: u8 = 0;
        let hi: u8 = 0;

        // let mapLines
        let mut line_addr: u16 = 0;

        while addr <= stop as u32 {
            line_addr = addr as u16;

            let s_inst = format!("${}: ", to_hex(addr, 4));
        }
    }
}
