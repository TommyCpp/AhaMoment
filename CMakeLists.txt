cmake_minimum_required(VERSION 3.24)
project(AhaMoment)

set(CMAKE_CXX_STANDARD 17)
SET(CMAKE_CXX_FLAGS -pthread)

include_directories(cpp/delayQueue)

add_executable(AhaMoment
        cpp/delayQueue/Job.cpp
        cpp/delayQueue/Job.h
        cpp/delayQueue/main.cpp
        cpp/delayQueue/Worker.cpp
        cpp/delayQueue/Worker.h cpp/delayQueue/Timer.cpp cpp/delayQueue/Timer.h)
