cmake_minimum_required(VERSION 3.28)
project(PI_cli_testing)

set(CMAKE_CXX_STANDARD 20)

add_library(PI_cli_testing STATIC sig_lib/library.cpp)
