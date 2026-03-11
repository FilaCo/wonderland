#ifndef STDX_COLLECTIONS_SPARSE_SET_H
#define STDX_COLLECTIONS_SPARSE_SET_H

#define DECLARE_SPARSE_SET(T)                                                  \
  typedef T *sparse_set_##T;

/**
 * @brief Defines an array with fixed capacity of type T.
 */
#define DEFINE_SPARSE_SET(T)                                                   \
  typedef struct {                                                             \
    xalloc *allocator;                                                         \
    vec_uint32_t sparse;                                                       \
    vec_uint32_t packed;                                                       \
    T *values;                                                                 \
  } _sparse_set_##T##_hdr;

#endif // STDX_COLLECTIONS_SPARSE_SET_H
