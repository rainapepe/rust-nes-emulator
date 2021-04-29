use crate::bus::Bus;

// O registrador de status armazena 8 flags, para facilitar o acesso foi criado um enum para cada flag
pub enum Flags6502 {
    /** Carry Bit */
    C = 1 << 0,
    /** Zero */
    Z = 1 << 1,
    /** Disable Interrups */
    I = 1 << 2,
    /** Decimal Mode */
    D = 1 << 3,
    /** Break */
    B = 1 << 4,
    /** Unused */
    U = 1 << 5,
    /** Overflow */
    V = 1 << 6,
    /** Negative */
    N = 1 << 7,
}

pub struct Cpu6502 {
    // CPU registradores
    /** Registro Acumulador */
    pub a: u8,
    /** Registro X */
    pub x: u8,
    /** Registro Y */
    pub y: u8,
    /** Stack Pointer (aponta para um local na bus(barramento)) */
    pub stkp: u8,
    /** Program Counter (registrador que armazena a posição atual da sequencia de execução) */
    pub pc: u16,
    /** Registro de status */
    pub status: u8,

    // variaveis auxiliares para facilitar a emulação
    /** Represents the working input value to the ALU */
    pub fetched: u8,
    /** A convenience variable used everywhere */
    pub temp: u16,
    /** All used memory addresses end up in here */
    pub addr_abs: u16,
    /** Represents absolute address following a branch */
    pub addr_rel: u16,
    /** Is the instruction byte */
    pub opcode: u8,
    /** Counts how many cycles the instruction has remaining */
    pub cycles: u8,
    /** A global accumulation of the number of clocks */
    pub clock_count: u32,
    pub bus: Bus,
}

impl Cpu6502 {
    pub fn new_with_bus(bus: Bus) -> Cpu6502 {
        Cpu6502 {
            // registradores
            a: 0,
            x: 0,
            y: 0,
            stkp: 0,
            pc: 0,
            status: 0,

            // variaveis auxiliares
            fetched: 0,
            temp: 0,
            addr_abs: 0,
            addr_rel: 0,
            opcode: 0,
            cycles: 0,
            clock_count: 0,
            bus,
        }
    }
}

// Conectividade com a Bus
impl Cpu6502 {
    pub fn read(&mut self, addres: u16) -> u8 {
        return self.bus.read(addres, false);
    }

    pub fn write(&mut self, addres: u16, data: u8) {
        self.bus.write(addres, data);
    }

    pub fn bus_read(&mut self, addres: u16, read_only: bool) -> u8 {
        return self.bus.read(addres, read_only);
    }
}

// Funções para manipular flags
impl Cpu6502 {
    pub fn get_flag(&self, flag: Flags6502) -> u8 {
        // Ao utilizar o bitwise AND do status com a flag representante o resultado deve
        // ser o próprio valor da flag se o status for verdadeiro
        // Exemplos:
        //  255(status) & 128(flag) = 128
        //  254(status) & 1(flag) = 0 nesse caso o bit refente a 1 está desligado
        //  1(status) & 1(flag) = 1
        if (self.status & flag as u8) > 0 {
            return 1;
        }

        0
    }

    pub fn set_flag(&mut self, flag: Flags6502, value: bool) {
        if value {
            self.status |= flag as u8;
        } else {
            self.status &= flag as u8;
        }
    }
}

// Funções auxiliares
impl Cpu6502 {
    pub fn pc_next(&mut self) -> u16 {
        self.pc += 1;
        self.pc
    }

    pub fn pc_back(&mut self) -> u16 {
        self.pc -= 1;
        self.pc
    }

    pub fn stkp_push(&mut self, value: u8) {
        self.write(0x0100 + self.stkp as u16, value);
        if self.stkp == 0 {
            self.stkp = 255;
        } else {
            self.stkp -= 1;
        }
    }

    pub fn stkp_pop(&mut self) -> u8 {
        if self.stkp == 255 {
            self.stkp = 0;
        } else {
            self.stkp += 1;
        }
        self.read(0x0100 + self.stkp as u16)
    }

    pub fn pc_branch(&mut self) {
        self.cycles += 1;
        self.addr_abs = self.pc + self.addr_rel;

        if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
            self.cycles += 1;
        }

        self.pc = self.addr_abs;
    }

    pub fn read_16b(&mut self, addres: u16) -> u16 {
        let lo = self.read(addres) as u16;
        let hi = self.read(addres + 1) as u16;

        // println!("hi: {:#06x} lo: {:#06x}", hi, lo);

        (hi << 8) | lo
    }

    pub fn read_next_16b(&mut self) -> u16 {
        let lo = self.read(self.pc) as u16;
        self.pc_next();
        let hi = self.read(self.pc) as u16;
        self.pc_next();

        (hi << 8) | lo
    }
}
