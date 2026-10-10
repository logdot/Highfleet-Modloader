# HighFleet C++ logging SDK

A C++11 header-only logging API for mods.
Add `sdk/include` to your include path.
Highfleet and the modloader run on x86_64 Windows, so your mod DLL must also be built for x86_64 Windows.

## Use in a mod

Put the setup macro at global scope in **exactly one `.cpp` file**:

```cpp
#include <highfleet/log.hpp>

HIGHFLEET_SETUP_LOGGER()

extern "C" __declspec(dllexport) bool __cdecl init() {
    highfleet::logging::info("Ready!");
    highfleet::logging::warn("Optional configuration missing");
    return true;
}
```

Every other translation unit can include the header and log without repeating the macro.

Initialize your mod in `init`, where logging is already available.
If you choose to initialize in `DllMain` instead, do not log there: logging is unavailable before setup, and later calls under the loader lock can deadlock.

## Logging API

All user-facing names are in `highfleet::logging`:

- `error(message)`, `warn(message)`, `info(message)`, `debug(message)`, `trace(message)`.
- `enabled(level)`: ask the host before doing expensive message formatting.
- `flush()`: flush the modloader's buffered log output; does nothing before setup.
- `enum class level : std::uint32_t`: error = 1, warn = 2, info = 3, debug = 4, trace = 5.

```cpp
if (highfleet::logging::enabled(highfleet::logging::level::debug)) {
    // Build your diagnostic string here, then pass its c_str() to debug().
}
```

The functions accept UTF-8, NUL-terminated strings, borrow them synchronously, and do not format or allocate.
Keep the string alive until the call returns.
Null strings are treated as empty; embedded NUL bytes terminate the text.
Arbitrary invalid pointers are not made safe, and the SDK does not validate UTF-8.

For callers that already have diagnostic targets, each named helper also accepts `(target, message)`.
The generic entry point is `log(level, target, message)`, and `enabled(level, target)` accepts a target too.
Targets are forwarded as metadata, but **the host identifies and filters mods by their DLL stem**, not by these caller-controlled strings.
Most mods should just use the message-only helpers.

## Filtering and output

The SDK uses the modloader's logging configuration; see [Logging configuration](../README.md#logging-configuration).

## Host ABI details

Mods only need the macro and logging functions; they do not implement a logger.
`HIGHFLEET_SETUP_LOGGER()` emits this Windows export:

```cpp
extern "C" __declspec(dllexport) std::uint32_t __cdecl highfleet_mod_setup_v1(
    const highfleet::ffi::HfmHostApiV1* host);
```

The raw types in `highfleet::ffi` mirror `crates/highfleet-mod-api/src/ffi.rs`.
The version-1 table carries an opaque context, an optional enabled callback, a required write callback, and an optional flush callback.
Records carry byte-counted UTF-8 views (`HfmStr`), level, size, target/message, and empty source metadata.
No C++ objects, Rust traits, ownership, or strings requiring a shared allocator cross the ABI.

Setup validates version, size, and required callbacks, then copies the table.
The table itself only needs to live through setup.
Its context and callback code must remain valid and thread-safe for the entire lifetime of mod logging.
Callbacks must not throw or unwind across the C boundary.
Each record and its strings are borrowed only until the synchronous callback returns.

Installation succeeds once, using an atomic claim followed by release/acquire publication of the immutable copied table.
There is no replacement/uninstall API.
An external-linkage inline function's local static shares state across translation units under C++11.
Calls before publication are safe no-ops.
Setup status codes match Rust: 0 success, 1 null table, 2 unsupported version, 3 short table, 4 missing write callback, 5 already installed; `UINT32_MAX` is reserved for a Rust setup panic.

C++ and Rust mods use the same `highfleet_mod_setup_v1` export.
Non-Windows builds omit Windows export/calling-convention annotations for tests.

## Standalone tests

With CMake 3.20 or later, from the repository root:

```sh
cmake -S sdk -B build/cpp-sdk
cmake --build build/cpp-sdk --config Release
ctest --test-dir build/cpp-sdk -C Release --output-on-failure
```

Or directly with a C++11 compiler:

```sh
mkdir -p sdk/build
c++ -std=c++11 -Wall -Wextra -Wpedantic -Werror -Isdk/include \
    sdk/tests/log_test.cpp sdk/tests/log_other.cpp -o sdk/build/log_test
sdk/build/log_test
sdk/build/log_test optional
```

Both test modes use explicit checks that remain active with `NDEBUG`.
The optional-callback case runs in a separate process because installation is intentionally one-shot.
Tests cover ABI layouts matching Rust's assertions, setup rejection, table copying, synchronous forwarding/context, level values, filtering, flush, UTF-8/null text, and cross-translation-unit state.
