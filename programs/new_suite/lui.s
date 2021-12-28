.macro lui_tests
    # NOTE: these tests differ somewhat from riscv_tests!

    # TODO: need a generic macro here...
    # Test_Seq 1, IMM, lui, 0x00000000, 0x00000000, 0x00000
    # Test_Seq 2, IMM, lui, 0xfffff000, 0x00000000, 0xfffff
    # Test_Seq 3, IMM, lui, 0x00000fff, 0x00000fff, 0x0
    # Test_Seq 4, IMM, lui, 0xffffffff, 0x00000fff, 0xfffff
    # Test_Seq 5, IMM, lui, 0x7fffe7fe, 0x000007fe, 0x7fffe
    # Test_Seq 6, IMM, lui, 0x8ffff800, 0x00000800, 0x8ffff
    # Test_Seq 7, IMM, lui, 0x00005800, 0x00000800, 0x00005

    # shared src/dst??
    # Test_Rs1 8, IMM, lui, 0x0a005800, 0x00000800, 0x0a005

    # # dst bypass??
    # Test_Rd_Bypass 8, 0, IMM, lui, 0x00005800, 0x00000800, 0x00005
    # Test_Rd_Bypass 9, 1, IMM, lui, 0x000100f0, 0x000000f0, 0x00010
    # Test_Rd_Bypass 10, 2, IMM, lui, 0x00015045, 0x00000045, 0x00015

    # # zero register
    # Test_Rd_Zero_Imm 6, lui, 0xfafdd000, 0xfafdd
    # Test_Zero 5, IMM, lui, 0, 0x00000000, 0x80000
.endm
