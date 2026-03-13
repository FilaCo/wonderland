#include "Wonderland/IPS/Registry.h"
#include <gtest/gtest.h>

using namespace Wonderland::IPS;

TEST(RegistryTest, ItSpawnsEntity) {
  // arrange
  auto SUT = Registry();
  auto Expected = 0;

  // act
  auto Actual = SUT.spawn();

  // assert
  EXPECT_EQ(Expected, Actual);
  ASSERT_TRUE(SUT.isAlive(Actual));
}

TEST(RegistryTest, ItDespawnsEntity) {
  // arrange
  auto SUT = Registry();
  auto AliveEntity = SUT.spawn();
  auto DeadEntity = SUT.spawn();

  // act
  SUT.despawn(DeadEntity);

  // assert
  EXPECT_TRUE(SUT.isAlive(AliveEntity));
  ASSERT_TRUE(SUT.isDead(DeadEntity));
}

TEST(RegistryTest, ItRecyclesEntities) {
  // arrange
  auto SUT = Registry();

  // act
  for (auto i = 0; i < 10; ++i) {
    SUT.spawn();
  }

  for (uint32_t i = 0; i < 10; i += 2) {
    SUT.despawn(i);
  }

  // assert
  for (uint32_t i = 0; i < 10; ++i) {
    EXPECT_EQ(i & 1, SUT.isAlive(i));
  }

  for (uint32_t i = 0; i < 10; i += 2) {
    EXPECT_EQ((8 - i) | (1 << 20), SUT.spawn());
  }
}
