use super::Cpu6502;

// ADDRESSING MODES - Modos de endereçamento

// Modo de endereçamento é utilizado para carregar os argumentos da instrução.
// Toda instrução é composta de um opcode e um modo de endereçamento, e assim com o modo de endereçamento
// podemos carregar os argumentos nessesários para executar a instrução.

// O processador 6502 pode acessar entre o endereço 0x0000 e 0xFFFF, onde o Byte alto é referente a página na memória
// e assim temos 256 páginas com 256 bytes cada página.

// Alguns modos de endereçamento precisam de mais ciclos e para isso retornamos os ciclos adicionais necessários para cada modo

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum AddressMode {
    /** Address Mode: Implied */
    IMP,
    /** Address Mode: Immediate */
    IMM,
    /** Address Mode: Zero Page */
    ZP0,
    /** Address Mode: Zero Page with X Offset */
    ZPX,
    /** Address Mode: Zero Page with Y Offset */
    ZPY,
    /** Address Mode: Relative */
    REL,
    /** Address Mode: Absolute */
    ABS,
    /** Address Mode:  Absolute with X Offset */
    ABX,
    /** Address Mode:  Absolute with Y Offset */
    ABY,
    /** Address Mode: Indirect */
    IND,
    /** Address Mode: Indirect X */
    IZX,
    /** Address Mode: Indirect Y */
    IZY,
    /** Address Mode: Acumulator */
    ACC,
}

impl Cpu6502 {
    pub fn addres_mode(&mut self, addr_mode: AddressMode) -> u8 {
        match addr_mode {
            AddressMode::IMP => self.imp(),
            AddressMode::IMM => self.imm(),
            AddressMode::ZP0 => self.zp0(),
            AddressMode::ZPX => self.zpx(),
            AddressMode::ZPY => self.zpy(),
            AddressMode::REL => self.rel(),
            AddressMode::ABS => self.abs(),
            AddressMode::ABX => self.abx(),
            AddressMode::ABY => self.aby(),
            AddressMode::IND => self.ind(),
            AddressMode::IZX => self.izx(),
            AddressMode::IZY => self.izy(),
            AddressMode::ACC => self.acc(),
        }
    }

    fn acc(&mut self) -> u8 {
        self.fetched = 0;
        println!("ACCUMULATOR ADDRESS");
        0
    }

    /** Address Mode: Implied.

        Nesse modo a instrução não recebe nenhum argumento ou utiliza o acumulador como argumento, por isso
    vamos apenas ler o acumulador
         */
    fn imp(&mut self) -> u8 {
        self.fetched = self.a;
        0
    }

    /** Address Mode: Immediate.

        Como o próprio nome diz, a instrução é imediata e utiliza o próximo byte como argumento, vamos preparar para ler o
    endereço do próximo byte
         */
    fn imm(&mut self) -> u8 {
        self.addr_abs = self.pc;
        self.pc_next();
        0
    }

    /** Address Mode: Zero Page.

    Nesse modo utilizaremos um endereço da página zero como argumento (0x0000 - 0x00FF), vamos ler o valor do prómimo byte
    para montar o endereço da página zero que será utilizado.
    Como vamos ler da página zero precisamos somente de 1 byte para saber o endereço
    */
    fn zp0(&mut self) -> u8 {
        self.addr_abs = self.read(self.pc) as u16;
        self.pc_next();
        self.addr_abs &= 0x00FF;
        0
    }

    /** Address Mode:  Zero Page with X Offset.

    Mesmo funcionamento do Zero Page, porém vamos utilizar o registrador x para incrementar o offset
    */
    fn zpx(&mut self) -> u8 {
        self.addr_abs = self.read(self.pc) as u16 + self.x as u16;
        self.pc_next();
        self.addr_abs &= 0x00FF;
        0
    }

