#ifndef WONDERLAND_ECS_SPARSESET_HPP
#define WONDERLAND_ECS_SPARSESET_HPP

#include "Wonderland/ECS/Entity.hpp"
#include <memory>
#include <vector>

namespace Wonderland::ECS {
template <typename SparseAllocatorT = std::allocator<Entity>,
          typename DenseAllocatorT = std::allocator<Entity>>
class SparseSet final {
public:
  SparseSet() : Universe(0) {};

  bool insert(Entity Target) {
    if (has(Target)) {
      return false;
    }
    Dense.push_back(Target);
    if (Target >= Universe) {
      setUniverse(Target + 1);
    }
    Sparse[Target] = Dense.size() - 1;
    return true;
  }

  bool erase(Entity Target) noexcept {
    if (!has(Target)) {
      return false;
    }

    decltype(auto) SparseIdx = Sparse[Target];
    decltype(auto) LastEntity = Dense.back();
    Sparse[LastEntity] = SparseIdx;
    std::swap(Dense[SparseIdx], LastEntity);
    Dense.pop_back();
    return true;
  }

  bool has(Entity Target) const noexcept {
    if (Target >= Universe) {
      return false;
    }
    auto DenseIdx = Sparse[Target];

    return DenseIdx < Dense.size() && Dense[DenseIdx] == Target;
  }

  void clear() { Dense.clear(); }

  void setUniverse(Entity NewUniverse) {
    // Sparse.size() == Universe
    Sparse.resize(NewUniverse);
    Universe = NewUniverse;
  }

private:
  Entity Universe;
  std::vector<Entity, SparseAllocatorT> Sparse;
  std::vector<Entity, DenseAllocatorT> Dense;
};

} // namespace Wonderland::ECS

#endif // WONDERLAND_ECS_SPARSESET_HPP
