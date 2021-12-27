.macro add_tests
    # arithmetic
    Test_Rd_Rs1_Rs2 1, REG, add, 0x00000000, 0x00000000, 0x00000000
    Test_Rd_Rs1_Rs2 2, REG, add, 0x00000002, 0x00000001, 0x00000001
    Test_Rd_Rs1_Rs2 3, REG, add, 0x0000000a, 0x00000003, 0x00000007
    Test_Rd_Rs1_Rs2 4, REG, add, 0xffff8000, 0x00000000, 0xffff8000
    Test_Rd_Rs1_Rs2 5, REG, add, 0x80000000, 0x80000000, 0x00000000
    Test_Rd_Rs1_Rs2 6, REG, add, 0x7fff8000, 0x80000000, 0xffff8000
    Test_Rd_Rs1_Rs2 7, REG, add, 0x00007fff, 0x00000000, 0x00007fff
    Test_Rd_Rs1_Rs2 8, REG, add, 0x7fffffff, 0x7fffffff, 0x00000000
    Test_Rd_Rs1_Rs2 9, REG, add, 0x80007ffe, 0x7fffffff, 0x00007fff
    Test_Rd_Rs1_Rs2 10, REG, add, 0x80007fff, 0x80000000, 0x00007fff
    Test_Rd_Rs1_Rs2 11, REG, add, 0x7fff7fff, 0x7fffffff, 0xffff8000
    Test_Rd_Rs1_Rs2 12, REG, add, 0xffffffff, 0x00000000, 0xffffffff
    Test_Rd_Rs1_Rs2 13, REG, add, 0x00000000, 0xffffffff, 0x00000001
    Test_Rd_Rs1_Rs2 14, REG, add, 0xfffffffe, 0xffffffff, 0xffffffff
    Test_Rd_Rs1_Rs2 15, REG, add, 0x80000000, 0x00000001, 0x7fffffff

    # shared src/dst
    Test_Rs1_Rs1_Rs2 16, REG, add, 24, 13, 11
    Test_Rs2_Rs1_Rs2 17, add, 25, 14, 11
    Test_Rs1_Rs1_Rs1 18, add, 26, 13

    # dst bypass
    Test_Rd_Bypass 19, 0, REG, add, 24, 13, 11
    Test_Rd_Bypass 20, 1, REG, add, 25, 14, 11
    Test_Rd_Bypass 21, 2, REG, add, 26, 15, 11

    # src bypass
    Test_Rs1_Bypass 22, 0, 0, REG, add, 24, 13, 11
    Test_Rs1_Bypass 23, 0, 1, REG, add, 25, 14, 11
    Test_Rs1_Bypass 24, 0, 2, REG, add, 26, 15, 11
    Test_Rs1_Bypass 25, 1, 0, REG, add, 24, 13, 11
    Test_Rs1_Bypass 26, 1, 1, REG, add, 25, 14, 11
    Test_Rs1_Bypass 27, 2, 0, REG, add, 26, 15, 11
    Test_Rs2_Bypass 28, 0, 0, add, 24, 13, 11
    Test_Rs2_Bypass 29, 0, 1, add, 25, 14, 11
    Test_Rs2_Bypass 30, 0, 2, add, 26, 15, 11
    Test_Rs2_Bypass 31, 1, 0, add, 24, 13, 11
    Test_Rs2_Bypass 32, 1, 1, add, 25, 14, 11
    Test_Rs2_Bypass 33, 2, 0, add, 26, 15, 11

    # zero register
    Test_Rd_Rs1_Zero 34, add, 15, 15
    Test_Rd_Zero_Rs2 35, add, 32, 32
    Test_Rd_Zero_Zero 36, add, 0
    Test_Zero_Rs1_Rs2 37, REG, add, 16, 30
    
.endm
