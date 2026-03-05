#ifndef WECS_REGISTRY_H
#define WECS_REGISTRY_H

#include "id.h"
#include "stdx/alloc/xalloc.h"
#include <stddef.h>

typedef struct __registry *registry;

registry registry_new(xalloc *allocator);
registry registry_with_capacity(xalloc *allocator, size_t cap);
void registry_free(registry reg);

id registry_spawn(registry reg);
void registry_despawn(registry reg, id id_);

#endif // WECS_REGISTRY_H
