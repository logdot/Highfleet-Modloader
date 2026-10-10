#ifndef HIGHFLEET_LOG_HPP
#define HIGHFLEET_LOG_HPP

#include <atomic>
#include <cstddef>
#include <cstdint>
#include <cstring>

#if defined(_WIN32)
#define HIGHFLEET_LOGGING_CDECL __cdecl
#define HIGHFLEET_LOGGING_EXPORT __declspec(dllexport)
#else
#define HIGHFLEET_LOGGING_CDECL
#define HIGHFLEET_LOGGING_EXPORT
#endif

namespace highfleet {
namespace ffi {

constexpr std::uint32_t HFM_HOST_ABI_VERSION_1 = 1;
constexpr std::uint32_t HFM_STATUS_OK = 0;
constexpr std::uint32_t HFM_STATUS_NULL_API = 1;
constexpr std::uint32_t HFM_STATUS_UNSUPPORTED_ABI = 2;
constexpr std::uint32_t HFM_STATUS_API_TOO_SMALL = 3;
constexpr std::uint32_t HFM_STATUS_MISSING_CALLBACK = 4;
constexpr std::uint32_t HFM_STATUS_LOGGER_ALREADY_SET = 5;
constexpr std::uint32_t HFM_STATUS_PANIC = UINT32_MAX;

struct HfmStr {
    const std::uint8_t* ptr;
    std::size_t len;
};

struct HfmLogRecordV1 {
    std::uint32_t struct_size;
    std::uint32_t level;
    HfmStr target;
    HfmStr message;
    HfmStr module_path;
    HfmStr file;
    std::uint32_t line;
    std::uint32_t reserved;
};

using HfmLogEnabledV1 = std::uint8_t (HIGHFLEET_LOGGING_CDECL *)(void*, std::uint32_t, HfmStr);
using HfmLogWriteV1 = void (HIGHFLEET_LOGGING_CDECL *)(void*, const HfmLogRecordV1*);
using HfmLogFlushV1 = void (HIGHFLEET_LOGGING_CDECL *)(void*);

struct HfmHostApiV1 {
    std::uint32_t abi_version;
    std::uint32_t struct_size;
    void* context;
    HfmLogEnabledV1 log_enabled;
    HfmLogWriteV1 log_write;
    HfmLogFlushV1 log_flush;
};

} // namespace ffi

namespace logging {

enum class level : std::uint32_t {
    error = 1,
    warn = 2,
    info = 3,
    debug = 4,
    trace = 5
};

namespace detail {

struct logger_state {
    std::atomic<unsigned> phase;
    ffi::HfmHostApiV1 host;

    logger_state() noexcept : phase(0), host{} {}
};

inline logger_state& logger() noexcept {
    // External-linkage inline functions share their local static across translation units.
    static logger_state state;
    return state;
}

inline const ffi::HfmHostApiV1* installed_host() noexcept {
    logger_state& state = logger();
    return state.phase.load(std::memory_order_acquire) == 2 ? &state.host : nullptr;
}

inline ffi::HfmStr borrow(const char* text) noexcept {
    return {reinterpret_cast<const std::uint8_t*>(text),
            text == nullptr ? 0 : std::strlen(text)};
}

inline std::uint32_t install_host(const ffi::HfmHostApiV1* host) noexcept {
    if (host == nullptr) {
        return ffi::HFM_STATUS_NULL_API;
    }
    if (host->abi_version != ffi::HFM_HOST_ABI_VERSION_1) {
        return ffi::HFM_STATUS_UNSUPPORTED_ABI;
    }
    if (host->struct_size < sizeof(ffi::HfmHostApiV1)) {
        return ffi::HFM_STATUS_API_TOO_SMALL;
    }
    if (host->log_write == nullptr) {
        return ffi::HFM_STATUS_MISSING_CALLBACK;
    }
    detail::logger_state& state = detail::logger();
    unsigned expected = 0;
    if (!state.phase.compare_exchange_strong(expected, 1, std::memory_order_relaxed)) {
        return ffi::HFM_STATUS_LOGGER_ALREADY_SET;
    }
    state.host = *host;
    // Readers access the immutable table only after publication is complete.
    state.phase.store(2, std::memory_order_release);
    return ffi::HFM_STATUS_OK;
}

} // namespace detail

inline bool enabled(level severity, const char* target = nullptr) {
    const ffi::HfmHostApiV1* host = detail::installed_host();
    return host != nullptr &&
        (host->log_enabled == nullptr ||
         host->log_enabled(host->context, static_cast<std::uint32_t>(severity),
                           detail::borrow(target)) != 0);
}

inline void flush() {
    const ffi::HfmHostApiV1* host = detail::installed_host();
    if (host != nullptr && host->log_flush != nullptr) {
        host->log_flush(host->context);
    }
}

inline void log(level severity, const char* target, const char* message) {
    const ffi::HfmHostApiV1* host = detail::installed_host();
    if (host == nullptr || !enabled(severity, target)) {
        return;
    }
    const ffi::HfmLogRecordV1 record = {
        sizeof(ffi::HfmLogRecordV1), static_cast<std::uint32_t>(severity),
        detail::borrow(target), detail::borrow(message),
        {nullptr, 0}, {nullptr, 0}, 0, 0
    };
    host->log_write(host->context, &record);
}

inline void error(const char* target, const char* message) {
    log(level::error, target, message);
}

inline void warn(const char* target, const char* message) {
    log(level::warn, target, message);
}

inline void info(const char* target, const char* message) {
    log(level::info, target, message);
}

inline void debug(const char* target, const char* message) {
    log(level::debug, target, message);
}

inline void trace(const char* target, const char* message) {
    log(level::trace, target, message);
}

inline void error(const char* message) {
    error(nullptr, message);
}

inline void warn(const char* message) {
    warn(nullptr, message);
}

inline void info(const char* message) {
    info(nullptr, message);
}

inline void debug(const char* message) {
    debug(nullptr, message);
}

inline void trace(const char* message) {
    trace(nullptr, message);
}

} // namespace logging
} // namespace highfleet

#define HIGHFLEET_SETUP_LOGGER()                                               \
    extern "C" HIGHFLEET_LOGGING_EXPORT std::uint32_t HIGHFLEET_LOGGING_CDECL   \
    highfleet_mod_setup_v1(const ::highfleet::ffi::HfmHostApiV1* host) {        \
        return ::highfleet::logging::detail::install_host(host);              \
    }

#endif // HIGHFLEET_LOG_HPP
