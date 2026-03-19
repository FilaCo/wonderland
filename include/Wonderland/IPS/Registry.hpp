#ifndef WONDERLAND_IPS_REGISTRY_HPP
#define WONDERLAND_IPS_REGISTRY_HPP

#include "Wonderland/IPS/Id.hpp"
#include <cstdint>
#include <memory>
#include <vector>

namespace Wonderland::IPS {
template <typename AllocatorT = std::allocator<Id>> class Registry final {
public:
  Registry() noexcept : Available(0), NextPosition(0) {}
  Id spawn() { return Available > 0 ? recycle() : spawnImpl(); }
  void despawn(Id IdToDespawn) noexcept {
    auto PositionToDespawn = IdToDespawn.Position;
    if (isDead(IdToDespawn)) {
      return;
    }
    uint32_t NextVersion = IdToDespawn.Version + 1;
    if (NextVersion >= Id::RetiredVersion) {
      // Version would wrap — retire the slot permanently, do not recycle
      Ids[PositionToDespawn] = Id(PositionToDespawn, Id::RetiredVersion);
      return;
    }
    // Store previous NextPosition and actual version of the Id
    Ids[PositionToDespawn] = Id(NextPosition, NextVersion);

    NextPosition = PositionToDespawn;
    ++Available;
  }
  bool isAlive(Id Id) const noexcept {
    const auto TargetPosition = Id.Position;
    return TargetPosition < Ids.size() &&
           Id.Version == Ids[TargetPosition].Version;
  }
  bool isDead(Id Id) const noexcept { return !isAlive(Id); }

private:
  Id recycle() noexcept {
    // holder stores recycled id version
    auto PositionToRecycle = NextPosition;
    auto Holder = Ids[PositionToRecycle];

    auto Recycled = Id(PositionToRecycle, Holder.Version);

    NextPosition = Holder.Position;
    --Available;

    return Recycled;
  }
  Id spawnImpl() {
    auto Spawned = Id(NextPosition);
    Ids.push_back(Spawned);

    ++NextPosition;

    return Spawned;
  }

  uint32_t Available;
  uint32_t NextPosition;
  std::vector<Id, AllocatorT> Ids;
};
} // namespace Wonderland::IPS

#endif // WONDERLAND_IPS_REGISTRY_HPP
