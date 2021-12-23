# rustv

A RISC-V software emulator written in Rust.

# Supported Instruction Sets (and Extensions)
- rv32i
- TODO: Zicsr
- TODO: Zifencei
- TODO: rv64i
- TODO: M

# Example usage
```
# gen.sh assembles a test program, converts the .elf file to a .hex file and dumps the output
$ ./programs/gen.sh fib.s -dump
...
# run the .hex output
$ rustv fib.hex
...
```

# Implementation details
