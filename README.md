# rustv

A simple RISC-V software emulator written in Rust.

# Supported Instruction Sets (and Extensions)
- rv32i
- Zifencei (instruction is supported but does nothing)
- TODO: Zicsr
- TODO: rv64i
- TODO: M

# Example usage
```
# gen.sh assembles a test program, converts the .elf file to a .hex file and dumps the output
$ ./programs/gen.sh test.s -dump
_start():
   0:   0aa00f93                addi    x31,x0,170
   4:   0ab00f13                addi    x30,x0,171
   8:   01efa0b3                slt     x1,x31,x30
   c:   00008663                beq     x1,x0,18 <true>
...
# run the .hex output
$ rustv ./test.hex
reached end of instruction memory
```

# Implementation details
 - little-endian
 - misaligned memory accesses cause a fatal trap and result in a panic from the emulator.
 - execution automatically ends when execution reaches the end of instruction memory
   - the emulator will panic if execution jumps past this point
 - memory reads and writes are currently not reordered at all
