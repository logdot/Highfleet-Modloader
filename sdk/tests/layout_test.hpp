#ifndef HIGHFLEET_LAYOUT_TEST_HPP
#define HIGHFLEET_LAYOUT_TEST_HPP

#include <highfleet/log.hpp>
#include <type_traits>

namespace layout_test {
using namespace highfleet::ffi;
constexpr std::size_t pointer_size = sizeof(void*);

static_assert(std::is_standard_layout<HfmStr>::value, "C string layout");
static_assert(std::is_standard_layout<HfmLogRecordV1>::value, "C record layout");
static_assert(std::is_standard_layout<HfmHostApiV1>::value, "C host layout");
static_assert(std::is_trivially_copyable<HfmHostApiV1>::value, "host table can be copied");
static_assert(offsetof(HfmStr, ptr) == 0, "string pointer offset");
static_assert(offsetof(HfmStr, len) == pointer_size, "string length offset");
static_assert(sizeof(HfmStr) == pointer_size * 2, "string size");
static_assert(alignof(HfmStr) == pointer_size, "string alignment");

static_assert(offsetof(HfmLogRecordV1, struct_size) == 0, "record size offset");
static_assert(offsetof(HfmLogRecordV1, level) == 4, "record level offset");
static_assert(offsetof(HfmLogRecordV1, target) == 8, "record target offset");
static_assert(offsetof(HfmLogRecordV1, message) == 8 + pointer_size * 2, "record message offset");
static_assert(offsetof(HfmLogRecordV1, module_path) == 8 + pointer_size * 4, "record module offset");
static_assert(offsetof(HfmLogRecordV1, file) == 8 + pointer_size * 6, "record file offset");
static_assert(offsetof(HfmLogRecordV1, line) == 8 + pointer_size * 8, "record line offset");
static_assert(offsetof(HfmLogRecordV1, reserved) == 12 + pointer_size * 8, "record reserved offset");
static_assert(sizeof(HfmLogRecordV1) == 16 + pointer_size * 8, "record size");

static_assert(offsetof(HfmHostApiV1, abi_version) == 0, "host version offset");
static_assert(offsetof(HfmHostApiV1, struct_size) == 4, "host size offset");
static_assert(offsetof(HfmHostApiV1, context) == 8, "host context offset");
static_assert(offsetof(HfmHostApiV1, log_enabled) == 8 + pointer_size, "host enabled offset");
static_assert(offsetof(HfmHostApiV1, log_write) == 8 + pointer_size * 2, "host write offset");
static_assert(offsetof(HfmHostApiV1, log_flush) == 8 + pointer_size * 3, "host flush offset");
static_assert(sizeof(HfmHostApiV1) == 8 + pointer_size * 4, "host size");
} // namespace layout_test

#endif
