#ifndef WONDERLAND_IPS_REGISTRY_H
#define WONDERLAND_IPS_REGISTRY_H

#include "Wonderland/IPS/Id.h"
#include <memory>
#include <vector>

namespace Wonderland::IPS {
template <typename AllocatorT = std::allocator<Id>> class Registry final {
public:
  explicit constexpr Registry() noexcept;
  Id spawn();
  void despawn(Id IdToDespawn) noexcept;
  bool isAlive(Id Id) const noexcept;
  bool isDead(Id Id) const noexcept;

private:
  inline Id recycle() noexcept;
  inline Id spawnImpl() noexcept;

  size_t Available;
  size_t NextPosition;
  std::vector<Id, AllocatorT> Ids;
};
} // namespace Wonderland::IPS

#endif // WONDERLAND_IPS_REGISTRY_H
