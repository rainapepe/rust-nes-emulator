use super::addres_mode::AddresMode;
use super::{Cpu6502, Flags6502};

// Opcodes ======================================================
// There are 56 "legitimate" opcodes provided by the 6502 CPU. I
// have not modelled "unofficial" opcodes. As each opcode is
// defined by 1 byte, there are potentially 256 possible codes.
// Codes are not used in a "switch case" style on a processor,
// instead they are repsonisble for switching individual parts of
// CPU circuits on and off. The opcodes listed here are official,
// meaning that the functionality of the chip when provided with
// these codes is as the developers intended it to be. Unofficial
// codes will of course also influence the CPU circuitry in
// interesting ways, and can be exploited to gain additional
// functionality!
//
// These functions return 0 normally, but some are capable of
// requiring more clock cycles when executed under certain
// conditions combined with certain addressing modes. If that is
// the case, they return 1.
//
// I have included detailed explanations of each function in
// the class implementation file. Note they are listed in
// alphabetical order here for ease of finding.
#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    XXX,
}

impl Cpu6502 {
    pub fn opcode(&mut self, code: Opcode) -> u8 {
        match code {
            Opcode::ADC => self.adc(),
            Opcode::AND => self.and(),
            Opcode::ASL => self.asl(),
            Opcode::BCC => self.bcc(),
            Opcode::BCS => self.bcs(),
            Opcode::BEQ => self.beq(),
            Opcode::BIT => self.bit(),
            Opcode::BMI => self.bmi(),
            Opcode::BNE => self.bne(),
            Opcode::BPL => self.bpl(),
            Opcode::BRK => self.brk(),
            Opcode::BVC => self.bvc(),
            Opcode::BVS => self.bvs(),
            Opcode::CLC => self.clc(),
            Opcode::CLD => self.cld(),
            Opcode::CLI => self.cli(),
            Opcode::CLV => self.clv(),
            Opcode::CMP => self.cmp(),
            Opcode::CPX => self.cpx(),
            Opcode::CPY => self.cpy(),
            Opcode::DEC => self.dec(),
            Opcode::DEX => self.dex(),
            Opcode::DEY => self.dey(),
            Opcode::EOR => self.eor(),
            Opcode::INC => self.inc(),
            Opcode::INX => self.inx(),
            Opcode::INY => self.iny(),
            Opcode::JMP => self.jmp(),
            Opcode::JSR => self.jsr(),
            Opcode::LDA => self.lda(),
            Opcode::LDX => self.ldx(),
            Opcode::LDY => self.ldy(),
            Opcode::LSR => self.lsr(),
            Opcode::NOP => self.nop(),
            Opcode::ORA => self.ora(),
            Opcode::PHA => self.pha(),
            Opcode::PHP => self.php(),
            Opcode::PLA => self.pla(),
            Opcode::PLP => self.plp(),
            Opcode::ROL => self.rol(),
            Opcode::ROR => self.ror(),
            Opcode::RTI => self.rti(),
            Opcode::RTS => self.rts(),
            Opcode::SBC => self.sbc(),
            Opcode::SEC => self.sec(),
            Opcode::SED => self.sed(),
            Opcode::SEI => self.sei(),
            Opcode::STA => self.sta(),
            Opcode::STX => self.stx(),
            Opcode::STY => self.sty(),
            Opcode::TAX => self.tax(),
            Opcode::TAY => self.tay(),
            Opcode::TSX => self.tsx(),
            Opcode::TXA => self.txa(),
            Opcode::TXS => self.txs(),
            Opcode::TYA => self.tya(),
            Opcode::XXX => self.xxx(),
        }
    }

    /** INSTRUCÕES */

    // Adição, vamos adicionar algum valor no acumulador
    // Instruction: Add with Carry In
    // Function:    A = A + M + C
    // A: Acumulador, M: valor que será acumulado, C: flag Carry Bit (emprestimo aritimetico)
    // Flags Out:   C, V, N, Z

