/**
 * @file Declares `Id` type.
 */
#ifndef WONDERLAND_IPS_ID_HPP
#define WONDERLAND_IPS_ID_HPP

#include <cstddef>
#include <cstdint>

namespace Wonderland::IPS {
/**
 * @brief Opaque type for identifiers in IPS framework.
 */
class Id final {
public:
  explicit constexpr Id(size_t RawId) noexcept
      : Position(RawId & PositionMask), Version(RawId >> PositionBits) {}

  /**
   * @brief Converts `Id` to a `size_t` value.
   *
   * Can be useful to use `Id` instance as an index.
   *
   * @return `size_t` representation of `Id` type
   */
  constexpr operator size_t() const {
    return Position | (Version << PositionBits);
  }

  /**
   * @note It accepts `Other` by value because it's expected to be cheaper than
   * pass `const Id&`.
   */
  constexpr bool operator==(Id Other) const {
    return Position == Other.Position && Version == Other.Version;
  }

  constexpr bool operator!=(Id Other) const { return !(*this == Other); }

private:
  /**
   * @note it's caller responsibility to ensure that `Version` fits it's
   * boundaries. Too big values will be cut off.
   */
  constexpr Id(uint32_t Position, uint32_t Version) noexcept
      : Position(Position), Version(Version) {}

  template <typename AllocatorT> friend class Registry;

  static constexpr uint8_t PositionBits = 20;
  static constexpr uint8_t VersionBits = 12;
  /**
   * @note As any `Id` instance consists of 2 parts: Version in `VersionBits`
   * older bits and `Position` in `PositionBits` remainder, then we need this
   * mask to extract the `Position` value.
   */
  static constexpr uint32_t PositionMask = (1u << PositionBits) - 1;
  /**
   * Reserved version after which we cannot recycle an `Id`.
   */
  static constexpr uint32_t RetiredVersion = (1u << VersionBits) - 1;

  uint32_t Position : PositionBits;
  uint32_t Version : VersionBits;
};
} // namespace Wonderland::IPS

#endif // WONDERLAND_IPS_ID_HPP
