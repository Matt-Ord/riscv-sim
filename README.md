# RISC-V Sim

A simulation of a simple RISC-V computer

## Getting Started

### Installing

This project requires cargo to be installed which can be downloaded from the
[rust website](https://www.rust-lang.org/tools/install).

### Running

Once rust is installed the library can then be tested using

```sh
cargo test
```

A RISC machine can then be made by providing an already created registry and
memory

```rust
let mut memory = Memory::default();
memory.set_four_byte(u20::new(0), 100);
let mut registry = Registry::default();
registry.set(rs1, 100);

let mut machine = RISCMachine {
    memory,
    registry,
    program_counter: u20::new(0),
};
```

or by simply loading the memory into an empty machine

```rust
let mut memory = Memory::default();

let mut machine = RISCMachine::default();
machine.load_memory(memory);
```

The machine currently supports 8 instructions

```rust
pub enum CPUInstruction {
    ADD { rd: u5, rs1: u5, rs2: u5 },
    SUB { rd: u5, rs1: u5, rs2: u5 },
    LW { rd: u5, rs1: u5, imm: u32 },
    SW { rs1: u5, rs2: u5, imm: u32 },
    JAL { rd: u5, imm: u32 },
    JALR { rd: u5, rs1: u5, imm: u32 },
    BEQ { rs1: u5, rs2: u5, imm: u32 },
    BNE { rs1: u5, rs2: u5, imm: u32 },
}
```

A detailed description of each instruction can be found
[here](https://www.csl.cornell.edu/courses/ece5745/handouts/ece5745-tinyrv-isa.txt).

The machine can be ran one tick at a time, or until the program counter reaches
a specific value

```rust
//Process a single instruction
machine.tick()?;

//Process until the program counter reaches 10
machine.run(&|pc| -> bool { pc == u20::new(10) })?;
```

## Acknowledgments

This simulation took inspiration from the
[Tiny RISC-V](https://www.csl.cornell.edu/courses/ece5745/handouts/ece5745-tinyrv-isa.txt)
architecture, which is a simple subset of the complete RISC-V specification
which can be found
[here](https://riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf).
