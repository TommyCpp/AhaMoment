cmake_minimum_required(VERSION 3.24)
project(comparator)

set(CMAKE_CXX_STANDARD 17)
SET(CMAKE_CXX_FLAGS -pthread)

add_executable(comparator
        main.cpp)