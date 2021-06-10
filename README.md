# rust-nes-emulator

Emulador de NES escrito em Rust com propósito de estudos.

## Como usar

### Executando

```shell
$ cargo run --release -- --rom game.nes
```

### Build

```shell
$ cargo build --release
$ cd target/release
$ ./rust-nes-emulator --rom game.nes
```

## Feature & Known Issue

- [x] CPU
  - [x] Registradores
  - [x] MemoryAccess
  - [x] Opcode oficiais
  - [x] Opcode não oficiais
  - [x] Clock
- [x] Cassette(Mapper)
  - [x] Ler arquivos .nes
  - [ ] Mappers
    - [x] Mapper0
- [x] PPU
- [x] PAD
  - [x] Pad-1
  - [ ] Pad-2 (Bug)
- [ ] APU
