#include "Wonderland/ECS/Registry.h"
#include "Wonderland/ECS/Entity.h"

namespace Wonderland::ECS {
Entity Registry::spawn() {
  if (Available > 0) {
    return recycle();
  }
  return spawnImpl();
}

static const uint8_t EntityPositionBits = 20;
static const uint32_t EntityPositionMask = (1u << EntityPositionBits) - 1;

static inline Entity entityNew(uint32_t Position, uint32_t Version) {
  return Position | (Version << EntityPositionBits);
}

static inline uint32_t entityGetPosition(Entity Entity) {
  return Entity & EntityPositionMask;
}

static inline uint32_t entityGetVersion(Entity Entity) {
  return Entity >> EntityPositionBits;
}

Entity Registry::recycle() {
  // holder stores recycled id version
  auto PositionToRecycle = NextPosition;
  auto Holder = Entities[PositionToRecycle];

  auto Recycled = entityNew(PositionToRecycle, entityGetVersion(Holder));

  // restore invariants
  NextPosition = entityGetPosition(Holder);
  --Available;

  return Recycled;
}
Entity Registry::spawnImpl() {
  auto Spawned = entityNew(NextPosition, 0);
  Entities.push_back(Spawned);

  // restore invariants
  ++NextPosition;

  return Spawned;
}

void Registry::despawn(Entity Entity) {
  // holder stores previous NextPosition and actual version of the entity
  auto PositionToDespawn = entityGetPosition(Entity);
  auto Holder = entityNew(NextPosition, entityGetVersion(Entity) + 1);
  Entities[PositionToDespawn] = Holder;

  // restore invariants
  NextPosition = PositionToDespawn;
  ++Available;
}

bool Registry::isAlive(Entity Entity) {
  auto TargetPosition = entityGetPosition(Entity);
  return TargetPosition < Entities.size() &&
         entityGetVersion(Entity) == entityGetVersion(Entities[TargetPosition]);
}
bool Registry::isDead(Entity Entity) { return !isAlive(Entity); }

} // namespace Wonderland::ECS
