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

        self.pc = (hi << 8) | lo;

        // limpar registradores
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.stkp = 0xFD;
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
        self.pc = ((hi as u16) << 8) | lo as u16;

        self.cycles = 8;
    }

    /** Perform one clock cycles worth of emulation */
    pub fn cpu_clock(&mut self) {
        // println!("cycles: {}", self.cycles);
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
            // println!("pc: {}", self.pc);

            // unsafe {
            //     if let Some(bus) = self.bus.as_mut() {
            //         if let Some(cart) = &mut bus.cartridge {
            //             println!("cart in cpu clock: {}", cart.prg_memory[0x3fff]);
            //         }
            //     }
            // }

            self.opcode = self.read(self.pc);

            // println!("opcode: {}", self.opcode);
            // Sempre setar a flag unused para 1 (true)
            self.set_flag(Flags6502::U, true);

            // println!("pc_next");
            // Incrimentar o program counter
            self.pc_next();

            // println!("get_instruction");
            let instruction = self.get_instruction();

            // println!(
            //     "instruction({:?},{:?}) - started",
            //     instruction.opcode, instruction.addres_mode
            // );

            // numero inicial de ciclos
            self.cycles = instruction.cycles;

            // aplicar o addres mode e guardar os ciclos adicionais
            let aditional_cycles1 = self.addres_mode(instruction.addres_mode);

            // println!("fetch: {}", self.fetched);

            // executar o opcode e guardar os ciclos adicionais
            let aditional_cycles2 = self.opcode(instruction.opcode);

            // println!(
            //     "instruction({:?},{:?}) - finished",
            //     instruction.opcode, instruction.addres_mode
            // );

            // adicionar ciclos
            self.cycles += aditional_cycles1 & aditional_cycles2;

            // Sempre setar a flag unused para 1 (true)
            self.set_flag(Flags6502::U, true);
        }

        self.clock_count += 1;

        // decrementando o numero de ciclos
        self.cycles -= 1;
    }

    pub fn clock(&mut self) {
        // println!("bus: {}", self.version);
        // Clocking. The heart and soul of an emulator. The running
        // frequency is controlled by whatever calls this function.
        // So here we "divide" the clock as necessary and call
        // the peripheral devices clock() function at the correct
        // times.

        // The fastest clock frequency the digital system cares
        // about is equivalent to the PPU clock. So the PPU is clocked
        // each time this function is called...
        self.bus.ppu.clock();

        // The CPU runs 3 times slower than the PPU so we only call its
        // clock() function every 3 times this function is called. We
        // have a global counter to keep track of this.
        if self.bus.system_clock_counter % 3 == 0 {
            // Is the system performing a DMA transfer form CPU memory to
            // OAM memory on PPU?...
            if self.bus.dma_transfer {
                // ...Yes! We need to wait until the next even CPU clock cycle
                // before it starts...
                if self.bus.dma_dummy {
                    // ...So hang around in here each clock until 1 or 2 cycles
                    // have elapsed...
                    if self.bus.system_clock_counter % 2 == 1 {
                        // ...and finally allow DMA to start
                        self.bus.dma_dummy = false;
                    }
                } else {
                    // DMA can take place!
                    if self.bus.system_clock_counter % 2 == 0 {
                        // On even clock cycles, read from CPU bus
                        self.bus.dma_data = self.bus.read(
                            (self.bus.dma_page as u16) << 8 | self.bus.dma_addr as u16,
                            false,
                        );
                    } else {
                        // On odd clock cycles, write to PPU OAM
                        self.bus.ppu.oam_write(self.bus.dma_addr, self.bus.dma_data);
                        // Increment the lo byte of the address
                        self.bus.dma_addr += 1;
                        // If this wraps around, we know that 256
                        // bytes have been written, so end the DMA
                        // transfer, and proceed as normal
                        if self.bus.dma_addr == 0x00 {
                            self.bus.dma_transfer = false;
                            self.bus.dma_dummy = true;
                        }
                    }
                }
            } else {
                // No DMA happening, the CPU is in control of its
                // own destiny. Go forth my friend and calculate
                // awesomeness for many generations to come...
                self.cpu_clock();
            }
        }

        // The PPU is capable of emitting an interrupt to indicate the
        // vertical blanking period has been entered. If it has, we need
        // to send that irq to the CPU.
        if self.bus.ppu.nmi {
            self.bus.ppu.nmi = false;
            self.nmi();
        }

        self.bus.system_clock_counter += 1;
    }
}
