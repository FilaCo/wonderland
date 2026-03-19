#include "Wonderland/ECS/Registry.hpp"
#include <gtest/gtest.h>

using namespace Wonderland::ECS;

TEST(RegistryTest, ItSpawnsEntity) {
  // arrange
  auto SUT = Registry();
  auto Expected = 0;

  // act
  auto Actual = SUT.spawn();

  // assert
  EXPECT_EQ(Expected, Actual);
  EXPECT_TRUE(SUT.isAlive(Actual));
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
  EXPECT_TRUE(SUT.isDead(DeadEntity));
}

TEST(RegistryTest, ItRecyclesEntities) {
  // arrange
  auto SUT = Registry();

  // act
  for (auto i = 0; i < 10; ++i) {
    SUT.spawn();
  }

  for (auto i = 0; i < 10; i += 2) {
    SUT.despawn(i);
  }

  // assert
  for (auto i = 0; i < 10; ++i) {
    EXPECT_EQ(i & 1, SUT.isAlive(i));
  }

  for (auto i = 0; i < 10; i += 2) {

    EXPECT_EQ((8 - i) | (1 << 20), SUT.spawn());
  }
}

TEST(RegistryTest, ItDoesNotDespawnDeadEntities) {
  // arrange
  auto SUT = Registry();
  auto Target = SUT.spawn();
  SUT.despawn(Target);

  // Only one increment of the version
  auto ExpectedRecycled = 1 << 20;
  auto ExpectedSpawned = 1;

  // act
  SUT.despawn(Target);
  auto Recycled = SUT.spawn();
  auto Spawned = SUT.spawn();

  // assert
  EXPECT_EQ(ExpectedRecycled, Recycled);
  EXPECT_EQ(ExpectedSpawned, Spawned);
}

TEST(RegistryTest, StaleIsDeadAfterRecycle) {
  // arrange
  auto SUT = Registry();
  auto Original = SUT.spawn();

  // act
  SUT.despawn(Original);
  auto Recycled = SUT.spawn();

  // assert
  EXPECT_TRUE(SUT.isAlive(Recycled));
  EXPECT_TRUE(SUT.isDead(Original));
}

TEST(RegistryTest, RecyclingCapabilityStressTest) {
  // arrange
  auto SUT = Registry();
  auto Expected = 1;

  // act
  for (auto i = 0; i < 4096; ++i) {
    SUT.despawn(SUT.spawn());
  }
  auto Actual = SUT.spawn();

  // assert
  EXPECT_EQ(Expected, Actual);
}

TEST(RegistryTest, IdsOutOfBoundsAreDead) {
  // arrange
  auto SUT = Registry();
  auto Target = 1;

  // act
  // assert
  EXPECT_TRUE(SUT.isDead(Target));
}