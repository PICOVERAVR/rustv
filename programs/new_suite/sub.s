.macro sub_tests
    # arithmetic
    Test_Rd_Rs1_Rs2 1, REG, sub, 0x00000000, 0x00000000, 0x00000000
    Test_Rd_Rs1_Rs2 2, REG, sub, 0x00000000, 0x00000001, 0x00000001
    Test_Rd_Rs1_Rs2 3, REG, sub, 0xfffffffc, 0x00000003, 0x00000007
    Test_Rd_Rs1_Rs2 4, REG, sub, 0x00008000, 0x00000000, 0xffff8000
    Test_Rd_Rs1_Rs2 5, REG, sub, 0x80000000, 0x80000000, 0x00000000
    Test_Rd_Rs1_Rs2 6, REG, sub, 0x80008000, 0x80000000, 0xffff8000
    Test_Rd_Rs1_Rs2 7, REG, sub, 0xffff8001, 0x00000000, 0x00007fff
    Test_Rd_Rs1_Rs2 8, REG, sub, 0x7fffffff, 0x7fffffff, 0x00000000
    Test_Rd_Rs1_Rs2 9, REG, sub, 0x7fff8000, 0x7fffffff, 0x00007fff
    Test_Rd_Rs1_Rs2 10, REG, sub, 0x7fff8001, 0x80000000, 0x00007fff
    Test_Rd_Rs1_Rs2 11, REG, sub, 0x80007fff, 0x7fffffff, 0xffff8000
    Test_Rd_Rs1_Rs2 12, REG, sub, 0x00000001, 0x00000000, 0xffffffff
    Test_Rd_Rs1_Rs2 13, REG, sub, 0xfffffffe, 0xffffffff, 0x00000001
    Test_Rd_Rs1_Rs2 14, REG, sub, 0x00000000, 0xffffffff, 0xffffffff

    # shared src/dst
    Test_Rs1_Rs1_Rs2 15, REG, sub, 2, 13, 11
    Test_Rs2_Rs1_Rs2 16, sub, 3, 14, 11
    Test_Rs1_Rs1_Rs1 17, sub, 0, 13

    # dst bypass
    Test_Rd_Bypass 18, 0, REG, sub, 2, 13, 11
    Test_Rd_Bypass 19, 1, REG, sub, 3, 14, 11
    Test_Rd_Bypass 20, 2, REG, sub, 4, 15, 11

    # src bypass
    Test_Rs1_Bypass 21, 0, 0, REG, sub, 2, 13, 11
    Test_Rs1_Bypass 22, 0, 1, REG, sub, 3, 14, 11
    Test_Rs1_Bypass 23, 0, 2, REG, sub, 4, 15, 11
    Test_Rs1_Bypass 24, 1, 0, REG, sub, 2, 13, 11
    Test_Rs1_Bypass 25, 1, 1, REG, sub, 3, 14, 11
    Test_Rs1_Bypass 26, 2, 0, REG, sub, 4, 15, 11
    Test_Rs2_Bypass 27, 0, 0, sub, 2, 13, 11
    Test_Rs2_Bypass 28, 0, 1, sub, 3, 14, 11
    Test_Rs2_Bypass 29, 0, 2, sub, 4, 15, 11
    Test_Rs2_Bypass 30, 1, 0, sub, 2, 13, 11
    Test_Rs2_Bypass 31, 1, 1, sub, 3, 14, 11
    Test_Rs2_Bypass 32, 2, 0, sub, 4, 15, 11

    # zero register
    Test_Rd_Rs1_Zero 33, sub, 15, 15
    Test_Rd_Zero_Rs2 34, sub, 32, -32
    Test_Rd_Zero_Zero 35, sub, 0
    Test_Zero_Rs1_Rs2 36, REG, sub, 16, 30
.endm
