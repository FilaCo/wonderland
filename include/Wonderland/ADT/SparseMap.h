/**
 * @file Declares SparseMap<K, V>.
 */
#ifndef WONDERLAND_ADT_SPARSEMAP_H
#define WONDERLAND_ADT_SPARSEMAP_H

#include <concepts>
#include <cstddef>
#include <memory>
#include <vector>

namespace Wonderland::ADT {
/**
 * @brief A SparseMap<K, V> implementation.
 *
 * @tparam K -
 * @tparam V
 */
template <std::convertible_to<std::size_t> KeyT, typename ValueT,
          typename KeyAllocatorT = std::allocator<KeyT>,
          typename ValueAllocatorT = std::allocator<ValueT>>
class SparseMap final {
public:
  explicit constexpr SparseMap(KeyAllocatorT &KeyAllocator,
                               ValueAllocatorT &ValueAllocator) noexcept;
  void clear() noexcept;

private:
  size_t Size;
  std::vector<KeyT, KeyAllocatorT> Sparse;
  std::vector<KeyT, KeyAllocatorT> Dense;
  std::vector<ValueT, ValueAllocatorT> DenseValues;
};
} // namespace Wonderland::ADT

#endif // WONDERLAND_ADT_SPARSEMAP_H
