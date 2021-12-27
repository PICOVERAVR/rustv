.macro or_tests
    # logical tests
    Test_Rd_Rs1_Rs2 1, REG, or, 0xff0fff0f, 0xff00ff00, 0x0f0f0f0f
    Test_Rd_Rs1_Rs2 2, REG, or, 0xfff0fff0, 0x0ff00ff0, 0xf0f0f0f0
    Test_Rd_Rs1_Rs2 3, REG, or, 0x0fff0fff, 0x00ff00ff, 0x0f0f0f0f
    Test_Rd_Rs1_Rs2 4, REG, or, 0xf0fff0ff, 0xf00ff00f, 0xf0f0f0f0

    # shared src/dst
    Test_Rs1_Rs1_Rs2 5, REG, or, 0xff0fff0f, 0xff00ff00, 0x0f0f0f0f
    Test_Rs2_Rs1_Rs2 6, or, 0xff0fff0f, 0xff00ff00, 0x0f0f0f0f
    Test_Rs1_Rs1_Rs1 7, or, 0xff00ff00, 0xff00ff00

    # dst bypass
    Test_Rd_Bypass 8, 0, REG, or, 0xff0fff0f, 0xff00ff00, 0x0f0f0f0f
    Test_Rd_Bypass 9, 1, REG, or, 0xfff0fff0, 0x0ff00ff0, 0xf0f0f0f0
    Test_Rd_Bypass 10, 2, REG, or, 0x0fff0fff, 0x00ff00ff, 0x0f0f0f0f

    # src bypass
    Test_Rs1_Bypass 11, 0, 0, REG, or, 0xff0fff0f, 0xff00ff00, 0x0f0f0f0f
    Test_Rs1_Bypass 12, 0, 1, REG, or, 0xfff0fff0, 0x0ff00ff0, 0xf0f0f0f0
    Test_Rs1_Bypass 13, 0, 2, REG, or, 0x0fff0fff, 0x00ff00ff, 0x0f0f0f0f
    Test_Rs1_Bypass 14, 1, 0, REG, or, 0xff0fff0f, 0xff00ff00, 0x0f0f0f0f
    Test_Rs1_Bypass 15, 1, 1, REG, or, 0xfff0fff0, 0x0ff00ff0, 0xf0f0f0f0
    Test_Rs1_Bypass 16, 2, 0, REG, or, 0x0fff0fff, 0x00ff00ff, 0x0f0f0f0f
    Test_Rs2_Bypass 17, 0, 0, or, 0xff0fff0f, 0xff00ff00, 0x0f0f0f0f
    Test_Rs2_Bypass 18, 0, 1, or, 0xfff0fff0, 0x0ff00ff0, 0xf0f0f0f0
    Test_Rs2_Bypass 19, 0, 2, or, 0x0fff0fff, 0x00ff00ff, 0x0f0f0f0f
    Test_Rs2_Bypass 20, 1, 0, or, 0xff0fff0f, 0xff00ff00, 0x0f0f0f0f
    Test_Rs2_Bypass 21, 1, 1, or, 0xfff0fff0, 0x0ff00ff0, 0xf0f0f0f0
    Test_Rs2_Bypass 22, 2, 0, or, 0x0fff0fff, 0x00ff00ff, 0x0f0f0f0f

    # zero register
    Test_Rd_Rs1_Zero 23, or, 0xff00ff00, 0xff00ff00
    Test_Rd_Zero_Rs2 24, or, 0xff00ff00, 0xff00ff00
    Test_Rd_Zero_Zero 25, or, 0
    Test_Zero_Rs1_Rs2 26, REG, or, 0x11111111, 0x22222222
.endm
