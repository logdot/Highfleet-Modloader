#include <highfleet/log.hpp>

void log_from_other_translation_unit() {
    highfleet::logging::log(highfleet::logging::level::debug, "mod", "other TU");
}
