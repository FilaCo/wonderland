#include "Wonderland/IPS/Id.h"
#include <cstddef>

namespace Wonderland::IPS {

constexpr Id::Id(size_t RawId) noexcept
    : Position(RawId & PositionMask), Version(RawId >> PositionBits) {}

constexpr Id::operator size_t() const {
  return Position | (Version << PositionBits);
}

bool Id::operator==(Id Other) const {
  return Position == Other.Position && Version == Other.Version;
}
bool Id::operator!=(Id Other) const { return !(*this == Other); }

constexpr Id::Id(uint32_t Position, uint32_t Version) noexcept
    : Position(Position), Version(Version) {}

} // namespace Wonderland::IPS