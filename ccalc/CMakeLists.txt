cmake_minimum_required(VERSION 3.13)
project(ccalc)

add_executable(ccalc
    src/main.c
    src/localmath.h
    src/localmath.c)

target_include_directories(ccalc PUBLIC
    ${CMAKE_SOURCE_DIR}/vendor/dynomath/include)
target_link_libraries(ccalc ${CMAKE_SOURCE_DIR}/vendor/dynomath/lib/libdynomath${CMAKE_SHARED_LIBRARY_SUFFIX})