.include "test_macros.s"

# use tests for all rv32i instructions
.include "rv32i.s"

.text
.global _start
_start:

    add_tests
    addi_tests
    sub_tests
    and_tests
    andi_tests
    or_tests
    ori_tests
    xor_tests
    xori_tests

    Stop
    