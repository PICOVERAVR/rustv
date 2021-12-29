.macro auipc_tests

# la (same as lla for non-PIC code) (or auipc / addi for clarity?) rd1, 1f
# nop, nop, nop, etc.
# 1f: la rd2, 1
# assert_eq rd1, rd2

Test_Seq auipc, addr_1, "la a0, auipc_test_1; nop; nop; nop; nop; nop; nop; auipc_test_1: la a1, auipc_test_1"

# TEST_CASE(2, a0, 10000, \
#     .align 3; \
#     lla a0, 1f + 10000; \
#     jal a1, 1f; \
#     1: sub a0, a0, a1; \
#   )

#   TEST_CASE(3, a0, -10000, \
#     .align 3; \
#     lla a0, 1f - 10000; \
#     jal a1, 1f; \
#     1: sub a0, a0, a1; \
#   )
.endm
