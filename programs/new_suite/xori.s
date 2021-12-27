.macro xori_tests
    # logical tests
    Test_Rd_Rs1_Rs2 1, IMM, xori, 0xff00f00f, 0x00ff0f00, -241 # 0xf0f
    Test_Rd_Rs1_Rs2 2, IMM, xori, 0x0ff00f00, 0x0ff00ff0, 0x0f0
    Test_Rd_Rs1_Rs2 3, IMM, xori, 0x00ff0ff0, 0x00ff08ff, 0x70f
    Test_Rd_Rs1_Rs2 4, IMM, xori, 0xf00ff0ff, 0xf00ff00f, 0x0f0

    # shared src/dst
    Test_Rs1_Rs1_Rs2 5, IMM, xori, 0xff00f00f, 0xff00f700, 0x70f

    # dst bypass
    Test_Rd_Bypass 6, 0, IMM, xori, 0x0ff00f00, 0x0ff00ff0, 0x0f0
    Test_Rd_Bypass 7, 1, IMM, xori, 0x00ff0ff0, 0x00ff08ff, 0x70f
    Test_Rd_Bypass 8, 2, IMM, xori, 0xf00ff0ff, 0xf00ff00f, 0x0f0

    # src bypass
    Test_Rs1_Bypass 9, 0, -1, IMM, xori, 0x0ff00f00, 0x0ff00ff0, 0x0f0
    Test_Rs1_Bypass 10, 1, -1, IMM, xori, 0x00ff0ff0, 0x00ff0fff, 0x00f
    Test_Rs1_Bypass 11, 2, -1, IMM, xori, 0xf00ff0ff, 0xf00ff00f, 0x0f0

    # zero register
    Test_Rd_Zero_Imm 12, xori, 0x0f0, 0x0f0
    Test_Zero_Rs1_Rs2 13, IMM, xori, 0x00ff00ff, 0x70f
.endm
