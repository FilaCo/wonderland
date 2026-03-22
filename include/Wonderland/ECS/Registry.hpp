#ifndef WONDERLAND_ECS_REGISTRY_HPP
#define WONDERLAND_ECS_REGISTRY_HPP

#include "Wonderland/ECS/Id.hpp"
#include <cassert>
#include <memory>
#include <vector>

namespace Wonderland::ECS {
template <typename AllocatorT = std::allocator<Id>> class Registry final {
public:
  Registry() noexcept : AvailableCount(0), NextPosition(0) {}

  explicit Registry(AllocatorT &Allocator) noexcept
      : AvailableCount(0), NextPosition(0), Ids{Allocator} {}

  Id spawn() { return AvailableCount > 0 ? recycle() : spawnImpl(); }

  void despawn(Id Target) noexcept {
    if (isDead(Target)) {
      return;
    }
    auto PositionToDespawn = getPosition(Target);
    auto NextVersion = getVersion(Target) + 1;
    if (NextVersion > IdMaxVersion) {
      // Version would wrap — retire the slot permanently, do not recycle
      Ids[PositionToDespawn] = makeId(PositionToDespawn, IdMaxVersion);
      return;
    }
    Ids[PositionToDespawn] = makeId(NextPosition, NextVersion);

    NextPosition = PositionToDespawn;
    ++AvailableCount;
  }

  /**
   * @note id with out of range position considered to be **dead**.
   */
  bool isAlive(Id Target) const noexcept {
    const auto TargetPosition = getPosition(Target);
    return TargetPosition < Ids.size() &&
           getVersion(Target) == getVersion(Ids[TargetPosition]);
  }

  bool isDead(Id Target) const noexcept { return !isAlive(Target); }

private:
  Id recycle() noexcept {
    auto PositionToRecycle = NextPosition;
    // holder stores recycled Id's version
    auto Holder = Ids[PositionToRecycle];

    auto Recycled = makeId(PositionToRecycle, getVersion(Holder));

    NextPosition = getPosition(Holder);
    --AvailableCount;

    return Recycled;
  }

  Id spawnImpl() {
    assert(NextPosition < IdMaxPosition);
    auto Spawned = NextPosition;

    Ids.push_back(Spawned);

    ++NextPosition;

    return Spawned;
  }

  static constexpr unsigned char IdPositionBits = 20;
  static constexpr unsigned int IdMaxPosition = 1u << IdPositionBits;
  static constexpr unsigned int IdPositionMask = IdMaxPosition - 1;

  static constexpr unsigned char IdVersionBits = 12;

  static constexpr unsigned int IdMaxVersion = (1u << IdVersionBits) - 1;

  static constexpr Id makeId(unsigned int Position, unsigned int Version) {
    return Position | (Version << IdPositionBits);
  }

  static constexpr unsigned int getPosition(Id Target) {
    return Target & IdPositionMask;
  }

  static constexpr unsigned int getVersion(Id Target) {
    return Target >> IdPositionBits;
  }

  std::size_t AvailableCount;
  std::size_t NextPosition;
  std::vector<Id, AllocatorT> Ids;
};
} // namespace Wonderland::ECS

#endif // WONDERLAND_ECS_REGISTRY_HPP
