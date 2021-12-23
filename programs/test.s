.text
_start:
	li x31, 0xaa
	li x30, 0xab
	slt x1, x31, x30
	beq x1, x0, true
false:
	nop
	nop
true:
	la x4, end
	jal x2, end
	nop
	nop
end:
	li x3, 0x1