    /** Address Mode: Zero Page with Y Offset.

    Mesma coisa do Zero Page with X Offset, porém utilizando o registrador y.
    */
    fn zpy(&mut self) -> u8 {
        self.addr_abs = self.read(self.pc) as u16 + self.y as u16;
        self.pc_next();
        self.addr_abs &= 0x00FF;
        0
    }

    /** Address Mode: Relative.

    Esse modo é exclusivo para instruções branch (desvio), o endereço deve estar
    entre -128 e +127 relativo ao endereço atual. Vamos ler o valor do prómio byte para
    montar o endereço relativo.
    */
    fn rel(&mut self) -> u8 {
        self.addr_rel = self.read(self.pc) as u16;
        self.pc_next();

        // Caso o primeiro bit for true então o valor é negativo
        if (self.addr_rel & 0x80) > 0 {
            self.addr_rel |= 0xFF00;
        }

        0
    }

    /** Address Mode: Absolute.

       Um endereço completo de 16 bits será utilizado como argumento, vamos ler os próximos 2 bytes
    */
    fn abs(&mut self) -> u8 {
        self.addr_abs = self.read_next_16b();

        0
    }

    /** Address Mode:  Absolute with X Offset.

    Mesma coisa do Absolute, porém utilizando o registrador x como offset.
    Caso o endereço resultado mudar de página um ciclo adicional é necessário.
    */
    fn abx(&mut self) -> u8 {
        let addr_abs = self.read_next_16b();
        self.addr_abs = addr_abs + self.x as u16;

        if (self.addr_abs & 0xFF00) != (addr_abs & 0xFF00) {
            return 1;
        }

        0
    }

    /** Address Mode:  Absolute with Y Offset.

        Mesma funcionalidade do ABX porém utilizando o registrador y como offset.
    */
    fn aby(&mut self) -> u8 {
        let addr_abs = self.read_next_16b();
        self.addr_abs = addr_abs + self.y as u16;

        if (self.addr_abs & 0xFF00) != (addr_abs & 0xFF00) {
            return 1;
        }

        0
    }

    /** Address Mode: Indirect.

    O endereço de 16 bits fornecido é lido para obter o endereço 16b correto, ou seja, vamos
    ler o endereço 16 bits e o próximo endereço para formar o novo endereço de 16 bits. Essa instrução
    tem um bug no hardware, pois quando o byte inferior (lsb) for 0xFF em vez do próximo endereço ser
    da próxima pagina a instrução lê o byte do começo da página, vamos emular esse bug também.
    */
    fn ind(&mut self) -> u8 {
        let ptr = self.read_next_16b();

        if (ptr & 0x00FF) == 0x00FF {
            self.addr_abs = ((self.read(ptr & 0xFF00) as u16) << 8) | self.read(ptr) as u16;
        } else {
            self.addr_abs = ((self.read(ptr + 1) as u16) << 8) | self.read(ptr) as u16;
        }

        0
    }

    /** Address Mode: Indirect X.

        The supplied 8-bit address is offset by X Register to index
    a location in page 0x00. The actual 16-bit address is read
    from this location
        */
    fn izx(&mut self) -> u8 {
        let t = self.read(self.pc) as u16;
        self.pc_next();

        let addr = (t + self.x as u16) & 0x00FF;
        let lo = self.read(addr & 0x00FF) as u16;
        let hi = self.read((addr + 1) & 0x00FF) as u16;

        self.addr_abs = (hi << 8) | lo;

        0
    }

    /** Address Mode: Indirect Y.

        The supplied 8-bit address indexes a location in page 0x00. From
    here the actual 16-bit address is read, and the contents of
    Y Register is added to it to offset it. If the offset causes a
    change in page then an additional clock cycle is required.
    */
    fn izy(&mut self) -> u8 {
        let t = self.read(self.pc) as u16;
        self.pc_next();
        let lo = self.read(t & 0x00FF) as u16;
        let hi = self.read((t + 1) & 0x00FF) as u16;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs = self.addr_abs + self.y as u16;

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            return 1;
        }

        0
    }
}
