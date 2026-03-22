#ifndef WONDERLAND_ECS_PROPPOOL_HPP
#define WONDERLAND_ECS_PROPPOOL_HPP

#include "Wonderland/ECS/Id.hpp"
#include <memory>
#include <optional>
#include <utility>
#include <vector>

namespace Wonderland::ECS {
template <typename PropT, typename SparseAllocatorT = std::allocator<Id>,
          typename DenseAllocatorT = std::allocator<Id>,
          typename PropsAllocatorT = std::allocator<PropT>>
class PropPool final {
public:
  PropPool(Id Universe = 0) : Universe(Universe) {};

  std::optional<PropT> insert(Id Target, PropT Prop) {
    if (has(Target)) {
      return std::optional(std::exchange(Props[Sparse[Target]], Prop));
    }

    Dense.push_back(Target);
    Props.push_back(Prop);
    if (Target >= Universe) {
      setUniverse(Target + 1);
    }
    Sparse[Target] = Dense.size() - 1;
    return std::nullopt;
  }

  std::optional<PropT> erase(Id Target) noexcept {
    if (!has(Target)) {
      return std::nullopt;
    }

    const auto LastId = Dense.back();
    decltype(auto) SparseIdx = Sparse[Target];
    std::swap(Dense[SparseIdx], Dense.back());
    std::swap(Props[SparseIdx], Props.back());
    std::swap(SparseIdx, Sparse[LastId]);

    Dense.pop_back();
    auto Removed = std::move(Props.back());
    Props.pop_back();

    return Removed;
  }

  bool has(Id Target) const noexcept {
    if (Target >= Universe) {
      return false;
    }
    auto DenseIdx = Sparse[Target];

    return DenseIdx < Dense.size() && Dense[DenseIdx] == Target;
  }

  PropT *at(Id Target) noexcept {
    return has(Target) ? &Props[Sparse[Target]] : nullptr;
  }

  const PropT *at(Id Target) const noexcept {
    return has(Target) ? &Props[Sparse[Target]] : nullptr;
  }

  PropT &operator[](Id Target) { return Props[Sparse[Target]]; }

  const PropT &operator[](Id Target) const { return Props[Sparse[Target]]; }

  void clear() {
    Dense.clear();
    Props.clear();
  }

  void setUniverse(Id NewUniverse) {
    // Sparse.size() == Universe
    Sparse.resize(NewUniverse);
    Universe = NewUniverse;
  }

private:
  Id Universe;
  std::vector<Id, SparseAllocatorT> Sparse;
  std::vector<Id, DenseAllocatorT> Dense;
  std::vector<PropT, PropsAllocatorT> Props;
};

} // namespace Wonderland::ECS

#endif // WONDERLAND_ECS_PROPPOOL_HPP
