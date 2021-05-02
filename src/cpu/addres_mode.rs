use super::Cpu6502;

// ADDRESSING MODES

// The 6502 can address between 0x0000 - 0xFFFF. The high byte is often referred
// to as the "page", and the low byte is the offset into that page. This implies
// there are 256 pages, each containing 256 bytes.
//
// Several addressing modes have the potential to require an additional clock
// cycle if they cross a page boundary. This is combined with several instructions
// that enable this additional clock cycle. So each addressing function returns
// a flag saying it has potential, as does each instruction. If both instruction
// and address function return 1, then an additional clock cycle is required.

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum AddressMode {
    /** Address Mode: Implied */
    IMP,
    /** Address Mode: Immediate */
    IMM,
    /** Address Mode: Zero Page */
    ZP0,
    /** Address Mode:  Zero Page with X Offset */
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

    nesse modo a instrução não recebe nenhum dado adicional, nesse caso a instrução faz coisas simples
     */
    fn imp(&mut self) -> u8 {
        self.fetched = self.a;
        0
    }

    /** Address Mode: Immediate.

    a instrução usa o próximo byte como valor, vamos preparar para ler o endereço do próximo byte
     */
    fn imm(&mut self) -> u8 {
        self.addr_abs = self.pc;
        self.pc_next();
        println!("addr_abs: {:#06x}", self.addr_abs);
        0
    }

    /** Address Mode: Zero Page.

        To save program bytes, zero page addressing allows you to absolutely address
    a location in first 0xFF bytes of address range. Clearly this only requires
    one byte instead of the usual two.
     */
    fn zp0(&mut self) -> u8 {
        self.addr_abs = self.read(self.pc) as u16;
        self.pc_next();
        self.addr_abs &= 0x00FF;
        0
    }

    /** Address Mode:  Zero Page with X Offset.

        Fundamentally the same as Zero Page addressing, but the contents of the X Register
    is added to the supplied single byte address. This is useful for iterating through
    ranges within the first page.
     */
    fn zpx(&mut self) -> u8 {
        self.addr_abs = self.read(self.pc + self.x as u16) as u16;
        self.pc_next();
        self.addr_abs &= 0x00FF;
        0
    }

    /** Address Mode: Zero Page with Y Offset.

    Same as above but uses Y Register for offset
     */
    fn zpy(&mut self) -> u8 {
        self.addr_abs = self.read(self.pc + self.x as u16) as u16;
        self.pc_next();
        self.addr_abs &= 0x00FF;
        0
    }

    /** Address Mode: Relative.

    This address mode is exclusive to branch instructions. The address
    must reside within -128 to +127 of the branch instruction, i.e.
    you cant directly branch to any address in the addressable range.
     */
    fn rel(&mut self) -> u8 {
        self.addr_rel = self.read(self.pc) as u16;
        self.pc_next();
        if (self.addr_rel & 0x80) > 0 {
            self.addr_rel |= 0xFF00;
        }

        0
    }

    /** Address Mode: Absolute.

       A full 16-bit address is loaded and used
    */
    fn abs(&mut self) -> u8 {
        self.addr_abs = self.read_next_16b();

        0
    }

    /** Address Mode:  Absolute with X Offset.

    Fundamentally the same as absolute addressing, but the contents of the X Register
    is added to the supplied two byte address. If the resulting address changes
    the page, an additional clock cycle is required
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

        Fundamentally the same as absolute addressing, but the contents of the Y Register
    is added to the supplied two byte address. If the resulting address changes
    the page, an additional clock cycle is required
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

        The supplied 16-bit address is read to get the actual 16-bit address. This is
    instruction is unusual in that it has a bug in the hardware! To emulate its
    function accurately, we also need to emulate this bug. If the low byte of the
    supplied address is 0xFF, then to read the high byte of the actual address
    we need to cross a page boundary. This doesnt actually work on the chip as
    designed, instead it wraps back around in the same page, yielding an
    invalid actual address
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
        let t = self.read(self.pc);
        self.pc_next();

        self.addr_abs = self.read_16b(((t + self.x) & 0x00FF) as u16);

        0
    }

    /** Address Mode: Indirect Y.

        The supplied 8-bit address indexes a location in page 0x00. From
    here the actual 16-bit address is read, and the contents of
    Y Register is added to it to offset it. If the offset causes a
    change in page then an additional clock cycle is required.
    */
    fn izy(&mut self) -> u8 {
        let t = self.read(self.pc);
        self.pc_next();

        let addr_abs = self.read_16b((t & 0x00FF) as u16);
        self.addr_abs = addr_abs + self.y as u16;

        if (self.addr_abs & 0x00FF) != (addr_abs & 0x00FF) {
            return 1;
        }

        0
    }
}
