cmake_minimum_required(VERSION 3.20)
project(highfleet_cpp_sdk LANGUAGES CXX)

add_library(highfleet_logging INTERFACE)
target_include_directories(highfleet_logging INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}/include")
target_compile_features(highfleet_logging INTERFACE cxx_std_11)

include(CTest)
if(BUILD_TESTING)
    add_executable(highfleet_log_test tests/log_test.cpp tests/log_other.cpp)
    target_link_libraries(highfleet_log_test PRIVATE highfleet_logging)
    set_target_properties(highfleet_log_test PROPERTIES
        CXX_STANDARD 11
        CXX_STANDARD_REQUIRED YES
        CXX_EXTENSIONS NO)
    if(MSVC)
        target_compile_options(highfleet_log_test PRIVATE /W4 /WX /permissive-)
    else()
        target_compile_options(highfleet_log_test PRIVATE -Wall -Wextra -Wpedantic -Werror)
    endif()
    add_test(NAME logging COMMAND highfleet_log_test)
    add_test(NAME optional_callbacks COMMAND highfleet_log_test optional)
endif()
