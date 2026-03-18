/**
 * @file Declares `Id` type.
 */
#ifndef WONDERLAND_IPS_ID_H
#define WONDERLAND_IPS_ID_H

#include <cstddef>
#include <cstdint>

namespace Wonderland::IPS {
/**
 * @brief Opaque type for identifiers in IPS framework.
 */
class Id final {
public:
  /**
   * @brief Constructs `Id` from raw `size_t` value.
   *
   * @param RawId - value to construct from.
   */
  explicit constexpr Id(size_t RawId) noexcept;

  ~Id() = default;

  /**
   * @brief Converts `Id` to a `size_t` value.
   *
   * Can be useful to use `Id` instance as an index.
   *
   * @return `size_t` representation of `Id` type
   */
  constexpr operator size_t() const;

  /**
   * @brief Checks if two `Id` instances are equal.
   *
   * @param Other - value to check with.
   *
   * @return `true` if instances are equal.
   *
   * @note It accepts `Other` by value because it's expected to be cheaper than
   * pass `const Id&`.
   */
  bool operator==(Id Other) const;

  /**
   * @brief Checks if two `Id` instances are **not** equal.
   *
   * @param Other - value to check with.
   *
   * @return `true` if instances are **not** equal.
   *
   * @note It accepts `Other` by value because it's expected to be cheaper than
   * pass `const Id&`.
   */
  bool operator!=(Id Other) const;

private:
  /**
   * @brief Constructs `Id` from `Position` and `Version`.
   *
   * @param Position - `Id` position in registry.
   * @param Version - `Id` version
   *
   * @note it's callee responsibility to ensure that `Version` fits it's
   * boundaries. Too big values will be cut off.
   */
  explicit constexpr Id(uint32_t Position, uint32_t Version) noexcept;

  template <typename AllocatorT> friend class Registry;

  /**
   * @brief Bits reserved for `Id::Position` field.
   */
  static const uint8_t PositionBits = 20;
  /**
   * @brief Bits reserved for `Id::Version` field.
   */
  static const uint8_t VersionBits = 12;
  /**
   * @brief Mask which is used to extract a position value from raw id value.
   */
  static const uint32_t PositionMask = (1 << 20) - 1;

  /**
   * @brief Position in registry.
   */
  uint32_t Position : PositionBits;
  /**
   * @brief Id version.
   *
   * It's used to discriminate despawned ids.
   */
  uint32_t Version : VersionBits;
};
} // namespace Wonderland::IPS

#endif // WONDERLAND_IPS_ID_H
