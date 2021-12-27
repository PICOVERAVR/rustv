.include "test_macros.s"

.include "add.s"
.include "addi.s"
.include "and.s"
.include "andi.s"

.text
.global _start
_start:

    add_tests
    addi_tests
    and_tests
    andi_tests

    Stop
    