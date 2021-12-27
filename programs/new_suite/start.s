.include "test_macros.s"

.include "add.s"
.include "addi.s"
.include "and.s"

.text
.global _start
_start:

    add_tests
    addi_tests
    and_tests

    Stop
    