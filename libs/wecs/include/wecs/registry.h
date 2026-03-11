#ifndef WECS_REGISTRY_H
#define WECS_REGISTRY_H

#include "id.h"
#include "stdx/alloc/xalloc.h"
#include <stddef.h>
#include <stdbool.h>

typedef struct registry *registry;

registry registry_new(xalloc *allocator);
registry registry_with_capacity(xalloc *allocator, size_t capacity);
void registry_free(registry reg);

id registry_spawn(registry reg);
void registry_despawn(registry reg, id id_);

bool registry_is_alive(registry reg, id id_);
bool registry_is_dead(registry reg, id id_);

#endif // WECS_REGISTRY_H
