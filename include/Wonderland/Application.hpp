#ifndef WONDERLAND_APPLICATION_HPP
#define WONDERLAND_APPLICATION_HPP

#include <expected>

class ApplicationBuilder;

namespace Wonderland {

class Application final {
public:
  static ApplicationBuilder builder();
  int run();

private:
  friend class ApplicationBuilder;
  Application();
};

struct ApplicationBuilderError;

class ApplicationBuilder final {
public:
  ApplicationBuilder();

  ApplicationBuilder *addSystem();

  std::expected<Application, ApplicationBuilderError> build();
};

enum class ApplicationBuilderErrorKind : unsigned char {};

struct ApplicationBuilderError {
  ApplicationBuilderErrorKind Kind;
};

} // namespace Wonderland

#endif // WONDERLAND_APPLICATION_HPP