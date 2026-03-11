#ifndef WONDERLAND_COLLECTIONS_SPARSESET_H
#define WONDERLAND_COLLECTIONS_SPARSESET_H

#include <cstdint>
#include <vector>

namespace Wonderland::Collections {
template <typename ItemT> class SparseSet {
private:
  std::pmr::vector<uint32_t> Sparse;
  std::pmr::vector<uint32_t> Dense;
  std::pmr::vector<ItemT> DenseItems;
};
} // namespace Wonderland::Collections

#endif // WONDERLAND_COLLECTIONS_SPARSESET_H
