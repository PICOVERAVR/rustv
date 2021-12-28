.macro slli_tests
    # logical
    Test_Rd_Rs1_Rs2 1, IMM, slli, 0x00000001, 0x00000001, 0
    Test_Rd_Rs1_Rs2 2, IMM, slli, 0x00000002, 0x00000001, 1
    Test_Rd_Rs1_Rs2 3, IMM, slli, 0x00000080, 0x00000001, 7
    Test_Rd_Rs1_Rs2 4, IMM, slli, 0x00004000, 0x00000001, 14
    Test_Rd_Rs1_Rs2 5, IMM, slli, 0x80000000, 0x00000001, 31
    Test_Rd_Rs1_Rs2 6, IMM, slli, 0xffffffff, 0xffffffff, 0
    Test_Rd_Rs1_Rs2 7, IMM, slli, 0xfffffffe, 0xffffffff, 1
    Test_Rd_Rs1_Rs2 8, IMM, slli, 0xffffff80, 0xffffffff, 7 
    Test_Rd_Rs1_Rs2 9, IMM, slli, 0xffffc000, 0xffffffff, 14
    Test_Rd_Rs1_Rs2 10, IMM, slli, 0x80000000, 0xffffffff, 31
    Test_Rd_Rs1_Rs2 11, IMM, slli, 0x21212121, 0x21212121, 0
    Test_Rd_Rs1_Rs2 12, IMM, slli, 0x42424242, 0x21212121, 1
    Test_Rd_Rs1_Rs2 13, IMM, slli, 0x90909080, 0x21212121, 7
    Test_Rd_Rs1_Rs2 14, IMM, slli, 0x48484000, 0x21212121, 14
    Test_Rd_Rs1_Rs2 15, IMM, slli, 0x80000000, 0x21212121, 31

    # shared src/dst
    Test_Rs1_Rs1_Rs2 16, IMM, slli, 0x00000080, 0x00000001, 7

    # dst bypass
    Test_Rd_Bypass 17, 0, IMM, slli, 0x00000080, 0x00000001, 7
    Test_Rd_Bypass 18, 1, IMM, slli, 0x00004000, 0x00000001, 14
    Test_Rd_Bypass 19, 2, IMM, slli, 0x80000000, 0x00000001, 31

    # src bypass
    Test_Rs1_Bypass 20, 0, -1, IMM, slli, 0x00000080, 0x00000001, 7
    Test_Rs1_Bypass 21, 1, -1, IMM, slli, 0x00004000, 0x00000001, 14
    Test_Rs1_Bypass 22, 2, -1, IMM, slli, 0x80000000, 0x00000001, 31

    # zero register
    Test_Rd_Zero_Imm 23, slli, 0, 31
    Test_Zero_Rs1_Rs2 24, IMM, slli, 33, 20

.endm
