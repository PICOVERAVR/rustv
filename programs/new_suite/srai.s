.macro srai_tests
    # logical
    Test_Rd_Rs1_Rs2 1, IMM, srai, 0x00000000, 0x00000000, 0
    Test_Rd_Rs1_Rs2 2, IMM, srai, 0xc0000000, 0x80000000, 1
    Test_Rd_Rs1_Rs2 3, IMM, srai, 0xff000000, 0x80000000, 7
    Test_Rd_Rs1_Rs2 4, IMM, srai, 0xfffe0000, 0x80000000, 14
    Test_Rd_Rs1_Rs2 5, IMM, srai, 0xffffffff, 0x80000001, 31
    Test_Rd_Rs1_Rs2 6, IMM, srai, 0x7fffffff, 0x7fffffff, 0
    Test_Rd_Rs1_Rs2 7, IMM, srai, 0x3fffffff, 0x7fffffff, 1
    Test_Rd_Rs1_Rs2 8, IMM, srai, 0x00ffffff, 0x7fffffff, 7
    Test_Rd_Rs1_Rs2 9, IMM, srai, 0x0001ffff, 0x7fffffff, 14
    Test_Rd_Rs1_Rs2 10, IMM, srai, 0x00000000, 0x7fffffff, 31
    Test_Rd_Rs1_Rs2 11, IMM, srai, 0x81818181, 0x81818181, 0
    Test_Rd_Rs1_Rs2 12, IMM, srai, 0xc0c0c0c0, 0x81818181, 1
    Test_Rd_Rs1_Rs2 13, IMM, srai, 0xff030303, 0x81818181, 7
    Test_Rd_Rs1_Rs2 14, IMM, srai, 0xfffe0606, 0x81818181, 14
    Test_Rd_Rs1_Rs2 15, IMM, srai, 0xffffffff, 0x81818181, 31

    # shared src/dst
    Test_Rs1_Rs1_Rs2 16, IMM, srai, 0xff000000, 0x80000000, 7

    # dst bypass
    Test_Rd_Bypass 17, 0, IMM, srai, 0xff000000, 0x80000000, 7
    Test_Rd_Bypass 18, 1, IMM, srai, 0xfffe0000, 0x80000000, 14
    Test_Rd_Bypass 19, 2, IMM, srai, 0xffffffff, 0x80000001, 31

    # src bypass
    Test_Rs1_Bypass 20, 0, -1, IMM, srai, 0xff000000, 0x80000000, 7
    Test_Rs1_Bypass 21, 1, -1, IMM, srai, 0xfffe0000, 0x80000000, 14
    Test_Rs1_Bypass 22, 2, -1, IMM, srai, 0xffffffff, 0x80000001, 31

    # zero register
    Test_Rd_Zero_Imm 23, srai, 0, 31
    Test_Zero_Rs1_Rs2 24, IMM, srai, 33, 20
.endm
