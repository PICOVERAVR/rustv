#!/usr/bin/env sh

BNAME="$(basename $1 .s)"
riscv32-elf-as -march=rv32i $1 -o $BNAME.elf &&
riscv32-elf-objcopy $BNAME.elf $BNAME.hex --output-target=binary

# dump the executable assembly if user asks for it
if [ $2 == "-dump" ]
then
	riscv32-elf-objdump -d \
		--line-numbers \
		--visualize-jumps=extended-color \
		-Mno-aliases,numeric \
		$BNAME.elf | tail +8
fi

