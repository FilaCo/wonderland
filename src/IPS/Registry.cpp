#include "Wonderland/IPS/Registry.h"
#include "Wonderland/IPS/Id.h"

namespace Wonderland::IPS {
Id Registry::spawn() {
  if (Available > 0) {
    return recycle();
  }
  return spawnImpl();
}

static const uint8_t IdPositionBits = 20;
static const uint32_t IdPositionMask = (1u << IdPositionBits) - 1;

static inline Id idNew(uint32_t Position, uint32_t Version) {
  return Position | (Version << IdPositionBits);
}

static inline uint32_t idGetPosition(Id Id) { return Id & IdPositionMask; }

static inline uint32_t idGetVersion(Id Id) { return Id >> IdPositionBits; }

Id Registry::recycle() {
  // holder stores recycled id version
  auto PositionToRecycle = NextPosition;
  auto Holder = Ids[PositionToRecycle];

  auto Recycled = idNew(PositionToRecycle, idGetVersion(Holder));

  // restore invariants
  NextPosition = idGetPosition(Holder);
  --Available;

  return Recycled;
}
Id Registry::spawnImpl() {
  auto Spawned = idNew(NextPosition, 0);
  Ids.push_back(Spawned);

  // restore invariants
  ++NextPosition;

  return Spawned;
}

void Registry::despawn(Id Id) {
  // holder stores previous NextPosition and actual version of the Id
  auto PositionToDespawn = idGetPosition(Id);
  auto Holder = idNew(NextPosition, idGetVersion(Id) + 1);
  Ids[PositionToDespawn] = Holder;

  // restore invariants
  NextPosition = PositionToDespawn;
  ++Available;
}

bool Registry::isAlive(Id Id) {
  auto TargetPosition = idGetPosition(Id);
  return TargetPosition < Ids.size() &&
         idGetVersion(Id) == idGetVersion(Ids[TargetPosition]);
}
bool Registry::isDead(Id Id) { return !isAlive(Id); }

} // namespace Wonderland::IPS
