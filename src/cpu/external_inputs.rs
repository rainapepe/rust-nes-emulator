use super::{Cpu6502, Flags6502};

// Funções externar, no hardware eles são representados como pinos que produzem alguma alteração no estado
impl Cpu6502 {
    /** Forces the 6502 into a known state. This is hard-wired inside the CPU. The
    registers are set to 0x00, the status register is cleared except for unused
    bit which remains at 1. An absolute address is read from location 0xFFFC
    which contains a second address that the program counter is set to. This
    allows the programmer to jump to a known and programmable location in the
    memory to start executing from. Typically the programmer would set the value
    at location 0xFFFC at compile time.
    */
    pub fn reset(&mut self) {
        // Get address to set program counter to
        self.addr_abs = 0xFFFC;
        let lo = self.read(self.addr_abs + 0) as u16;
        let hi = self.read(self.addr_abs + 1) as u16;

        self.pc = (hi << 0) | lo;

        // limpar registradores
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.stkp = 0;
        self.status = 0 | Flags6502::U as u8;

        // limpar variaveis auxiliares
        self.addr_rel = 0;
        self.addr_abs = 0;
        self.fetched = 0;

        self.cycles = 8;
    }

    /** Interrupt requests are a complex operation and only happen if the
    "disable interrupt" flag is 0. IRQs can happen at any time, but
    you dont want them to be destructive to the operation of the running
    program. Therefore the current instruction is allowed to finish
    (which I facilitate by doing the whole thing when cycles == 0) and
    then the current program counter is stored on the stack. Then the
    current status register is stored on the stack. When the routine
    that services the interrupt has finished, the status register
    and program counter can be restored to how they where before it
    occurred. This is impemented by the "RTI" instruction. Once the IRQ
    has happened, in a similar way to a reset, a programmable address
    is read form hard coded location 0xFFFE, which is subsequently
    set to the program counter.
    */
    pub fn irq(&mut self) {
        if self.get_flag(Flags6502::I) == 0 {
            // Vamos adicionar o pc (program counter) a stack, como o pc é 16 bits vamos ter que dividir em 2 e adicionar as duas partes
            self.write(0x0100 + self.stkp as u16, ((self.pc >> 8) & 0x00FF) as u8);
            self.stkp -= 1;
            self.write(0x0100 + self.stkp as u16, (self.pc & 0x00FF) as u8);
            self.stkp -= 1;

            // Adicionando o status na stack
            self.set_flag(Flags6502::B, false);
            self.set_flag(Flags6502::U, true);
            self.set_flag(Flags6502::I, true);
            self.write(0x0100 + self.stkp as u16, self.status);
            self.stkp -= 1;

            // Ler novo pc (program counter) a partir de um endereço fixo
            self.addr_abs = 0xFFFE;
            let lo = self.read(self.addr_abs + 0);
            let hi = self.read(self.addr_abs + 1);
            self.pc = ((hi << 8) | lo) as u16;

            self.cycles = 7;
        }
    }

    /** A Non-Maskable Interrupt cannot be ignored. It behaves in exactly the
    same way as a regular IRQ, but reads the new program counter address
    form location 0xFFFA.
        */
    pub fn nmi(&mut self) {
        self.write(0x0100 + self.stkp as u16, ((self.pc >> 8) & 0x00FF) as u8);
        self.stkp -= 1;
        self.write(0x0100 + self.stkp as u16, (self.pc & 0x00FF) as u8);
        self.stkp -= 1;

        // Adicionando o status na stack
        self.set_flag(Flags6502::B, false);
        self.set_flag(Flags6502::U, true);
        self.set_flag(Flags6502::I, true);
        self.write(0x0100 + self.stkp as u16, self.status);
        self.stkp -= 1;

        // Ler novo pc (program counter) a partir de um endereço fixo
        self.addr_abs = 0xFFFA;
        let lo = self.read(self.addr_abs + 0);
        let hi = self.read(self.addr_abs + 1);
        self.pc = ((hi << 8) | lo) as u16;

        self.cycles = 8;
    }

    /** Perform one clock cycles worth of emulation */
    pub fn clock(&mut self) {
        // Each instruction requires a variable number of clock cycles to execute.
        // In my emulation, I only care about the final result and so I perform
        // the entire computation in one hit. In hardware, each clock cycle would
        // perform "microcode" style transformations of the CPUs state.
        //
        // To remain compliant with connected devices, it's important that the
        // emulation also takes "time" in order to execute instructions, so I
        // implement that delay by simply counting down the cycles required by
        // the instruction. When it reaches 0, the instruction is complete, and
        // the next one is ready to be executed.
        if self.cycles == 0 {
            // Ler o próximo byte de instrução, o valor desse Byte é para achar
            // qual é a operação e addresmode na tabela de tradução
            self.opcode = self.read(self.pc);

            self.set_flag(Flags6502::U, true);

            self.pc += 1;
        }
    }
}
