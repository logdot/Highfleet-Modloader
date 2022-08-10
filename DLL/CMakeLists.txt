cmake_minimum_required(VERSION 3.10)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

enable_language(CXX ASM_MASM)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/bin)

project(ShipworksExtended VERSION 0.1.1)
add_library(${PROJECT_NAME} SHARED dllmain.cpp mem.cpp pch.cpp shipworks.cpp shipworksASM.asm)