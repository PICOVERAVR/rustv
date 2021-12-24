#!/usr/bin/env sh

BNAME="$(basename $1 .s)"
TNAME="riscv32-unknown-elf"

$TNAME-as -march=rv32i $1 -o $BNAME.elf &&
$TNAME-objcopy $BNAME.elf $BNAME.hex --output-target=binary

# dump the executable assembly if user asks for it
if [ $2 == "-dump" ]
then
	$TNAME-objdump -d \
		--line-numbers \
		--visualize-jumps=extended-color \
		-Mno-aliases,numeric \
		$BNAME.elf | tail +8
fi

