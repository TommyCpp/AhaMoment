cmake_minimum_required(VERSION 3.24)
project(delayedQueue)

set(CMAKE_CXX_STANDARD 17)
SET(CMAKE_CXX_FLAGS -pthread)

include_directories(.)

add_executable(delayedQueue
        Job.cpp
        Job.h
        main.cpp
        Worker.cpp
        Worker.h Timer.cpp Timer.h)