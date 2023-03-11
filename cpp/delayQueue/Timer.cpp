//
// Created by zhongyang on 3/11/23.
//

#include <thread>
#include "Timer.h"
#include "worker.h"

Job Timer::generateJob() {
    // run every 100ms
    std::this_thread::sleep_for(std::chrono::milliseconds(100));
    long long executionTime = rand() % 500 + getCurrentTime();
    Job job(id++, executionTime);
    std::cout << "job id: " << job.id << " current time: " << getCurrentTime() << " execution time: " << job.executionTime << std::endl;
    return job;
}

long long Timer::getCurrentTime() {
    auto now = std::chrono::system_clock::now();
    return std::chrono::duration_cast<std::chrono::milliseconds>(now.time_since_epoch()).count();
}