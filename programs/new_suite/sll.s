.macro sll_tests
    # logical
    Test_Rd_Rs1_Rs2 1, REG, sll, 0x00000001, 0x00000001, 0
    Test_Rd_Rs1_Rs2 2, REG, sll, 0x00000002, 0x00000001, 1
    Test_Rd_Rs1_Rs2 3, REG, sll, 0x00000080, 0x00000001, 7
    Test_Rd_Rs1_Rs2 4, REG, sll, 0x00004000, 0x00000001, 14
    Test_Rd_Rs1_Rs2 5, REG, sll, 0x80000000, 0x00000001, 31
    Test_Rd_Rs1_Rs2 6, REG, sll, 0xffffffff, 0xffffffff, 0
    Test_Rd_Rs1_Rs2 7, REG, sll, 0xfffffffe, 0xffffffff, 1
    Test_Rd_Rs1_Rs2 8, REG, sll, 0xffffff80, 0xffffffff, 7
    Test_Rd_Rs1_Rs2 9, REG, sll, 0xffffc000, 0xffffffff, 14
    Test_Rd_Rs1_Rs2 10, REG, sll, 0x80000000, 0xffffffff, 31
    Test_Rd_Rs1_Rs2 11, REG, sll, 0x21212121, 0x21212121, 0
    Test_Rd_Rs1_Rs2 12, REG, sll, 0x42424242, 0x21212121, 1
    Test_Rd_Rs1_Rs2 13, REG, sll, 0x90909080, 0x21212121, 7
    Test_Rd_Rs1_Rs2 14, REG, sll, 0x48484000, 0x21212121, 14
    Test_Rd_Rs1_Rs2 15, REG, sll, 0x80000000, 0x21212121, 31

    # ensure that shifts only use lower 5 bits
    Test_Rd_Rs1_Rs2 16, REG, sll, 0x21212121, 0x21212121, 0xffffffe0
    Test_Rd_Rs1_Rs2 17, REG, sll, 0x42424242, 0x21212121, 0xffffffe1
    Test_Rd_Rs1_Rs2 18, REG, sll, 0x90909080, 0x21212121, 0xffffffe7
    Test_Rd_Rs1_Rs2 19, REG, sll, 0x48484000, 0x21212121, 0xffffffee
    Test_Rd_Rs1_Rs2 20, REG, sll, 0x00000000, 0x21212120, 0xffffffff

    # shared src/dst
    Test_Rs1_Rs1_Rs2 21, REG, sll, 0x00000080, 0x00000001, 7
    Test_Rs2_Rs1_Rs2 22, sll, 0x00004000, 0x00000001, 14
    Test_Rs1_Rs1_Rs1 23, sll, 24, 3

    # dst bypass
    Test_Rd_Bypass 24, 0, REG, sll, 0x00000080, 0x00000001, 7
    Test_Rd_Bypass 25, 1, REG, sll, 0x00004000, 0x00000001, 14
    Test_Rd_Bypass 26, 2, REG, sll, 0x80000000, 0x00000001, 31

    # src bypass
    Test_Rs1_Bypass 27, 0, 0, REG, sll, 0x00000080, 0x00000001, 7
    Test_Rs1_Bypass 28, 0, 1, REG, sll, 0x00004000, 0x00000001, 14
    Test_Rs1_Bypass 29, 0, 2, REG, sll, 0x80000000, 0x00000001, 31
    Test_Rs1_Bypass 30, 1, 0, REG, sll, 0x00000080, 0x00000001, 7
    Test_Rs1_Bypass 31, 1, 1, REG, sll, 0x00004000, 0x00000001, 14
    Test_Rs1_Bypass 32, 2, 0, REG, sll, 0x80000000, 0x00000001, 31
    Test_Rs2_Bypass 33, 0, 0, sll, 0x00000080, 0x00000001, 7
    Test_Rs2_Bypass 34, 0, 1, sll, 0x00004000, 0x00000001, 14
    Test_Rs2_Bypass 35, 0, 2, sll, 0x80000000, 0x00000001, 31
    Test_Rs2_Bypass 36, 1, 0, sll, 0x00000080, 0x00000001, 7
    Test_Rs2_Bypass 37, 1, 1, sll, 0x00004000, 0x00000001, 14
    Test_Rs2_Bypass 38, 2, 0, sll, 0x80000000, 0x00000001, 31

    # zero register
    Test_Rd_Rs1_Zero 39, sll, 32, 32
    Test_Rd_Zero_Rs2 40, sll, 0, 15
    Test_Rd_Zero_Zero 41, sll, 0
    Test_Zero_Rs1_Rs2 43, REG, sll, 1024, 2048
.endm
