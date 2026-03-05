#ifndef STDX_COLLECTIONS_VEC_H
#define STDX_COLLECTIONS_VEC_H

#include "stdx/alloc/xalloc.h"
#include <stddef.h>
#include <stdint.h>

#define DEFINE_VEC(T)                                                          \
  typedef T *vec_##T;                                                          \
  typedef struct {                                                             \
    xalloc *allocator;                                                         \
    size_t cap;                                                                \
    size_t size;                                                               \
    T buf[];                                                                   \
  } __vec_##T##_hdr;                                                           \
  T *vec_##T##_with_capacity(xalloc *allocator, size_t cap) {                  \
    __vec_##T##_hdr *res =                                                     \
        allocator(NULL, NULL, 0, sizeof(__vec_##T##_hdr) + sizeof(T) * cap);   \
    res->allocator = allocator;                                                \
    res->cap = cap;                                                            \
    res->size = 0;                                                             \
    return (T *)((uint8_t *)res + sizeof(__vec_##T##_hdr));                    \
  }                                                                            \
  T *vec_##T##_new(xalloc *allocator) {                                        \
    return vec_##T##_with_capacity(allocator, 0);                              \
  }                                                                            \
  __vec_##T##_hdr *__vec_##T##_get_hdr(const T *v) {                           \
    return (__vec_##T##_hdr *)((uint8_t *)v - sizeof(__vec_##T##_hdr));        \
  }                                                                            \
  size_t __vec_##T##_get_bytes_size(const __vec_##T##_hdr *h) {                \
    return sizeof(__vec_##T##_hdr) + sizeof(T) * h->cap;                       \
  }                                                                            \
  void vec_##T##_free(T *v) {                                                  \
    __vec_##T##_hdr *h = __vec_##T##_get_hdr(v);                               \
    h->allocator(NULL, h, __vec_##T##_get_bytes_size(h), 0);                   \
  }                                                                            \
  size_t vec_##T##_cap(const T *v) { return __vec_##T##_get_hdr(v)->cap; }     \
  size_t vec_##T##_size(const T *v) { return __vec_##T##_get_hdr(v)->size; }   \
  T *vec_##T##_push(T *v, const T item) {                                      \
    __vec_##T##_hdr *h = __vec_##T##_get_hdr(v);                               \
    if (h->size == h->cap) {                                                   \
      size_t osize = __vec_##T##_get_bytes_size(h);                            \
      h->cap = h->cap ? h->cap << 1 : 1;                                       \
      h = h->allocator(NULL, h, osize, __vec_##T##_get_bytes_size(h));         \
    }                                                                          \
    h->buf[h->size++] = item;                                                  \
    return h->buf;                                                             \
  }                                                                            \
  T *vec_##T##_pop(T *v) {                                                     \
    __vec_##T##_hdr *h = __vec_##T##_get_hdr(v);                               \
    if (h->size == 0) {                                                        \
      return NULL;                                                             \
    }                                                                          \
    return &h->buf[--h->size];                                                 \
  }

#endif // STDX_COLLECTIONS_VEC_H
