# nandputer

A project to implement a nand2tetris-style RISC-V virtual machine in Rust.

## Description

The goal of this project is to implement a virtualized RISC-V processor built entirely from NAND (and DFF) components. Rust is used for its speed, memory safety guarantees, and modern programming constructs. This project follows along with the [nand2tetris course](https://www.nand2tetris.org/course) for chapters 1-3 and 5; RISC-V is implemented instead of the educational Hack architecture.
## Getting Started

### Dependencies

This project requires a working Cargo installation; if you do not have Rust/Cargo installed, follow the appropriate instructions [here](https://www.rust-lang.org/tools/install).

### Installing
```
git clone https://github.com/witchofthewires/nandputer.git
cd nandputer
cargo build
```

### Executing program

Currently, the project has completed a nand2tetris-style progression up to Level 3: Memory. A full test suite is also implemented, and an interactive testbench is available by executing
```
cargo run
```
This opens the nandputer interactive prompt:
```
nandputer> 
```
Available commands
```
m - print current outputs of 64-byte memory
w - writes DATA to ADDR, both collected by interactive prompts
q - exit program
```

## License

This project is licensed under the Apache License - see the LICENSE.md file for details

## Acknowledgments

* [nand2tetris](https://www.nand2tetris.org/course)
* [rust book](https://doc.rust-lang.org/book/)
* [RISC-V Specifications](https://lf-riscv.atlassian.net/wiki/spaces/HOME/pages/16154769/RISC-V+Technical+Specifications)
