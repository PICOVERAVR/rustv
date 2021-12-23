.text
_start:
	addi x1, x0, 4

	li x2, 0xffffffff
	sw x2, 0(x1)
	
	li x2, 0
	sh x2, 0(x1)

	li x2, 0xff
	sb x2, 0(x1)
