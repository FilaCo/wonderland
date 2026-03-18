#include "Wonderland/ADT/SparseMap.h"
#include <concepts>

namespace Wonderland::ADT {
template <std::convertible_to<std::size_t> KeyT, typename ValueT,
          typename KeyAllocatorT, typename ValueAllocatorT>
void SparseMap<KeyT, ValueT, KeyAllocatorT, ValueAllocatorT>::clear() noexcept {
  Size = 0;
}
} // namespace Wonderland::ADT