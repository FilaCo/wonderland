#include "Wonderland/ECS/SparseSet.hpp"
#include <gtest/gtest.h>

using namespace Wonderland::ECS;

TEST(SparseSetTest, ItInsertsEntity) {
  // arrange
  auto SUT = SparseSet();
  auto Target = 0;

  // act
  EXPECT_TRUE(SUT.insert(Target));

  // assert
  EXPECT_TRUE(SUT.has(Target));
}

TEST(SparseSetTest, ItRemovesEntity) {
  // arrange
  auto SUT = SparseSet();
  auto Target = 1;
  SUT.insert(0);
  SUT.insert(Target);
  SUT.insert(123);

  // act
  EXPECT_TRUE(SUT.erase(Target));

  // assert
  EXPECT_FALSE(SUT.has(Target));
}

TEST(SparseSetTest, ItKeepsOtherEntitiesAccessibleAfterErase) {
  // arrange
  auto SUT = SparseSet();
  SUT.insert(0);
  SUT.insert(1);
  SUT.insert(
      123); // becomes last in Dense — will be swap-and-popped on erase(1)

  // act
  SUT.erase(1);

  // assert
  EXPECT_FALSE(SUT.has(1));
  EXPECT_TRUE(SUT.has(0));
  EXPECT_TRUE(SUT.has(123)); // fails without the Sparse[LastEntity] update
}