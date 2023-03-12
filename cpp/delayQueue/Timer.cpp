//
// Created by zhongyang on 3/11/23.
//

#include <thread>
#include "Timer.h"
#include "Worker.h"

template <typename... Args>
Job<Args...> Timer<Args...>::generateJob(std::function<void(Args...)> jobFunc, std::tuple<Args...> jobArgs) {
    // run every 100ms
    std::this_thread::sleep_for(std::chrono::milliseconds(100));
    long long executionTime = rand() % 500 + getCurrentTime();
    Job<Args...> job(id++, executionTime, jobFunc, jobArgs);
    std::cout << "job id: " << job.id << " current time: " << getCurrentTime() << " execution time: " << job.executionTime << std::endl;
    return job;
}

template <typename... Args>
long long Timer<Args...>::getCurrentTime() {
    auto now = std::chrono::system_clock::now();
    return std::chrono::duration_cast<std::chrono::milliseconds>(now.time_since_epoch()).count();
}

template class Timer<int, std::string>;