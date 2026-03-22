#ifndef WONDERLAND_APPLICATION_HPP
#define WONDERLAND_APPLICATION_HPP

class ApplicationBuilder;

namespace Wonderland {

class Application final {
public:
  static ApplicationBuilder builder();
  int run();
};

class ApplicationBuilder final {};
} // namespace Wonderland

#endif // WONDERLAND_APPLICATION_HPP