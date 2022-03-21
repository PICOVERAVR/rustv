# rustv

A simple RISC-V software emulator written in Rust.

# Supported Instruction Sets (and Extensions)
- RV32I
- M
- Zifencei (instruction is supported but does nothing)
- TODO: Zicsr

# Performance
(The binary is from the [riscv_friendly](https://github.com/PICOVERAVR/riscv_friendly) test suite)
  
If we consider only the hot loop of the program, it runs extremely fast for a software emulator:
 - debug build, no printing: 11.65 MHz
 - release build, no printing: 227.61 MHz

If we consider the entire program, rustv is roughly 3.9x faster than the standard RISC-V software emulator:
 - release build, no printing: 15.07 MHz
 - spike (a popular RISC-V emulator): 3.88 MHz

```
$ cargo run --release bare.hex
...
--------------------------------------------------------
Executed in  834.00 micros    fish           external
   usr time  874.00 micros  221.00 micros  653.00 micros
   sys time    0.00 micros    0.00 micros    0.00 micros

$ time spike -m1 --isa=rv32i spike.elf
--------------------------------------------------------
Executed in    3.40 millis    fish           external
   usr time    3.63 millis  127.00 micros    3.50 millis
   sys time    0.06 millis   56.00 micros    0.00 millis
```

Note that rustv does significantly less than spike, so these results should be taken with a grain of salt.

# Implementation details
 - Little-endian
 - Misaligned memory accesses cause a fatal trap and result in a panic from the emulator.
 - the emulator will panic if execution goes out of bounds
 - memory reads and writes are currently not reordered at all
