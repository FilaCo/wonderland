#include "Wonderland/ECS/PropPool.hpp"
#include <gmock/gmock.h>
#include <gtest/gtest.h>
#include <optional>

using namespace Wonderland::ECS;

TEST(PropPoolTest, ItInsertsId) {
  // arrange
  auto SUT = PropPool<int>();
  auto Target = 0;

  // act
  EXPECT_EQ(std::nullopt, SUT.insert(Target, 0));

  // assert
  EXPECT_TRUE(SUT.has(Target));
}

TEST(PropPoolTest, ItRemovesId) {
  // arrange
  auto SUT = PropPool<int>();
  auto Target = 1;
  SUT.insert(0, 0);
  SUT.insert(Target, Target);
  SUT.insert(123, 123);

  // act
  EXPECT_EQ(std::optional(Target), SUT.erase(Target));

  // assert
  EXPECT_FALSE(SUT.has(Target));
  EXPECT_TRUE(SUT.has(0));
  EXPECT_TRUE(SUT.has(123));
}
