.macro and_tests
    # logical
    Test_Rd_Rs1_Rs2 1, REG, and, 0x0f000f00, 0xff00ff00, 0x0f0f0f0f
    Test_Rd_Rs1_Rs2 2, REG, and, 0x00f000f0, 0x0ff00ff0, 0xf0f0f0f0
    Test_Rd_Rs1_Rs2 3, REG, and, 0x000f000f, 0x00ff00ff, 0x0f0f0f0f
    Test_Rd_Rs1_Rs2 4, REG, and, 0xf000f000, 0xf00ff00f, 0xf0f0f0f0

    # shared src/dst
    Test_Rs1_Rs1_Rs2 5, REG, and, 0x0f000f00, 0xff00ff00, 0x0f0f0f0f
    Test_Rs2_Rs1_Rs2 6, and, 0x00f000f0, 0x0ff00ff0, 0xf0f0f0f0
    Test_Rs1_Rs1_Rs1 7, and, 0xff00ff00, 0xff00ff00

    # dst bypass
    Test_Rd_Bypass 8, 0, REG, and, 0x0f000f00, 0xff00ff00, 0x0f0f0f0f
    Test_Rd_Bypass 9, 1, REG, and, 0x00f000f0, 0x0ff00ff0, 0xf0f0f0f0
    Test_Rd_Bypass 10, 2, REG, and, 0x000f000f, 0x00ff00ff, 0x0f0f0f0f

    # src bypass
    Test_Rs_Bypass 11, 0, 0, REG, and, 0x0f000f00, 0xff00ff00, 0x0f0f0f0f
    Test_Rs_Bypass 12, 0, 1, REG, and, 0x00f000f0, 0x0ff00ff0, 0xf0f0f0f0
    Test_Rs_Bypass 13, 0, 2, REG, and, 0x000f000f, 0x00ff00ff, 0x0f0f0f0f
    Test_Rs_Bypass 14, 1, 0, REG, and, 0x0f000f00, 0xff00ff00, 0x0f0f0f0f
    Test_Rs_Bypass 15, 1, 1, REG, and, 0x00f000f0, 0x0ff00ff0, 0xf0f0f0f0
    Test_Rs_Bypass 16, 2, 0, REG, and, 0x000f000f, 0x00ff00ff, 0x0f0f0f0f
    Test_Rs_Bypass 17, 0, 0, REG, and, 0x0f000f00, 0x0f0f0f0f, 0xff00ff00
    Test_Rs_Bypass 18, 0, 1, REG, and, 0x00f000f0, 0xf0f0f0f0, 0x0ff00ff0
    Test_Rs_Bypass 19, 0, 2, REG, and, 0x000f000f, 0x0f0f0f0f, 0x00ff00ff
    Test_Rs_Bypass 20, 1, 0, REG, and, 0x0f000f00, 0x0f0f0f0f, 0xff00ff00
    Test_Rs_Bypass 21, 1, 1, REG, and, 0x00f000f0, 0xf0f0f0f0, 0x0ff00ff0
    Test_Rs_Bypass 22, 2, 0, REG, and, 0x000f000f, 0x0f0f0f0f, 0x00ff00ff

    # zero register
    Test_Rd_Rs1_Zero 23, and, 0, 0xff00ff00
    Test_Rd_Zero_Rs2 24, and, 0, 0x00ff00ff
    Test_Rd_Zero_Zero 25, and, 0
    Test_Zero_Rs1_Rs2 26, REG, and, 0x11111111, 0x22222222
.endm
