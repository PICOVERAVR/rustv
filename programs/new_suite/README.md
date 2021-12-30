# minv

Minv is an instruction level test suite for RISC-V processors that aren't finished yet.  The test suite borrows from the official [riscv-tests](https://github.com/riscv-software-src/riscv-tests) test suite, but only uses a small subset of the base RV32I instruction set to run these tests.  As an example, this allows for testing arithmetic instructions even if jump and branch instructions aren't done yet.
  
Minv isn't designed to be a complete verification suite, as these already exist.  Minv is designed for hobbyists wanting to quickly test their RISC-V emulators or HDL cores as well as those who want to quickly verify subsets of the RISC-V ISA.

## Features
 - Uses only a small subset of the RV32I instruction set for all tests (see the Required Instructions section for details)
 - Easy to read, use, and port
   - Requires only riscv-binutils

## Building
 - Run `make` to build the test suite.
 - Run `make dump` to dump the assembly of the test suite (with arrows indicating jump targets)
 - Run `make dump_simple` to dump the assembly of the test suite using `x` names for registers, no pseudoinstructions, and no jump arrows.

## Required (Pseudo)instructions
 - `li`
    - formed using `lui` and `addi`
    - for loading immediates into registers
 - `la`
    - formed using `auipc` and `addi`
    - for loading addresses into registers
 - `add`
    - for moving values between registers
 - `ecall`
    - used to compare registers for equality

## Porting
