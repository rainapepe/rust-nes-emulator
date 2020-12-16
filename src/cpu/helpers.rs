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
fn to_hex(n: u32, d: u8) -> String {
    let test = String::with_capacity(d as usize);
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
    }
}
