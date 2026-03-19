#ifndef WONDERLAND_ECS_REGISTRY_HPP
#define WONDERLAND_ECS_REGISTRY_HPP

#include "Wonderland/ECS/Entity.hpp"
#include <cassert>
#include <memory>
#include <vector>

namespace Wonderland::ECS {
template <typename AllocatorT = std::allocator<Entity>> class Registry final {
public:
  Registry() noexcept : AvailableCount(0), NextPosition(0) {}

  explicit Registry(AllocatorT &Allocator) noexcept
      : AvailableCount(0), NextPosition(0), Entities{Allocator} {}

  Entity spawn() { return AvailableCount > 0 ? recycle() : spawnImpl(); }

  void despawn(Entity Target) noexcept {
    if (isDead(Target)) {
      return;
    }
    auto PositionToDespawn = getPosition(Target);
    auto NextGeneration = getGeneration(Target) + 1;
    if (NextGeneration > EntityMaxGeneration) {
      // Generation would wrap — retire the slot permanently, do not recycle
      Entities[PositionToDespawn] =
          makeEntity(PositionToDespawn, EntityMaxGeneration);
      return;
    }
    Entities[PositionToDespawn] = makeEntity(NextPosition, NextGeneration);

    NextPosition = PositionToDespawn;
    ++AvailableCount;
  }

  /**
   * @note id with out of range position considered to be **dead**.
   */
  bool isAlive(Entity Target) const noexcept {
    const auto TargetPosition = getPosition(Target);
    return TargetPosition < Entities.size() &&
           getGeneration(Target) == getGeneration(Entities[TargetPosition]);
  }

  bool isDead(Entity Target) const noexcept { return !isAlive(Target); }

private:
  Entity recycle() noexcept {
    auto PositionToRecycle = NextPosition;
    // holder stores recycled Entity generation
    auto Holder = Entities[PositionToRecycle];

    auto Recycled = makeEntity(PositionToRecycle, getGeneration(Holder));

    NextPosition = getPosition(Holder);
    --AvailableCount;

    return Recycled;
  }

  Entity spawnImpl() {
    assert(NextPosition < EntityMaxPosition);
    auto Spawned = NextPosition;

    Entities.push_back(Spawned);

    ++NextPosition;

    return Spawned;
  }

  static constexpr unsigned char EntityPositionBits = 20;
  static constexpr unsigned int EntityMaxPosition = 1u << EntityPositionBits;
  static constexpr unsigned int EntityPositionMask = EntityMaxPosition - 1;

  static constexpr unsigned char EntityGenerationBits = 12;

  static constexpr unsigned int EntityMaxGeneration =
      (1u << EntityGenerationBits) - 1;

  static constexpr Entity makeEntity(unsigned int Position,
                                     unsigned int Generation) {
    return Position | (Generation << EntityPositionBits);
  }

  static constexpr unsigned int getPosition(Entity Target) {
    return Target & EntityPositionMask;
  }

  static constexpr unsigned int getGeneration(Entity Target) {
    return Target >> EntityPositionBits;
  }

  std::size_t AvailableCount;
  std::size_t NextPosition;
  std::vector<Entity, AllocatorT> Entities;
};
} // namespace Wonderland::ECS

#endif // WONDERLAND_ECS_REGISTRY_HPP
