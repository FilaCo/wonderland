#include "Wonderland/Application.hpp"

using namespace Wonderland;

ApplicationBuilder Application::builder() noexcept {
  return ApplicationBuilder();
}

int Application::run() noexcept { return 0; }

std::expected<Application, ApplicationBuilderError>
ApplicationBuilder::build() noexcept {
  return Application();
}