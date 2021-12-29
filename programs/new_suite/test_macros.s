.include "port_macros.s"

# generic code macros

# Test_Val runs a section of code and compares the value in a1 to exp_val.
# The test number is stored in t0 and is included for debugging purposes.

# NOTE: s1, s2, and t0 are used for testing values and should not be written in the code parameter.

# single line example:
# Test_Val 1, 5, "addi a1, zero, 5"
# multi line example:
# Test_Val 1, 5, "nop; nop; nop; addi a1, zero, 5"
.macro Test_Val suite, test, exp_val, code
    Test_Setup \suite, \test

    \code
    li a0, \exp_val
    Assert_eq a0, a1
.endm

# Test_Seq runs a section of code and checks if the values in a0 and a1 are equal.
# The test number is stored in t0 and is included for debugging purposes.

# NOTE: s1, s2, and t0 are used for testing values and should not be written in the code parameter.

# single line example:
# Test_Val 1, "addi a1, zero, 5"
# multi line example:
# Test_Val 1, "addi a1, zero, 5; addi a0, zero, 5"
.macro Test_Seq suite, test, code
    Test_Setup \suite, \test

    \code
    Assert_eq a0, a1
.endm

# gives a useful symbol name to a test case (helpful when looking at assembly)
.macro Test_Setup suite, test
    li t0, \@
\suite\()_\test:
.endm

# arithmetic instruction macros

.equiv REG, 0
.equiv IMM, 1

.macro Test_Rd_Rs1_Rs2 suite, test, imm, instr, exp_val, lval, rval
    Test_Setup \suite, \test

    li a4, \exp_val
    li a1, \lval
.if \imm
    \instr a3, a1, \rval
.else
    li a2, \rval
    \instr a3, a1, a2
.endif

    Assert_eq a3, a4
.endm

.macro Test_Rs1_Rs1_Rs2 suite, test, imm, instr, exp_val, lval, rval
    Test_Setup \suite, \test

    li a3, \exp_val
    li a1, \lval
.if \imm
    \instr a1, a1, \rval
.else
    li a2, \rval
    \instr a1, a1, a2
.endif

    Assert_eq a1, a3
.endm

.macro Test_Rs2_Rs1_Rs2 suite, test, instr, exp_val, lval, rval
    Test_Setup \suite, \test

    li a3, \exp_val
    li a1, \lval
    li a2, \rval
    \instr a2, a1, a2

    Assert_eq a2, a3
.endm

.macro Test_Rs1_Rs1_Rs1 suite, test, instr, exp_val, lval
    Test_Setup \suite, \test

    li a1, \lval
    li a2, \exp_val
    \instr a1, a1, a1

    Assert_eq a1, a2
.endm

# NOTE: riscv_tests loops three times on every invocation of this test
.macro Test_Rd_Bypass suite, test, nop_count, imm, instr, exp_val, lval, rval
    Test_Setup \suite, \test

    li a4, \exp_val
    li a1, \lval
.if \imm
    \instr a3, a1, \rval
.else
    li a2, \rval
    \instr a3, a1, a2
.endif
.rept \nop_count
    # repeat for nop_count iterations
    nop
.endr
    addi t1, a3, 0

    Assert_eq a3, a4
    Assert_eq t1, a3
.endm

# rs1 bypass and rs2 bypass macros are almost the same except for loading operands in the opposite order

# NOTE: riscv_tests loops three times on every invocation of this test
.macro Test_Rs1_Bypass suite, test, rs1_nop_count, rs2_nop_count, imm, instr, exp_val, lval, rval
    Test_Setup \suite, \test

    li a4, \exp_val
    li a1, \lval
.rept \rs1_nop_count
    nop
.endr

.if \imm
    \instr a3, a1, \rval
.else
    li a2, \rval
.rept \rs2_nop_count
    nop
.endr
    \instr a3, a1, a2
.endif

    Assert_eq a3, a4
.endm

# NOTE: riscv_tests loops three times on every invocation of this test
.macro Test_Rs2_Bypass suite, test, rs1_nop_count, rs2_nop_count, instr, exp_val, lval, rval
    Test_Setup \suite, \test

    li a4, \exp_val
    li a2, \rval
.rept \rs2_nop_count
    nop
.endr

    li a1, \lval
.rept \rs1_nop_count
    nop
.endr
    \instr a3, a1, a2

    Assert_eq a3, a4
.endm

.macro Test_Rd_Rs1_Zero suite, test, instr, exp_val, lval
    Test_Setup \suite, \test

    li a3, \exp_val
    li a1, \lval
    \instr a2, a1, zero

    Assert_eq a2, a3
.endm

.macro Test_Rd_Zero_Rs2 suite, test, instr, exp_val, rval
    Test_Setup \suite, \test

    li a3, \exp_val
    li a1, \rval
    \instr a2, zero, a1

    Assert_eq a2, a3
.endm

.macro Test_Rd_Zero_Imm suite, test, instr, exp_val, lval
    Test_Setup \suite, \test

    li a3, \exp_val
    \instr a2, zero, \lval

    Assert_eq a2, a3
.endm

.macro Test_Rd_Zero_Zero suite, test, instr, exp_val
    Test_Setup \suite, \test

    li a2, \exp_val
    \instr a1, zero, zero

    Assert_eq a1, a2
.endm

.macro Test_Zero_Rs1_Rs2 suite, test, imm, instr, lval, rval
    Test_Setup \suite, \test

    li a3, 0
    li a1, \lval
.if \imm
    \instr zero, a1, \rval
.else
    li a2, \rval
    \instr zero, a1, a2
.endif

    # can catch cases where zero register can be written with nonzero values
    Assert_eq a3, zero
.endm
