#include "Wonderland/IPS/Registry.h"
#include "Wonderland/IPS/Id.h"

namespace Wonderland::IPS {
template <typename AllocatorT>
constexpr Registry<AllocatorT>::Registry() noexcept
    : Available(0), NextPosition(0), Ids({}) {}

template <typename AllocatorT> Id Registry<AllocatorT>::spawn() {
  if (Available > 0) {
    return recycle();
  }
  return spawnImpl();
}

template <typename AllocatorT> Id Registry<AllocatorT>::recycle() noexcept {
  // holder stores recycled id version
  auto PositionToRecycle = NextPosition;
  auto Holder = Ids[PositionToRecycle];

  auto Recycled = Id(PositionToRecycle, Holder.Version);

  // restore invariants
  NextPosition = Holder.Position;
  --Available;

  return Recycled;
}
template <typename AllocatorT> Id Registry<AllocatorT>::spawnImpl() noexcept {
  auto Spawned = Id(NextPosition);
  Ids.push_back(Spawned);

  // restore invariants
  ++NextPosition;

  return Spawned;
}

template <typename AllocatorT>
void Registry<AllocatorT>::despawn(Id IdToDespawn) noexcept {
  // holder stores previous NextPosition and actual version of the Id
  auto PositionToDespawn = IdToDespawn.Position;
  auto Holder = Id(NextPosition, IdToDespawn.Version + 1);
  Ids[PositionToDespawn] = Holder;

  // restore invariants
  NextPosition = PositionToDespawn;
  ++Available;
}

template <typename AllocatorT>
bool Registry<AllocatorT>::isAlive(Id Id) const noexcept {
  auto TargetPosition = Id.Position;
  return TargetPosition < Ids.size() &&
         Id.Version == Ids[TargetPosition].Version;
}
template <typename AllocatorT>
bool Registry<AllocatorT>::isDead(Id Id) const noexcept {
  return !isAlive(Id);
}

} // namespace Wonderland::IPS
