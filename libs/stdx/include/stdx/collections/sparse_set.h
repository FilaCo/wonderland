#ifndef STDX_COLLECTIONS_SPARSE_SET_H
#define STDX_COLLECTIONS_SPARSE_SET_H

/**
 * @brief Macro that defines a sparse set (a.k.a sparse_set_T) of type T.
 */
#define DEFINE_SPARSE_SET(T)                                                   \
  typedef T *sparse_set_##T;                                                   \
  typedef struct {                                                             \
    xalloc *allocator;                                                         \
    vec_uint32_t sparse;                                                       \
    vec_uint32_t packed;                                                       \
    T *values;                                                                 \
  } __sparse_set_##T##_hdr;

#endif // STDX_COLLECTIONS_SPARSE_SET_H
