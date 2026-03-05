#include "wecs/registry.h"
#include "stdx/alloc/xalloc.h"
#include "stdx/collections/vec.h"
#include <stdint.h>

DEFINE_VEC(id)

struct __registry {
  xalloc *allocator;
  uint32_t available;
  uint32_t next_position;
  vec_id ids;
};

// registry registry_new(xalloc *allocator) {
//   return registry_with_capacity(allocator, 0);
// }
// registry registry_with_capacity(xalloc *allocator, size_t cap) {
//   registry res = allocator(NULL, NULL, 0, sizeof(struct __registry));
//   res->allocator = allocator;
//   res->available = 0;
//   res->next_position = 0;
//   res->ids = vec_id_with_capacity(allocator, cap);

//   return res;
// }
// void registry_free(registry reg) {
//   vec_id_free(reg->ids);
//   reg->allocator(NULL, reg, sizeof(struct __registry), 0);
// }

// static const uint8_t ID_POSITION_BITS = 20;
// static const uint32_t ID_POSITION_MASK = (1 << ID_POSITION_BITS) - 1;

// static inline uint32_t __id_new(uint32_t position, uint32_t version) {
//   return position | (version << ID_POSITION_BITS);
// }

// static inline uint32_t __id_position(id id_) { return id_ & ID_POSITION_MASK;
// }

// static inline uint16_t __id_version(id id_) { return id_ >> ID_POSITION_BITS;
// }

// static inline id __registry_recycle(registry reg) {
//   // holder stores recycled id version
//   uint32_t position_to_recycle = reg->next_position;
//   id holder = reg->ids[reg->next_position];

//   id recycled = __id_new(reg->next_position, __id_version(holder));

//   // restore invariants
//   reg->next_position = __id_position(id id_) return recycled;
// }

// static inline id __registry_spawn(registry reg) {}

// id registry_spawn(registry reg) {
//   if (reg->available > 0) {
//     return __registry_recycle(reg);
//   }

//   return __registry_spawn(reg);
// }
// void registry_despawn(registry reg, id id_) {}