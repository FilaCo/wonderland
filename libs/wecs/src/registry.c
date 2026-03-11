#include "wecs/registry.h"
#include "stdx/alloc/xalloc.h"
#include "stdx/collections/vec.h"
#include <assert.h>
#include <stdint.h>

DECLARE_VEC(id)
DEFINE_VEC(id)

struct registry {
  xalloc *allocator;
  uint32_t available;
  uint32_t next_position;
  vec_id ids;
};

registry registry_new(xalloc *allocator) {
  return registry_with_capacity(allocator, 0);
}
registry registry_with_capacity(xalloc *allocator, size_t capacity) {
  registry res = allocator(NULL, NULL, 0, sizeof(struct registry));
  res->allocator = allocator;
  res->available = 0;
  res->next_position = 0;
  res->ids = vec_id_with_capacity(allocator, capacity);

  return res;
}
void registry_free(registry reg) {
  vec_id_free(reg->ids);
  reg->allocator(NULL, reg, sizeof(struct registry), 0);
}

enum { ID_POSITION_BITS = 20, ID_POSITION_MASK = (1u << ID_POSITION_BITS) - 1 };

static inline uint32_t id_new(uint32_t position, uint32_t version) {
  return position | (version << ID_POSITION_BITS);
}

static inline uint32_t id_get_position(id id_) {
  return id_ & ID_POSITION_MASK;
}

static inline uint16_t id_get_version(id id_) {
  return id_ >> ID_POSITION_BITS;
}

static inline id registry__recycle(registry reg) {
  // holder stores recycled id version
  uint32_t position_to_recycle = reg->next_position;
  id holder = reg->ids[position_to_recycle];

  id recycled = id_new(position_to_recycle, id_get_version(holder));

  // restore invariants
  reg->next_position = id_get_position(holder);
  --reg->available;

  return recycled;
}

static inline id registry__spawn(registry reg) {
  id spawned = id_new(reg->next_position, 0);
  reg->ids = vec_id_push(reg->ids, spawned);

  // restore invariants
  ++reg->next_position;

  return spawned;
}

id registry_spawn(registry reg) {
  if (reg->available > 0) {
    return registry__recycle(reg);
  }

  return registry__spawn(reg);
}
void registry_despawn(registry reg, id id_) {
  // holder stores previous next_position and actual version of the entity
  uint32_t position_to_despawn = id_get_position(id_);
  id holder = id_new(reg->next_position, id_get_version(id_) + 1);
  reg->ids[position_to_despawn] = holder;

  // restore invariants
  reg->next_position = position_to_despawn;
  ++reg->available;
}

bool registry_is_alive(registry reg, id id_) {
  assert(id_get_position(id_) < vec_id_size(reg->ids));
  return id_get_version(id_) == id_get_version(reg->ids[id_get_position(id_)]);
}

bool registry_is_dead(registry reg, id id_) {
  return !registry_is_alive(reg, id_);
}
