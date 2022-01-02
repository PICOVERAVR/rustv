.macro jalr_tests

# TODO: check if this is correct!
jalr_jump_1:
    li t0, 1

    li ra, 0
    la a0, 1f
jalr_jump_addr:
    jalr a0
    Assert_ne zero, zero
1:  la a1, jalr_jump_addr
    #addi a1, a1, 4
    Assert_eq ra, a1

# jalr_jump_2:
#     li ra, 0
#     la a0, 1f
#     mv a1, a0
#     jalr a0
#     Assert_ne zero, zero
# 1:  Assert_eq ra, zero
#     Assert_eq a0, a1

# test_2:
#   li  TESTNUM, 2
#   li  x31, 0
#   la  x2, target_2

# linkaddr_2:
#   jalr x19, x2, 0
#   nop
#   nop

#   j fail

# target_2:
#   la  x1, linkaddr_2
#   addi x1, x1, 4
#   bne x1, x19, fail

#   #-------------------------------------------------------------
#   # Test 3: Check r0 target and that r31 is not modified
#   #-------------------------------------------------------------

# test_3:
#   li  TESTNUM, 3
#   li  x31, 0
#   la  x3, target_3

# linkaddr_3:
#   jalr x0, x3, 0
#   nop

#   j fail

# target_3:
#   bne x31, x0, fail

#   #-------------------------------------------------------------
#   # Bypassing tests
#   #-------------------------------------------------------------

#   TEST_JALR_SRC1_BYPASS( 4, 0, jalr );
#   TEST_JALR_SRC1_BYPASS( 5, 1, jalr );
#   TEST_JALR_SRC1_BYPASS( 6, 2, jalr );

#   #-------------------------------------------------------------
#   # Test delay slot instructions not executed nor bypassed
#   #-------------------------------------------------------------

#   TEST_CASE( 7, x1, 4, \
#     li  x1, 1; \
#     la  x2, 1f;
#     jalr x19, x2, -4; \
#     addi x1, x1, 1; \
#     addi x1, x1, 1; \
#     addi x1, x1, 1; \
#     addi x1, x1, 1; \
# 1:  addi x1, x1, 1; \
#     addi x1, x1, 1; \
#   )
.endm
