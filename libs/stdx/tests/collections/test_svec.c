#include "stdx/collections/svec.h"
#include "unity.h"

DECLARE_SVEC(int, 16)
DEFINE_SVEC(int, 16)

void setUp(void) {}

void tearDown(void) {}

void test_svec_push(void) {
    // arrange
    svec_int_16 sut ={._size = 0};
    int elem = 42;

    // act
    svec_int_16_push(&sut, elem);

    // assert
    TEST_ASSERT_EQUAL(1, svec_int_16_size(&sut));
    TEST_ASSERT_EQUAL(elem, svec_int_16_data(&sut)[0]);
}
