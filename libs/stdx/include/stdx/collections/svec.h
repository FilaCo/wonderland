#ifndef STDX_COLLECTIONS_SVEC_H
#define STDX_COLLECTIONS_SVEC_H

#include <stdbool.h>
#include <stddef.h>

/**
 * @brief Declares an array with fixed capacity of type T.
 */
#define DECLARE_SVEC(T, N)                                                     \
  typedef struct {                                                             \
    size_t _size;                                                              \
    T _data[N];                                                                \
  } svec_##T##_##N;                                                            \
  size_t svec_##T##_##N##_size(const svec_##T##_##N *v);                       \
  const T *svec_##T##_##N##_data(const svec_##T##_##N *v);                     \
  bool svec_##T##_##N##_empty(const svec_##T##_##N *v);                        \
  void svec_##T##_##N##_push(svec_##T##_##N *v, const T item);                 \
  void svec_##T##_##N##_pop(svec_##T##_##N *v);

/**
 * @brief Defines an array with fixed capacity of type T.
 */
#define DEFINE_SVEC(T, N)                                                      \
  size_t svec_##T##_##N##_size(const svec_##T##_##N *v) { return v->_size; }   \
  const T *svec_##T##_##N##_data(const svec_##T##_##N *v) { return v->_data; } \
  bool svec_##T##_##N##_empty(const svec_##T##_##N *v) {                       \
    return svec_##T##_##N##_size(v) == 0;                                      \
  }                                                                            \
  void svec_##T##_##N##_push(svec_##T##_##N *v, const T item) {                \
    if (v->_size == (N))                                                       \
      return;                                                                  \
    v->_data[v->_size++] = item;                                               \
  }                                                                            \
  void svec_##T##_##N##_pop(svec_##T##_##N *v) {                               \
    if (v->_size == 0) {                                                       \
      return;                                                                  \
    }                                                                          \
    --v->_size;                                                                \
  }

#endif // STDX_COLLECTIONS_SVEC_H
