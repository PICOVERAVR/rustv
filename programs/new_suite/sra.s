.macro sra_tests
    # logical
    Test_Rd_Rs1_Rs2 1, REG, sra, 0x80000000, 0x80000000, 0
    Test_Rd_Rs1_Rs2 2, REG, sra, 0xc0000000, 0x80000000, 1
    Test_Rd_Rs1_Rs2 3, REG, sra, 0xff000000, 0x80000000, 7
    Test_Rd_Rs1_Rs2 4, REG, sra, 0xfffe0000, 0x80000000, 14
    Test_Rd_Rs1_Rs2 5, REG, sra, 0xffffffff, 0x80000001, 31
    Test_Rd_Rs1_Rs2 6, REG, sra, 0x7fffffff, 0x7fffffff, 0
    Test_Rd_Rs1_Rs2 7, REG, sra, 0x3fffffff, 0x7fffffff, 1
    Test_Rd_Rs1_Rs2 8, REG, sra, 0x00ffffff, 0x7fffffff, 7
    Test_Rd_Rs1_Rs2 9, REG, sra, 0x0001ffff, 0x7fffffff, 14
    Test_Rd_Rs1_Rs2 10, REG, sra, 0x00000000, 0x7fffffff, 31
    Test_Rd_Rs1_Rs2 11, REG, sra, 0x81818181, 0x81818181, 0
    Test_Rd_Rs1_Rs2 12, REG, sra, 0xc0c0c0c0, 0x81818181, 1
    Test_Rd_Rs1_Rs2 13, REG, sra, 0xff030303, 0x81818181, 7
    Test_Rd_Rs1_Rs2 14, REG, sra, 0xfffe0606, 0x81818181, 14
    Test_Rd_Rs1_Rs2 15, REG, sra, 0xffffffff, 0x81818181, 31

    # ensure that shifts only use lower 5 bits
    Test_Rd_Rs1_Rs2 16, REG, sra, 0x81818181, 0x81818181, 0xffffffc0
    Test_Rd_Rs1_Rs2 17, REG, sra, 0xc0c0c0c0, 0x81818181, 0xffffffc1
    Test_Rd_Rs1_Rs2 18, REG, sra, 0xff030303, 0x81818181, 0xffffffc7
    Test_Rd_Rs1_Rs2 19, REG, sra, 0xfffe0606, 0x81818181, 0xffffffce
    Test_Rd_Rs1_Rs2 20, REG, sra, 0xffffffff, 0x81818181, 0xffffffff

    # shared src/dst
    Test_Rs1_Rs1_Rs2 21, REG, sra, 0xff000000, 0x80000000, 7
    Test_Rs2_Rs1_Rs2 22, sra, 0xfffe0000, 0x80000000, 14
    Test_Rs1_Rs1_Rs1 23, sra, 0, 7

    # dst bypass
    Test_Rd_Bypass 24, 0, REG, sra, 0xff000000, 0x80000000, 7
    Test_Rd_Bypass 25, 1, REG, sra, 0xfffe0000, 0x80000000, 14
    Test_Rd_Bypass 26, 2, REG, sra, 0xffffffff, 0x80000000, 31

    # src bypass
    Test_Rs1_Bypass 27, 0, 0, REG, sra, 0xff000000, 0x80000000, 7
    Test_Rs1_Bypass 28, 0, 1, REG, sra, 0xfffe0000, 0x80000000, 14
    Test_Rs1_Bypass 29, 0, 2, REG, sra, 0xffffffff, 0x80000000, 31
    Test_Rs1_Bypass 30, 1, 0, REG, sra, 0xff000000, 0x80000000, 7
    Test_Rs1_Bypass 31, 1, 1, REG, sra, 0xfffe0000, 0x80000000, 14
    Test_Rs1_Bypass 32, 2, 0, REG, sra, 0xffffffff, 0x80000000, 31
    Test_Rs2_Bypass 33, 0, 0, sra, 0xff000000, 0x80000000, 7
    Test_Rs2_Bypass 34, 0, 1, sra, 0xfffe0000, 0x80000000, 14
    Test_Rs2_Bypass 35, 0, 2, sra, 0xffffffff, 0x80000000, 31
    Test_Rs2_Bypass 36, 1, 0, sra, 0xff000000, 0x80000000, 7
    Test_Rs2_Bypass 37, 1, 1, sra, 0xfffe0000, 0x80000000, 14
    Test_Rs2_Bypass 38, 2, 0, sra, 0xffffffff, 0x80000000, 31

    # zero register
    Test_Rd_Rs1_Zero 39, sra, 32, 32
    Test_Rd_Zero_Rs2 40, sra, 0, 15
    Test_Rd_Zero_Zero 41, sra, 0
    Test_Zero_Rs1_Rs2 42, REG, sra, 1024, 2048
.endm
