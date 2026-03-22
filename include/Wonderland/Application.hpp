#ifndef WONDERLAND_APPLICATION_HPP
#define WONDERLAND_APPLICATION_HPP

#include <expected>

namespace Wonderland {

class ApplicationBuilder;

class Application final {
public:
  static ApplicationBuilder builder() noexcept;

  ~Application() = default;
  Application(const Application &) = delete;
  Application &operator=(const Application &) = delete;
  Application(Application &&) = default;
  Application &operator=(Application &&) = default;

  int run() noexcept;

private:
  friend class ApplicationBuilder;
  Application() = default;
};

enum class ApplicationBuilderError : unsigned char {};

class ApplicationBuilder final {
public:
  ApplicationBuilder() = default;

  constexpr int run() noexcept {
    auto App =
        build().transform(&Application::run).transform_error([](auto &&Error) {
          return static_cast<int>(Error);
        });

    return App.has_value() ? App.value() : App.error();
  }

  std::expected<Application, ApplicationBuilderError> build() noexcept;
};

} // namespace Wonderland

#endif // WONDERLAND_APPLICATION_HPP