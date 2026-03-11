#ifndef WONDERLAND_ECS_REGISTRY_H
#define WONDERLAND_ECS_REGISTRY_H

#include "Wonderland/ECS/Entity.h"
#include <cstdint>
#include <vector>

namespace Wonderland::ECS {
class Registry {
public:
  Entity spawn();
  void despawn(Entity Entity);
  bool isAlive(Entity Entity);
  bool isDead(Entity Entity);

private:
  Entity recycle();
  Entity spawnImpl();

  uint32_t Available;
  uint32_t NextPosition;
  std::pmr::vector<Entity> Entities;
};
} // namespace Wonderland::ECS

#endif // WONDERLAND_ECS_REGISTRY_H
