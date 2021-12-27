.macro andi_tests
    # logical
    Test_Rd_Rs1_Rs2 1, IMM, andi, 0xff00ff00, 0xff00ff00, -241 # 0xf0f
    Test_Rd_Rs1_Rs2 2, IMM, andi, 0x000000f0, 0x0ff00ff0, 0x0f0
    Test_Rd_Rs1_Rs2 3, IMM, andi, 0x0000000f, 0x00ff00ff, 0x70f
    Test_Rd_Rs1_Rs2 4, IMM, andi, 0x00000000, 0xf00ff00f, 0x0f0

    # shared src/dst
    Test_Rs1_Rs1_Rs2 5, IMM, andi, 0x00000000, 0xff00ff00, 0x0f0

    # dst bypass
    Test_Rd_Bypass 6, 0, IMM, andi, 0x00000700, 0x0ff00ff0, 0x70f
    Test_Rd_Bypass 7, 1, IMM, andi, 0x000000f0, 0x00ff00ff, 0x0f0
    Test_Rd_Bypass 8, 2, IMM, andi, 0xf00ff00f, 0xf00ff00f, -241 # 0xf0f

    # src bypass
    Test_Rs1_Bypass 9, 0, -1, IMM, andi, 0x00000700, 0x0ff00ff0, 0x70f
    Test_Rs1_Bypass 10, 1, -1, IMM, andi, 0x000000f0, 0x00ff00ff, 0x0f0
    Test_Rs1_Bypass 11, 2, -1, IMM, andi, 0x0000000f, 0xf00ff00f, 0x70f

    # zero register
    Test_Rd_Zero_Imm 12, andi, 0, 0x0f0
    Test_Zero_Rs1_Rs2 13, IMM, andi, 0x00ff00ff, 0x70f
.endm
