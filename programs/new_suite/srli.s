.macro srli_tests
    # logical
    Test_Rd_Rs1_Rs2 1, IMM, srli, 0xffff8000, 0xffff8000, 0
    Test_Rd_Rs1_Rs2 2, IMM, srli, 0x7fffc000, 0xffff8000, 1
    Test_Rd_Rs1_Rs2 3, IMM, srli, 0x01ffff00, 0xffff8000, 7
    Test_Rd_Rs1_Rs2 4, IMM, srli, 0x0003fffe, 0xffff8000, 14
    Test_Rd_Rs1_Rs2 5, IMM, srli, 0x0001ffff, 0xffff8001, 15
    Test_Rd_Rs1_Rs2 6, IMM, srli, 0xffffffff, 0xffffffff, 0
    Test_Rd_Rs1_Rs2 7, IMM, srli, 0x7fffffff, 0xffffffff, 1
    Test_Rd_Rs1_Rs2 8, IMM, srli, 0x01ffffff, 0xffffffff, 7
    Test_Rd_Rs1_Rs2 9, IMM, srli, 0x0003ffff, 0xffffffff, 14
    Test_Rd_Rs1_Rs2 10, IMM, srli, 0x00000001, 0xffffffff, 31
    Test_Rd_Rs1_Rs2 11, IMM, srli, 0x21212121, 0x21212121, 0
    Test_Rd_Rs1_Rs2 12, IMM, srli, 0x10909090, 0x21212121, 1
    Test_Rd_Rs1_Rs2 13, IMM, srli, 0x00424242, 0x21212121, 7
    Test_Rd_Rs1_Rs2 14, IMM, srli, 0x00008484, 0x21212121, 14
    Test_Rd_Rs1_Rs2 15, IMM, srli, 0x00000000, 0x21212121, 31

    # shared src/dst
    Test_Rs1_Rs1_Rs2 16, IMM, srli, 0x7fffc000, 0xffff8000, 1

    # dst bypass
    Test_Rd_Bypass 17, 0, IMM, srli, 0x7fffc000, 0xffff8000, 1
    Test_Rd_Bypass 18, 1, IMM, srli, 0x0003fffe, 0xffff8000, 14
    Test_Rd_Bypass 19, 2, IMM, srli, 0x0001ffff, 0xffff8000, 15

    # src bypass
    Test_Rs1_Bypass 20, 0, -1, IMM, srli, 0x7fffc000, 0xffff8000, 1
    Test_Rs1_Bypass 21, 1, -1, IMM, srli, 0x0003fffe, 0xffff8000, 14
    Test_Rs1_Bypass 22, 2, -1, IMM, srli, 0x0001ffff, 0xffff8000, 15

    # zero register
    Test_Rd_Zero_Imm 23, srli, 0, 31
    Test_Zero_Rs1_Rs2 24, IMM, srli, 33, 20
.endm
