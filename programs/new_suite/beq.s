.macro beq_tests
    Test_Branch_Taken beq, taken_1, beq, 0, 0
    Test_Branch_Taken beq, taken_2, beq, 1, 1
    Test_Branch_Taken beq, taken_3, beq, -1, -1

    Test_Branch_Not_Taken beq, taken_4, beq, 0, 1
    Test_Branch_Not_Taken beq, taken_5, beq, 1, 0
    Test_Branch_Not_Taken beq, taken_6, beq, -1, 1
    Test_Branch_Not_Taken beq, taken_7, beq, 1, -1

#   TEST_BR2_SRC12_BYPASS( 9,  0, 0, beq, 0, -1 );
#   TEST_BR2_SRC12_BYPASS( 10, 0, 1, beq, 0, -1 );
#   TEST_BR2_SRC12_BYPASS( 11, 0, 2, beq, 0, -1 );
#   TEST_BR2_SRC12_BYPASS( 12, 1, 0, beq, 0, -1 );
#   TEST_BR2_SRC12_BYPASS( 13, 1, 1, beq, 0, -1 );
#   TEST_BR2_SRC12_BYPASS( 14, 2, 0, beq, 0, -1 );

#   TEST_BR2_SRC12_BYPASS( 15, 0, 0, beq, 0, -1 );
#   TEST_BR2_SRC12_BYPASS( 16, 0, 1, beq, 0, -1 );
#   TEST_BR2_SRC12_BYPASS( 17, 0, 2, beq, 0, -1 );
#   TEST_BR2_SRC12_BYPASS( 18, 1, 0, beq, 0, -1 );
#   TEST_BR2_SRC12_BYPASS( 19, 1, 1, beq, 0, -1 );
#   TEST_BR2_SRC12_BYPASS( 20, 2, 0, beq, 0, -1 );

#   #-------------------------------------------------------------
#   # Test delay slot instructions not executed nor bypassed
#   #-------------------------------------------------------------

#   TEST_CASE( 21, x1, 3, \
#     li  x1, 1; \
#     beq x0, x0, 1f; \
#     addi x1, x1, 1; \
#     addi x1, x1, 1; \
#     addi x1, x1, 1; \
#     addi x1, x1, 1; \
# 1:  addi x1, x1, 1; \
#     addi x1, x1, 1; \
#   )
.endm
