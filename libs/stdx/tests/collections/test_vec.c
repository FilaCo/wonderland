#include "stdx/alloc/xalloc.h"
#include "stdx/collections/vec.h"
#include "unity.h"
#include <stdlib.h>

DEFINE_VEC(int)

void *test_alloc([[maybe_unused]] void *ctx, void *ptr,
                 [[maybe_unused]] size_t osize, size_t nsize) {
  return realloc(ptr, nsize);
}

void setUp(void) {}

void tearDown(void) {}

void it_creates_new_vec(void) {
  // arrange
  vec_int sut;

  // act
  sut = vec_int_new(test_alloc);

  // assert
  TEST_ASSERT_NOT_NULL(sut);
  TEST_ASSERT_EQUAL(0, vec_int_size(sut));
  TEST_ASSERT_EQUAL(0, vec_int_cap(sut));

  // cleanup
  vec_int_free(sut);
}

void it_pushes_item_to_vec(void) {
  // arrange
  vec_int sut = vec_int_new(test_alloc);
  int elem = 42;

  // act
  sut = vec_int_push(sut, elem);

  // assert
  TEST_ASSERT_EQUAL(1, vec_int_size(sut));
  TEST_ASSERT_EQUAL(1, vec_int_cap(sut));
  TEST_ASSERT_EQUAL(elem, sut[0]);

  // cleanup
  vec_int_free(sut);
}

void it_pops_item_from_vec(void) {
  // arrange
  vec_int sut = vec_int_new(test_alloc);
  int elem0 = 42, elem1 = 123, elem2 = -100500;
  sut = vec_int_push(sut, elem0);
  sut = vec_int_push(sut, elem1);
  sut = vec_int_push(sut, elem2);

  // act
  int *actual = vec_int_pop(sut);

  // assert
  TEST_ASSERT_EQUAL(2, vec_int_size(sut));
  TEST_ASSERT_EQUAL(4, vec_int_cap(sut));
  TEST_ASSERT_EQUAL(elem0, sut[0]);
  TEST_ASSERT_EQUAL(elem1, sut[1]);
  TEST_ASSERT_EQUAL(elem2, *actual);

  // cleanup
  vec_int_free(sut);
}

void it_pushes_items_to_vec_until_it_grows(void) {
  // arrange
  vec_int sut = vec_int_new(test_alloc);

  // act
  for (int i = 0; i < 10; ++i) {
    sut = vec_int_push(sut, i);
  }

  // assert
  TEST_ASSERT_EQUAL(10, vec_int_size(sut));
  TEST_ASSERT_EQUAL(16, vec_int_cap(sut));
  for (int i = 0; i < 10; ++i) {
    TEST_ASSERT_EQUAL(i, sut[i]);
  }

  // cleanup
  vec_int_free(sut);
}
