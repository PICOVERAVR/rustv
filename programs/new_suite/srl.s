.macro srl_tests
    # logical
    Test_Rd_Rs1_Rs2 1, REG, srl, 0xffff8000, 0xffff8000, 0
    Test_Rd_Rs1_Rs2 2, REG, srl, 0x7fffc000, 0xffff8000, 1
    Test_Rd_Rs1_Rs2 3, REG, srl, 0x01ffff00, 0xffff8000, 7
    Test_Rd_Rs1_Rs2 4, REG, srl, 0x0003fffe, 0xffff8000, 14
    Test_Rd_Rs1_Rs2 5, REG, srl, 0x0001ffff, 0xffff8001, 15
    Test_Rd_Rs1_Rs2 6, REG, srl, 0xffffffff, 0xffffffff, 0
    Test_Rd_Rs1_Rs2 7, REG, srl, 0x7fffffff, 0xffffffff, 1
    Test_Rd_Rs1_Rs2 8, REG, srl, 0x01ffffff, 0xffffffff, 7
    Test_Rd_Rs1_Rs2 9, REG, srl, 0x0003ffff, 0xffffffff, 14
    Test_Rd_Rs1_Rs2 10, REG, srl, 0x00000001, 0xffffffff, 31
    Test_Rd_Rs1_Rs2 11, REG, srl, 0x21212121, 0x21212121, 0
    Test_Rd_Rs1_Rs2 12, REG, srl, 0x10909090, 0x21212121, 1
    Test_Rd_Rs1_Rs2 13, REG, srl, 0x00424242, 0x21212121, 7
    Test_Rd_Rs1_Rs2 14, REG, srl, 0x00008484, 0x21212121, 14
    Test_Rd_Rs1_Rs2 15, REG, srl, 0x00000000, 0x21212121, 31

    # ensure that shifts only use lower 5 bits
    Test_Rd_Rs1_Rs2 16, REG, srl, 0x21212121, 0x21212121, 0xffffffe0
    Test_Rd_Rs1_Rs2 17, REG, srl, 0x10909090, 0x21212121, 0xffffffe1
    Test_Rd_Rs1_Rs2 18, REG, srl, 0x00424242, 0x21212121, 0xffffffe7
    Test_Rd_Rs1_Rs2 19, REG, srl, 0x00008484, 0x21212121, 0xffffffee
    Test_Rd_Rs1_Rs2 20, REG, srl, 0x00000000, 0x21212121, 0xffffffff

    # shared src/dst
    Test_Rs1_Rs1_Rs2 21, REG, srl, 0x7fffc000, 0xffff8000, 1
    Test_Rs2_Rs1_Rs2 22, srl, 0x0003fffe, 0xffff8000, 14
    Test_Rs1_Rs1_Rs1 23, srl, 0, 7

    # dst bypass
    Test_Rd_Bypass 24, 0, REG, srl, 0x7fffc000, 0xffff8000, 1
    Test_Rd_Bypass 25, 1, REG, srl, 0x0003fffe, 0xffff8000, 14
    Test_Rd_Bypass 26, 2, REG, srl, 0x0001ffff, 0xffff8000, 15

    # src bypass
    Test_Rs1_Bypass 27, 0, 0, REG, srl, 0x7fffc000, 0xffff8000, 1
    Test_Rs1_Bypass 28, 0, 1, REG, srl, 0x01ffff00, 0xffff8000, 7
    Test_Rs1_Bypass 29, 0, 2, REG, srl, 0x0001ffff, 0xffff8000, 15
    Test_Rs1_Bypass 30, 1, 0, REG, srl, 0x7fffc000, 0xffff8000, 1
    Test_Rs1_Bypass 31, 1, 1, REG, srl, 0x01ffff00, 0xffff8000, 7
    Test_Rs1_Bypass 32, 2, 0, REG, srl, 0x0001ffff, 0xffff8000, 15
    Test_Rs2_Bypass 33, 0, 0, srl, 0x7fffc000, 0xffff8000, 1
    Test_Rs2_Bypass 34, 0, 1, srl, 0x01ffff00, 0xffff8000, 7
    Test_Rs2_Bypass 35, 0, 2, srl, 0x0001ffff, 0xffff8000, 15
    Test_Rs2_Bypass 36, 1, 0, srl, 0x7fffc000, 0xffff8000, 1
    Test_Rs2_Bypass 37, 1, 1, srl, 0x01ffff00, 0xffff8000, 7
    Test_Rs2_Bypass 38, 2, 0, srl, 0x0001ffff, 0xffff8000, 15

    # zero register
    Test_Rd_Rs1_Zero 39, srl, 32, 32
    Test_Rd_Zero_Rs2 40, srl, 0, 15
    Test_Rd_Zero_Zero 41, srl, 0
    Test_Zero_Rs1_Rs2 42, REG, srl, 1024, 2048
.endm
