#ifndef STDX_COLLECTIONS_SPARSE_SET_H
#define STDX_COLLECTIONS_SPARSE_SET_H

#define DEFINE_SPARSE_SET(T)                                                   \
  typedef T *sparse_set_##T;                                                   \
  typedef struct {                                                             \
    xalloc *allocator;                                                         \
    vec_uint32_t sparse;                                                       \
    vec_uint32_t packed;                                                       \
    T *values;                                                                 \
  } __sparse_set_##T##_hdr;

#endif // STDX_COLLECTIONS_SPARSE_SET_H
