## Bus

- [x] Ram
- [] Cartridge connection (Read & Write)
- [] CPU connection (Read & Write)
- [] PPU connection (Read & Write)
- [] APU connection (Read & Write)
- [] Clock

## CPU

- [x] Registers
- [x] Register Status Flags
- [x] Bus connection (Read and Write)
- [x] AddressModes
- [x] OpCodes
- [x] Instructions
- [x] External Inputs (Pins)
- [x] Clock

## Cartridge

- [x] Memory access (Read & Write)
- [x] File Reader
- [] Mapper interface/Trait
- [] Mappers
  - [x] Mapper 000

## PPU

## APU

## Próximas atividades

- Corrigir os registradores control, mask e status do PPU para utilizar apenas 1 byte (reg) e as propriedades da struct retornar o valor do bit correspondente no reg
