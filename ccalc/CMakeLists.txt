cmake_minimum_required(VERSION 3.13)
project(ccalc)

add_executable(ccalc
    src/main.c
    src/localmath.h
    src/localmath.c)

target_include_directories(ccalc PUBLIC
    /Users/andrew/src/test/safer_dynamic/dynomath/include)
target_link_libraries(ccalc /Users/andrew/src/test/safer_dynamic/target/release/libdynomath.dylib)
