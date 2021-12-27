.macro addi_tests
    # arithmetic
    Test_Rd_Rs1_Rs2 1, IMM, addi, 0x00000000, 0x00000000, 0x000
    Test_Rd_Rs1_Rs2 2, IMM, addi, 0x00000002, 0x00000001, 0x001
    Test_Rd_Rs1_Rs2 3, IMM, addi, 0x0000000a, 0x00000003, 0x007
    Test_Rd_Rs1_Rs2 4, IMM, addi, 0xfffff800, 0x00000000, -2048 # 0x800
    Test_Rd_Rs1_Rs2 5, IMM, addi, 0x80000000, 0x80000000, 0x000
    Test_Rd_Rs1_Rs2 6, IMM, addi, 0x7ffff800, 0x80000000, -2048 # 0x800
    Test_Rd_Rs1_Rs2 7, IMM, addi, 0x000007ff, 0x00000000, 0x7ff
    Test_Rd_Rs1_Rs2 8, IMM, addi, 0x7fffffff, 0x7fffffff, 0x000
    Test_Rd_Rs1_Rs2 9, IMM, addi, 0x800007fe, 0x7fffffff, 0x7ff
    Test_Rd_Rs1_Rs2 10, IMM, addi, 0x800007ff, 0x80000000, 0x7ff
    Test_Rd_Rs1_Rs2 11, IMM, addi, 0x7ffff7ff, 0x7fffffff, -2048 # 0x800
    Test_Rd_Rs1_Rs2 12, IMM, addi, 0xffffffff, 0x00000000, -1 # 0xfff
    Test_Rd_Rs1_Rs2 13, IMM, addi, 0x00000000, 0xffffffff, 0x001
    Test_Rd_Rs1_Rs2 14, IMM, addi, 0xfffffffe, 0xffffffff, -1 # 0xfff
    Test_Rd_Rs1_Rs2 15, IMM, addi, 0x80000000, 0x7fffffff, 0x001

    # shared src/dst
    Test_Rs1_Rs1_Rs2 16, IMM, addi, 24, 13, 11

    # dst bypass
    Test_Rd_Bypass 17, 0, IMM, addi, 24, 13, 11
    Test_Rd_Bypass 18, 1, IMM, addi, 23, 13, 10
    Test_Rd_Bypass 19, 2, IMM, addi, 22, 13, 9

    # src bypass
    Test_Rs_Bypass 20, 0, -1, IMM, addi, 24, 13, 11
    Test_Rs_Bypass 21, 1, -1, IMM, addi, 23, 13, 10
    Test_Rs_Bypass 22, 2, -1, IMM, addi, 22, 13, 9

    # zero register
    Test_Rd_Zero_Imm 23, addi, 32, 32
    Test_Zero_Rs1_Rs2 24, IMM, addi, 33, 50
.endm