    // Calculo de overflow:
    // a negação do XOR entre o acumulador e o dado obtido na memória !(A ^ M) e
    // XOR entre o acumulador e o resultado (A ^ R)
    //
    // V = !(A^M) & (A^R)
    //
    // A  M  R | V | A^R | A^M |~(A^M) |
    // 0  0  0 | 0 |  0  |  0  |   1   |
    // 0  0  1 | 1 |  1  |  0  |   1   |
    // 0  1  0 | 0 |  0  |  1  |   0   |
    // 0  1  1 | 0 |  1  |  1  |   0   |
    // 1  0  0 | 0 |  1  |  1  |   0   |
    // 1  0  1 | 0 |  0  |  1  |   0   |
    // 1  1  0 | 1 |  1  |  0  |   1   |
    // 1  1  1 | 0 |  0  |  0  |   1   |
    fn adc(&mut self) -> u8 {
        // guardando valor que será adicionado no acumulador
        self.fetch();

        // realizando a soma
        self.temp = self.a as u16 + self.fetched as u16 + self.get_flag(Flags6502::C) as u16;

        // A flag Carry é setada se a soma passou do tamanho de 1 bit para informar que ouve um imprestimo
        self.set_flag(Flags6502::C, self.temp > 255);

        // A flag Zero é marcada caso o resultado for zero
        self.set_flag(Flags6502::Z, (self.temp & 0x00FF) == 0);

        // V = !(A^M) & (A^R)
        let overflow = (!(self.a ^ self.fetched) as u16 & (self.a as u16 ^ self.temp)) & 0x0080;
        self.set_flag(Flags6502::V, overflow > 0);

        // Flag negativa, se o primeiro bit for verdadeiro então o resultado é negativo (0x80 = 128 = 10000000)
        self.set_flag(Flags6502::N, (self.temp & 0x80) > 0);

        // Salvar o resultado no acumulador
        self.a = (self.temp & 0x00FF) as u8;

        1
    }

    // Instruction: Subtraction with Borrow In
    // Function:    A = A - M - (1 - C)
    // Flags Out:   C, V, N, Z
    //
    // Explanation:
    // Given the explanation for ADC above, we can reorganise our data
    // to use the same computation for addition, for subtraction by multiplying
    // the data by -1, i.e. make it negative
    fn sbc(&mut self) -> u8 {
        // guardando valor que será adicionado no acumulador
        self.fetch();

        // valor invertido
        let value = self.fetched as u16 ^ 0x00FF;

        // realizando a subtração (somar com o valor invertido para subtrair)
        self.temp = self.a as u16 + value + self.get_flag(Flags6502::C) as u16;

        // A flag Carry é setada se a soma passou do tamanho de 1 bit para informar que ouve um imprestimo
        self.set_flag(Flags6502::C, (self.temp & 0xFF00) > 0);

        // A flag Zero é marcada caso o resultado for zero
        self.set_flag(Flags6502::Z, (self.temp & 0x00FF) == 0);

        // V = !(A^M) & (A^R)
        let overflow = (self.temp ^ self.a as u16) & (self.temp ^ value) & 0x0080;
        self.set_flag(Flags6502::V, overflow > 0);

        // Flag negativa, se o primeiro bit for verdadeiro então o resultado é negativo (0x80 = 128 = 10000000)
        self.set_flag(Flags6502::N, (self.temp & 0x80) > 0);

        // Salvar o resultado no acumulador
        self.a = (self.temp & 0x00FF) as u8;

        1
    }

    // Instruction: Bitwise Logic AND
    // Function:    A = A & M
    // Flags Out:   N, Z
    fn and(&mut self) -> u8 {
        self.fetch();

        self.a = self.a & self.fetched;
        self.set_flag(Flags6502::Z, self.a == 0);
        self.set_flag(Flags6502::N, (self.a & 0x80) > 0);

        1
    }

    // Instruction: Arithmetic Shift Left
    // Function:    A = C <- (A << 1) <- 0
    // Flags Out:   N, Z, C
    fn asl(&mut self) -> u8 {
        self.fetch();

        self.temp = (self.fetched << 1) as u16;
        self.set_flag(Flags6502::C, (self.temp & 0xFF00) > 0);
        self.set_flag(Flags6502::Z, self.temp == 0);
        self.set_flag(Flags6502::N, (self.temp & 0x80) > 0);

        if let AddresMode::IMP = self.get_instruction().addres_mode {
            self.a = (self.temp & 0x00FF) as u8;
        } else {
            self.write(self.addr_abs, (self.temp & 0x00FF) as u8);
        }

        1
    }

