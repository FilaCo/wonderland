/**
 * @file vec.h
 * @brief Dynamic array (vec) collection.
 */
#ifndef STDX_COLLECTIONS_VEC_H
#define STDX_COLLECTIONS_VEC_H

#include "stdx/alloc/xalloc.h"
#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

/**
 * @brief Declares a dynamic array (a.k.a vec) of type T.
 */
#define DECLARE_VEC(T)                                                         \
  typedef T *vec_##T;                                                          \
  T *vec_##T##_with_capacity(xalloc *allocator, size_t capacity);              \
  T *vec_##T##_new(xalloc *allocator);                                         \
  void vec_##T##_free(T *v);                                                   \
  T *vec_##T##_reserve(T *v, size_t capacity);                                 \
  size_t vec_##T##_capacity(const T *v);                                       \
  size_t vec_##T##_size(const T *v);                                           \
  bool vec_##T##_empty(const T *v);                                            \
  T *vec_##T##_back(const T *v);                                               \
  T *vec_##T##_push(T *v, const T item);                                       \
  void vec_##T##_pop(T *v);

/**
 * @brief Defines a dynamic array (a.k.a vec) of type T.
 */
#define DEFINE_VEC(T)                                                          \
  typedef struct {                                                             \
    xalloc *allocator;                                                         \
    size_t capacity;                                                           \
    size_t size;                                                               \
    T data[];                                                                   \
  } vec_##T##_hdr_;                                                            \
  T *vec_##T##_with_capacity(xalloc *allocator, size_t capacity) {             \
    vec_##T##_hdr_ *res = allocator(                                           \
        NULL, NULL, 0, sizeof(vec_##T##_hdr_) + sizeof(T) * capacity);         \
    res->allocator = allocator;                                                \
    res->capacity = capacity;                                                  \
    res->size = 0;                                                             \
    return (T *)((uint8_t *)res + offsetof(vec_##T##_hdr_, data));              \
  }                                                                            \
  T *vec_##T##_new(xalloc *allocator) {                                        \
    return vec_##T##_with_capacity(allocator, 0);                              \
  }                                                                            \
  static inline vec_##T##_hdr_ *vec_##T##_get_hdr_(const T *v) {               \
    return (vec_##T##_hdr_ *)((uint8_t *)v - offsetof(vec_##T##_hdr_, data));   \
  }                                                                            \
  static inline vec_##T##_hdr_ *vec_##T##_grow_(vec_##T##_hdr_ *h,             \
                                                size_t capacity) {             \
    size_t hdr_size = sizeof(vec_##T##_hdr_);                                  \
    size_t elem_size = sizeof(T);                                              \
    size_t osize = hdr_size + elem_size * h->capacity;                         \
    size_t nsize = hdr_size + elem_size * capacity;                            \
    h = h->allocator(NULL, h, osize, nsize);                                   \
    h->capacity = capacity;                                                    \
    return h;                                                                  \
  }                                                                            \
  T *vec_##T##_reserve(T *v, size_t capacity) {                                \
    vec_##T##_hdr_ *h = vec_##T##_get_hdr_(v);                                 \
    size_t old_capacity = h->capacity;                                         \
    if (old_capacity >= capacity) {                                            \
      return v;                                                                \
    }                                                                          \
    h = vec_##T##_grow_(h, capacity);                                          \
    return (T *)((uint8_t *)h + offsetof(vec_##T##_hdr_, data));                \
  }                                                                            \
  void vec_##T##_free(T *v) {                                                  \
    vec_##T##_hdr_ *h = vec_##T##_get_hdr_(v);                                 \
    size_t hdr_size = sizeof(vec_##T##_hdr_);                                  \
    size_t bytes_size = hdr_size + sizeof(T) * h->capacity;                    \
    h->allocator(NULL, h, bytes_size, 0);                                      \
  }                                                                            \
  size_t vec_##T##_capacity(const T *v) {                                      \
    return vec_##T##_get_hdr_(v)->capacity;                                    \
  }                                                                            \
  size_t vec_##T##_size(const T *v) { return vec_##T##_get_hdr_(v)->size; }    \
  bool vec_##T##_empty(const T *v) { return vec_##T##_size(v) == 0; }          \
  T *vec_##T##_back(const T *v) {                                              \
    vec_##T##_hdr_ *h = vec_##T##_get_hdr_(v);                                 \
    return &h->data[h->size - 1];                                               \
  }                                                                            \
  T *vec_##T##_push(T *v, const T item) {                                      \
    vec_##T##_hdr_ *h = vec_##T##_get_hdr_(v);                                 \
    if (h->size == h->capacity) {                                              \
      h = vec_##T##_grow_(h, h->capacity ? h->capacity << 1 : 1);              \
    }                                                                          \
    h->data[h->size++] = item;                                                  \
    return h->data;                                                             \
  }                                                                            \
  void vec_##T##_pop(T *v) {                                                   \
    vec_##T##_hdr_ *h = vec_##T##_get_hdr_(v);                                 \
    if (h->size == 0) {                                                        \
      return;                                                                  \
    }                                                                          \
    --h->size;                                                                 \
  }

#endif // STDX_COLLECTIONS_VEC_H
