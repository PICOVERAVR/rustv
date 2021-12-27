.macro xor_tests
    # logical
    Test_Rd_Rs1_Rs2 1, REG, xor, 0xf00ff00f, 0xff00ff00, 0x0f0f0f0f
    Test_Rd_Rs1_Rs2 2, REG, xor, 0xff00ff00, 0x0ff00ff0, 0xf0f0f0f0
    Test_Rd_Rs1_Rs2 3, REG, xor, 0x0ff00ff0, 0x00ff00ff, 0x0f0f0f0f
    Test_Rd_Rs1_Rs2 4, REG, xor, 0x00ff00ff, 0xf00ff00f, 0xf0f0f0f0

    # shared src/dst
    Test_Rs1_Rs1_Rs2 5, REG, xor, 0xf00ff00f, 0xff00ff00, 0x0f0f0f0f
    Test_Rs2_Rs1_Rs2 6, xor, 0xf00ff00f, 0xff00ff00, 0x0f0f0f0f
    Test_Rs1_Rs1_Rs1 7, xor, 0x00000000, 0xff00ff00

    # dst bypass
    Test_Rd_Bypass 8, 0, REG, xor, 0xf00ff00f, 0xff00ff00, 0x0f0f0f0f
    Test_Rd_Bypass 9, 1, REG, xor, 0xff00ff00, 0x0ff00ff0, 0xf0f0f0f0
    Test_Rd_Bypass 10, 2, REG, xor, 0x0ff00ff0, 0x00ff00ff, 0x0f0f0f0f

    # src bypass
    Test_Rs1_Bypass 11, 0, 0, REG, xor, 0xf00ff00f, 0xff00ff00, 0x0f0f0f0f
    Test_Rs1_Bypass 12, 0, 1, REG, xor, 0xff00ff00, 0x0ff00ff0, 0xf0f0f0f0
    Test_Rs1_Bypass 13, 0, 2, REG, xor, 0x0ff00ff0, 0x00ff00ff, 0x0f0f0f0f
    Test_Rs1_Bypass 14, 1, 0, REG, xor, 0xf00ff00f, 0xff00ff00, 0x0f0f0f0f
    Test_Rs1_Bypass 15, 1, 1, REG, xor, 0xff00ff00, 0x0ff00ff0, 0xf0f0f0f0
    Test_Rs1_Bypass 16, 2, 0, REG, xor, 0x0ff00ff0, 0x00ff00ff, 0x0f0f0f0f
    Test_Rs2_Bypass 17, 0, 0, xor, 0xf00ff00f, 0xff00ff00, 0x0f0f0f0f
    Test_Rs2_Bypass 18, 0, 1, xor, 0xff00ff00, 0x0ff00ff0, 0xf0f0f0f0
    Test_Rs2_Bypass 19, 0, 2, xor, 0x0ff00ff0, 0x00ff00ff, 0x0f0f0f0f
    Test_Rs2_Bypass 20, 1, 0, xor, 0xf00ff00f, 0xff00ff00, 0x0f0f0f0f
    Test_Rs2_Bypass 21, 1, 1, xor, 0xff00ff00, 0x0ff00ff0, 0xf0f0f0f0
    Test_Rs2_Bypass 22, 2, 0, xor, 0x0ff00ff0, 0x00ff00ff, 0x0f0f0f0f

    # zero register
    Test_Rd_Rs1_Zero 23, xor, 0xff00ff00, 0xff00ff00
    Test_Rd_Zero_Rs2 24, xor, 0x00ff00ff, 0x00ff00ff
    Test_Rd_Zero_Zero 25, xor, 0
    Test_Zero_Rs1_Rs2 26, REG, xor, 0x11111111, 0x22222222
.endm
