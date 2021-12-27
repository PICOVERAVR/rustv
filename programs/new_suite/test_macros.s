.include "port_macros.s"

.equiv REG, 0
.equiv IMM, 1

.macro Test_Rd_Rs1_Rs2 test_idx, imm, instr, exp_val, lval, rval
    li t0, \test_idx

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

.macro Test_Rs1_Rs1_Rs2 test_idx, imm, instr, exp_val, lval, rval
    li t0, \test_idx

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

.macro Test_Rs2_Rs1_Rs2 test_idx, instr, exp_val, lval, rval
    li t0, \test_idx

    li a3, \exp_val
    li a1, \lval
    li a2, \rval
    \instr a2, a1, a2

    Assert_eq a2, a3
.endm

.macro Test_Rs1_Rs1_Rs1 test_idx, instr, exp_val, lval
    li t0, \test_idx

    li a1, \lval
    li a2, \exp_val
    \instr a1, a1, a1

    Assert_eq a1, a2
.endm

# NOTE: riscv_tests loops three times on every invocation of this test
.macro Test_Rd_Bypass test_idx, nop_count, imm, instr, exp_val, lval, rval
    li t0, \test_idx

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

# NOTE: riscv_tests loops three times on every invocation of this test
.macro Test_Rs_Bypass test_idx, rs1_nop_count, rs2_nop_count, imm, instr, exp_val, lval, rval
    li t0, \test_idx

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

.macro Test_Rd_Rs1_Zero test_idx, instr, exp_val, lval
    li t0, \test_idx

    li a3, \exp_val
    li a1, \lval
    \instr a2, a1, zero

    Assert_eq a2, a3
.endm

.macro Test_Rd_Zero_Rs2 test_idx, instr, exp_val, rval
    li t0, \test_idx

    li a3, \exp_val
    li a1, \rval
    \instr a2, zero, a1

    Assert_eq a2, a3
.endm

.macro Test_Rd_Zero_Imm test_idx, instr, exp_val, lval
    li t0, \test_idx

    li a3, \exp_val
    \instr a2, zero, \lval

    Assert_eq a2, a3
.endm

.macro Test_Rd_Zero_Zero test_idx, instr, exp_val
    li t0, \test_idx

    li a2, \exp_val
    \instr a1, zero, zero

    Assert_eq a1, a2
.endm

.macro Test_Zero_Rs1_Rs2 test_idx, imm, instr, exp_val, lval, rval
    li t0, \test_idx

    li a3, \exp_val
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
