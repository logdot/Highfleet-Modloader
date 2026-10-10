#include <highfleet/log.hpp>
#include "layout_test.hpp"

#include <cstdio>
#include <cstdlib>
#include <string>

HIGHFLEET_SETUP_LOGGER()

void log_from_other_translation_unit();

namespace {

struct Capture {
    unsigned writes = 0;
    unsigned flushes = 0;
    bool allow = true;
    unsigned filter_calls = 0;
    std::uint32_t filter_level = 0;
    std::string filter_target;
    std::uint32_t severity = 0;
    std::string target;
    std::string message;
};

void require(bool condition, const char* description) {
    if (!condition) {
        std::fprintf(stderr, "FAIL: %s\n", description);
        std::exit(EXIT_FAILURE);
    }
}

void HIGHFLEET_LOGGING_CDECL discard(void*, const highfleet::ffi::HfmLogRecordV1*) {}

void HIGHFLEET_LOGGING_CDECL flush_host(void* context) {
    ++static_cast<Capture*>(context)->flushes;
}

std::uint8_t HIGHFLEET_LOGGING_CDECL filter(
    void* context, std::uint32_t severity, highfleet::ffi::HfmStr target) {
    Capture& capture = *static_cast<Capture*>(context);
    ++capture.filter_calls;
    capture.filter_level = severity;
    capture.filter_target = target.len == 0 ? "" :
        std::string(reinterpret_cast<const char*>(target.ptr), target.len);
    return capture.allow ? 7 : 0;
}

void HIGHFLEET_LOGGING_CDECL write(void* context, const highfleet::ffi::HfmLogRecordV1* record) {
    Capture& capture = *static_cast<Capture*>(context);
    ++capture.writes;
    capture.severity = record->level;
    capture.target = record->target.len == 0 ? "" :
        std::string(reinterpret_cast<const char*>(record->target.ptr), record->target.len);
    capture.message = record->message.len == 0 ? "" :
        std::string(reinterpret_cast<const char*>(record->message.ptr), record->message.len);
    require(record->struct_size == sizeof(*record), "record advertises its size");
    require(record->reserved == 0, "reserved field is zero");
    require(record->module_path.len == 0, "absent module path is empty");
    require(record->file.len == 0, "absent file is empty");
    require(record->line == 0, "absent source line is zero");
}

void copies_host_table(Capture& capture) {
    using namespace highfleet::ffi;
    HfmHostApiV1 host = {1, sizeof(HfmHostApiV1), &capture, &filter, &write, &flush_host};
    require(highfleet_mod_setup_v1(&host) == HFM_STATUS_OK, "valid table installs");
    host.log_write = &discard;
    host.context = nullptr;
}

void forwards_records_across_translation_units(Capture& capture) {
    log_from_other_translation_unit();
    require(capture.writes == 1, "copied host table forwards from another translation unit");
    require(capture.severity == 4, "debug uses ABI level 4");
    require(capture.target == "mod", "target is forwarded");
    require(capture.message == "other TU", "message is forwarded");
}

void checks_host_filter(Capture& capture) {
    using namespace highfleet::logging;
    require(enabled(level::trace, "custom"), "nonzero host result enables logging");
    require(capture.filter_level == 5, "enabled passes the ABI level");
    require(capture.filter_target == "custom", "enabled passes the target");
    const unsigned previous_writes = capture.writes;
    capture.allow = false;
    require(!enabled(level::info, "mod"), "zero host result disables logging");
    info("mod", "suppressed");
    require(capture.writes == previous_writes, "disabled records are not written");
    capture.allow = true;
}

void forwards_message_only_helpers(Capture& capture) {
    using log_function = void (*)(const char*);
    const log_function helpers[] = {
        highfleet::logging::error, highfleet::logging::warn, highfleet::logging::info,
        highfleet::logging::debug, highfleet::logging::trace
    };
    for (std::uint32_t i = 0; i < 5; ++i) {
        const unsigned before = capture.writes;
        helpers[i](u8"hello \u2603");
        require(capture.writes == before + 1, "message-only helper writes one record");
        require(capture.severity == i + 1, "helper forwards its numeric level");
        require(capture.target.empty(), "message-only helper uses an empty target");
        require(capture.message == u8"hello \u2603", "message-only helper preserves UTF-8");
    }
}

void null_strings_are_empty(Capture& capture) {
    highfleet::logging::info(nullptr, nullptr);
    require(capture.target.empty(), "null target is empty");
    require(capture.message.empty(), "null message is empty");
}

void cannot_replace_installed_host(Capture& capture) {
    using namespace highfleet::ffi;
    HfmHostApiV1 replacement = {1, sizeof(HfmHostApiV1), nullptr, nullptr, &discard, nullptr};
    require(highfleet_mod_setup_v1(&replacement) == HFM_STATUS_LOGGER_ALREADY_SET,
            "setup can succeed only once");
    const unsigned before = capture.writes;
    highfleet::logging::info("still installed");
    require(capture.writes == before + 1, "failed reinstall preserves installed host");
}

void optional_callbacks_may_be_absent(Capture& capture) {
    using namespace highfleet::ffi;
    HfmHostApiV1 host = {1, sizeof(HfmHostApiV1), &capture, nullptr, &write, nullptr};
    require(highfleet_mod_setup_v1(&host) == HFM_STATUS_OK, "optional callbacks may be null");
    require(highfleet::logging::enabled(highfleet::logging::level::trace),
            "missing filter enables every level");
    highfleet::logging::trace("without optional callbacks");
    require(capture.writes == 1, "missing filter does not suppress records");
    highfleet::logging::flush();
    require(capture.flushes == 0, "missing flush is a no-op");
}

void rejects_invalid_host_tables() {
    using namespace highfleet::ffi;
    require(highfleet_mod_setup_v1(nullptr) == HFM_STATUS_NULL_API, "rejects null table");
    HfmHostApiV1 host = {99, sizeof(HfmHostApiV1), nullptr, nullptr, &discard, nullptr};
    require(highfleet_mod_setup_v1(&host) == HFM_STATUS_UNSUPPORTED_ABI,
            "rejects unsupported version");
    host.abi_version = HFM_HOST_ABI_VERSION_1;
    host.struct_size = 8;
    require(highfleet_mod_setup_v1(&host) == HFM_STATUS_API_TOO_SMALL, "rejects short table");
    host.struct_size = sizeof(HfmHostApiV1);
    host.log_write = nullptr;
    require(highfleet_mod_setup_v1(&host) == HFM_STATUS_MISSING_CALLBACK,
            "rejects missing required callback");
}

} // namespace

int main(int argc, char** argv) {
    rejects_invalid_host_tables();
    Capture capture;
    require(!highfleet::logging::enabled(highfleet::logging::level::info),
            "logging is disabled before setup");
    highfleet::logging::flush();
    highfleet::logging::info("mod", "before setup");
    require(capture.writes == 0, "logging before setup is a no-op");
    if (argc == 2 && std::string(argv[1]) == "optional") {
        optional_callbacks_may_be_absent(capture);
        std::puts("SDK optional callback tests passed");
        return EXIT_SUCCESS;
    }
    require(argc == 1, "unknown test argument");
    copies_host_table(capture);
    forwards_records_across_translation_units(capture);
    checks_host_filter(capture);
    highfleet::logging::flush();
    require(capture.flushes == 1, "flush forwards host context");
    forwards_message_only_helpers(capture);
    null_strings_are_empty(capture);
    cannot_replace_installed_host(capture);
    std::puts("SDK logging tests passed");
}