    // Instruction: Branch if Carry Clear
    // Function:    if(C == 0) pc = address
    fn bcc(&mut self) -> u8 {
        if self.get_flag(Flags6502::C) == 0 {
            self.pc_branch();
        }

        0
    }

    // Instruction: Branch if Carry Set
    // Function:    if(C == 1) pc = address
    fn bcs(&mut self) -> u8 {
        if self.get_flag(Flags6502::C) == 1 {
            self.pc_branch();
        }

        0
    }

    // Instruction: Branch if Equal
    // Function:    if(Z == 1) pc = address
    fn beq(&mut self) -> u8 {
        if self.get_flag(Flags6502::Z) == 1 {
            self.pc_branch();
        }

        0
    }

    fn bit(&mut self) -> u8 {
        self.fetch();

        self.temp = (self.a & self.fetched) as u16;
        self.set_flag(Flags6502::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(Flags6502::N, (self.fetched & (1 << 7)) > 0);
        self.set_flag(Flags6502::V, (self.fetched & (1 << 6)) > 0);

        0
    }

    // Instruction: Branch if Negative
    // Function:    if(N == 1) pc = address
    fn bmi(&mut self) -> u8 {
        if self.get_flag(Flags6502::N) == 1 {
            self.pc_branch();
        }

        0
    }

    // Instruction: Branch if Not Equal
    // Function:    if(Z == 0) pc = address
    fn bne(&mut self) -> u8 {
        if self.get_flag(Flags6502::Z) == 0 {
            self.pc_branch();
        }

        0
    }

    // Instruction: Branch if Positive
    // Function:    if(N == 0) pc = address
    fn bpl(&mut self) -> u8 {
        if self.get_flag(Flags6502::N) == 0 {
            self.pc_branch();
        }

        0
    }

    // Instruction: Break
    // Function: Program Sourced Interrupt
    fn brk(&mut self) -> u8 {
        self.pc_next();

        self.set_flag(Flags6502::I, true);
        self.stkp_push(((self.pc >> 8) & 0x00FF) as u8);
        self.stkp_push((self.pc & 0x00FF) as u8);

        self.set_flag(Flags6502::B, true);
        self.stkp_push(self.status);
        self.set_flag(Flags6502::B, false);

        self.pc = self.read_16b(0xFFFE);

        println!("brk: {}", self.pc);

        0
    }

    // Instruction: Branch if Overflow Clear
    // Function:    if(V == 0) pc = address
    fn bvc(&mut self) -> u8 {
        if self.get_flag(Flags6502::V) == 0 {
            self.pc_branch();
        }

        0
    }

    // Instruction: Branch if Overflow Set
    // Function:    if(V == 1) pc = address
    fn bvs(&mut self) -> u8 {
        if self.get_flag(Flags6502::V) == 1 {
            self.pc_branch();
        }

        0
    }

    // Instruction: Clear Carry Flag
    // Function:    C = 0
    fn clc(&mut self) -> u8 {
        self.set_flag(Flags6502::C, false);
        0
    }

    // Instruction: Clear Decimal Flag
    // Function:    D = 0
    fn cld(&mut self) -> u8 {
        self.set_flag(Flags6502::D, false);
        0
    }

    // Instruction: Disable Interrupts / Clear Interrupt Flag
    // Function:    I = 0
    fn cli(&mut self) -> u8 {
        self.set_flag(Flags6502::I, false);
        0
    }

    // Instruction: Clear Overflow Flag
    // Function:    V = 0
    fn clv(&mut self) -> u8 {
        self.set_flag(Flags6502::V, false);
        0
    }

    // Instruction: Compare Accumulator
    // Function:    C <- A >= M      Z <- (A - M) == 0
    // Flags Out:   N, C, Z
    fn cmp(&mut self) -> u8 {
        self.fetch();

        self.temp = (self.a - self.fetched) as u16;
        self.set_flag(Flags6502::C, self.a >= self.fetched);
        self.set_flag(Flags6502::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(Flags6502::N, (self.temp & 0x0080) > 0);

        1
    }

    // Instruction: Compare X Register
    // Function:    C <- X >= M      Z <- (X - M) == 0
    // Flags Out:   N, C, Z
    fn cpx(&mut self) -> u8 {
        self.fetch();

        self.temp = (self.x - self.fetched) as u16;
        self.set_flag(Flags6502::C, self.x >= self.fetched);
        self.set_flag(Flags6502::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(Flags6502::N, (self.temp & 0x0080) > 0);

        0
    }

    // Instruction: Compare Y Register
    // Function:    C <- Y >= M      Z <- (Y - M) == 0
    // Flags Out:   N, C, Z
    fn cpy(&mut self) -> u8 {
        self.fetch();

        self.temp = (self.y - self.fetched) as u16;
        self.set_flag(Flags6502::C, self.y >= self.fetched);
        self.set_flag(Flags6502::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(Flags6502::N, (self.temp & 0x0080) > 0);

        0
    }

    // Instruction: Decrement Value at Memory Location
    // Function:    M = M - 1
    // Flags Out:   N, Z
    fn dec(&mut self) -> u8 {
        self.fetch();

        self.temp = (self.fetched - 1) as u16;
        self.write(self.addr_abs, (self.temp & 0x00FF) as u8);
        self.set_flag(Flags6502::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(Flags6502::N, (self.temp & 0x0080) > 0);

        0
    }

    // Instruction: Decrement X Register
    // Function:    X = X - 1
    // Flags Out:   N, Z
    fn dex(&mut self) -> u8 {
        self.x -= 1;

        self.set_flag(Flags6502::Z, self.x == 0);
        self.set_flag(Flags6502::N, (self.x & 0x80) > 0);

        0
    }

    // Instruction: Decrement Y Register
    // Function:    Y = Y - 1
    // Flags Out:   N, Z
    fn dey(&mut self) -> u8 {
        self.y -= 1;

        self.set_flag(Flags6502::Z, self.y == 0);
        self.set_flag(Flags6502::N, (self.y & 0x80) > 0);

        0
    }

    // Instruction: Bitwise Logic XOR
    // Function:    A = A xor M
    // Flags Out:   N, Z
    fn eor(&mut self) -> u8 {
        self.fetch();

        self.a = self.a ^ self.fetched;

        self.set_flag(Flags6502::Z, self.a == 0);
        self.set_flag(Flags6502::N, (self.a & 0x80) > 0);

        1
    }

    // Instruction: Increment Value at Memory Location
    // Function:    M = M + 1
    // Flags Out:   N, Z
    fn inc(&mut self) -> u8 {
        self.fetch();

        self.temp = (self.fetched + 1) as u16;
        self.write(self.addr_abs, (self.temp & 0x00FF) as u8);

        self.set_flag(Flags6502::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(Flags6502::N, (self.temp & 0x0080) > 0);

        0
    }

    // Instruction: Increment X Register
    // Function:    X = X + 1
    // Flags Out:   N, Z
    fn inx(&mut self) -> u8 {
        self.x += 1;

        self.set_flag(Flags6502::Z, self.x == 0);
        self.set_flag(Flags6502::N, (self.x & 0x80) > 0);

        0
    }

    // Instruction: Increment Y Register
    // Function:    Y = Y + 1
    // Flags Out:   N, Z
    fn iny(&mut self) -> u8 {
        self.y += 1;

        self.set_flag(Flags6502::Z, self.y == 0);
        self.set_flag(Flags6502::N, (self.y & 0x80) > 0);

        0
    }

    // Instruction: Jump To Location
    // Function:    pc = address
    fn jmp(&mut self) -> u8 {
        self.pc = self.addr_abs;
        0
    }

    // Instruction: Jump To Sub-Routine
    // Function:    Push current pc to stack, pc = address
    fn jsr(&mut self) -> u8 {
        self.pc -= 1;

        self.stkp_push(((self.pc >> 8) & 0x00FF) as u8);
        self.stkp_push((self.pc & 0x00FF) as u8);

        self.pc = self.addr_abs;

        0
    }

    // Instruction: Load The Accumulator
    // Function:    A = M
    // Flags Out:   N, Z
    fn lda(&mut self) -> u8 {
        self.fetch();
        self.a = self.fetched;

        self.set_flag(Flags6502::Z, self.a == 0);
        self.set_flag(Flags6502::N, (self.a & 0x80) > 0);

        1
    }

    // Instruction: Load The X Register
    // Function:    X = M
    // Flags Out:   N, Z
    fn ldx(&mut self) -> u8 {
        self.fetch();
        self.x = self.fetched;

        self.set_flag(Flags6502::Z, self.x == 0);
        self.set_flag(Flags6502::N, (self.x & 0x80) > 0);

        1
    }

    // Instruction: Load The Y Register
    // Function:    Y = M
    // Flags Out:   N, Z
    fn ldy(&mut self) -> u8 {
        self.fetch();
        self.y = self.fetched;

        self.set_flag(Flags6502::Z, self.y == 0);
        self.set_flag(Flags6502::N, (self.y & 0x80) > 0);

        1
    }

    fn lsr(&mut self) -> u8 {
        self.fetch();

        self.set_flag(Flags6502::C, (self.fetched & 0x0001) > 0);
        self.temp = (self.fetched >> 1) as u16;

        self.set_flag(Flags6502::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(Flags6502::N, (self.temp & 0x80) > 0);

        if let AddresMode::IMP = self.get_instruction().addres_mode {
            self.a = (self.temp & 0x00FF) as u8;
        } else {
            self.write(self.addr_abs, (self.temp & 0x00FF) as u8);
        }

        0
    }

    fn nop(&mut self) -> u8 {
        match self.opcode {
            0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => 1,
            _ => 0,
        }
    }

    // Instruction: Bitwise Logic OR
    // Function:    A = A | M
    // Flags Out:   N, Z
    fn ora(&mut self) -> u8 {
        self.fetch();

        self.a = self.a | self.fetched;

        self.set_flag(Flags6502::Z, self.a == 0);
        self.set_flag(Flags6502::N, (self.a & 0x80) > 0);

        1
    }

    // Instruction: Push Accumulator to Stack
    // Function:    A -> stack
    fn pha(&mut self) -> u8 {
        self.write(0x0100 + self.stkp as u16, self.a);
        self.stkp_push(self.a);

        0
    }

    // Instruction: Push Status Register to Stack
    // Function:    status -> stack
    // Note:        Break flag is set to 1 before push
    fn php(&mut self) -> u8 {
        self.stkp_push(self.status | Flags6502::B as u8 | Flags6502::U as u8);
        self.set_flag(Flags6502::B, false);
        self.set_flag(Flags6502::U, false);

        0
    }

    // Instruction: Pop Accumulator off Stack
    // Function:    A <- stack
    // Flags Out:   N, Z
    fn pla(&mut self) -> u8 {
        self.a = self.stkp_pop();

        self.set_flag(Flags6502::Z, self.a == 0);
        self.set_flag(Flags6502::N, (self.a & 0x80) > 0);

        0
    }

    // Instruction: Pop Status Register off Stack
    // Function:    Status <- stack
    fn plp(&mut self) -> u8 {
        self.status = self.stkp_pop();
        self.set_flag(Flags6502::U, true);

        0
    }

    fn rol(&mut self) -> u8 {
        self.fetch();

        self.temp = (self.fetched << 1) as u16 | self.get_flag(Flags6502::C) as u16;

        self.set_flag(Flags6502::C, (self.temp & 0xFF00) > 0);
        self.set_flag(Flags6502::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(Flags6502::N, (self.temp & 0x0080) > 0);

        if let AddresMode::IMP = self.get_instruction().addres_mode {
            self.a = (self.temp & 0x00FF) as u8;
        } else {
            self.write(self.addr_abs, (self.temp & 0x00FF) as u8);
        }

        0
    }

    fn ror(&mut self) -> u8 {
        self.fetch();

        self.temp = (self.fetched >> 1) as u16 | ((self.get_flag(Flags6502::C) as u16) << 7);

        self.set_flag(Flags6502::C, (self.fetched & 0x01) > 0);
        self.set_flag(Flags6502::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(Flags6502::N, (self.temp & 0x0080) > 0);

        if let AddresMode::IMP = self.get_instruction().addres_mode {
            self.a = (self.temp & 0x00FF) as u8;
        } else {
            self.write(self.addr_abs, (self.temp & 0x00FF) as u8);
        }

        0
    }

    fn rti(&mut self) -> u8 {
        self.status = self.stkp_pop();
        self.status &= !(Flags6502::B as u8);
        self.status &= !(Flags6502::U as u8);

        self.pc = self.stkp_pop() as u16;
        self.pc |= (self.stkp_pop() as u16) << 8;

        0
    }

    fn rts(&mut self) -> u8 {
        self.pc = self.stkp_pop() as u16;
        self.pc |= (self.stkp_pop() as u16) << 8;

        self.pc_next();

        0
    }

    // Instruction: Set Carry Flag
    // Function:    C = 1
    fn sec(&mut self) -> u8 {
        self.set_flag(Flags6502::C, true);

        0
    }

    // Instruction: Set Decimal Flag
    // Function:    D = 1
    fn sed(&mut self) -> u8 {
        self.set_flag(Flags6502::D, true);

        0
    }

    // Instruction: Set Interrupt Flag / Enable Interrupts
    // Function:    I = 1
    fn sei(&mut self) -> u8 {
        self.set_flag(Flags6502::I, true);

        0
    }

    // Instruction: Store Accumulator at Address
    // Function:    M = A
    fn sta(&mut self) -> u8 {
        self.write(self.addr_abs, self.a);

        0
    }

    // Instruction: Store X Register at Address
    // Function:    M = X
    fn stx(&mut self) -> u8 {
        self.write(self.addr_abs, self.x);

        0
    }

    // Instruction: Store Y Register at Address
    // Function:    M = Y
    fn sty(&mut self) -> u8 {
        self.write(self.addr_abs, self.y);

        0
    }

    // Instruction: Transfer Accumulator to X Register
    // Function:    X = A
    // Flags Out:   N, Z
    fn tax(&mut self) -> u8 {
        self.x = self.a;

        self.set_flag(Flags6502::Z, self.x == 0);
        self.set_flag(Flags6502::N, (self.x & 0x80) > 0);

        0
    }

    // Instruction: Transfer Accumulator to Y Register
    // Function:    Y = A
    // Flags Out:   N, Z
    fn tay(&mut self) -> u8 {
        self.y = self.a;

        self.set_flag(Flags6502::Z, self.y == 0);
        self.set_flag(Flags6502::N, (self.y & 0x80) > 0);

        0
    }

    // Instruction: Transfer Stack Pointer to X Register
    // Function:    X = stack pointer
    // Flags Out:   N, Z
    fn tsx(&mut self) -> u8 {
        self.x = self.stkp;

        self.set_flag(Flags6502::Z, self.x == 0);
        self.set_flag(Flags6502::N, (self.x & 0x80) > 0);

        0
    }

    // Instruction: Transfer X Register to Accumulator
    // Function:    A = X
    // Flags Out:   N, Z
    fn txa(&mut self) -> u8 {
        self.a = self.x;

        self.set_flag(Flags6502::Z, self.a == 0);
        self.set_flag(Flags6502::N, (self.a & 0x80) > 0);

        0
    }

    // Instruction: Transfer X Register to Stack Pointer
    // Function:    stack pointer = X
    fn txs(&mut self) -> u8 {
        self.stkp = self.x;
        0
    }

    // Instruction: Transfer Y Register to Accumulator
    // Function:    A = Y
    // Flags Out:   N, Z
    fn tya(&mut self) -> u8 {
        self.a = self.y;

        self.set_flag(Flags6502::Z, self.a == 0);
        self.set_flag(Flags6502::N, (self.a & 0x80) > 0);

        0
    }

    // This function captures illegal opcodes
    fn xxx(&mut self) -> u8 {
        0
    }
}
