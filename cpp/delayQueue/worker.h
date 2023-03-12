#pragma once

#include <queue>
#include <mutex>
#include <condition_variable>
#include "job.h"

template <typename... Args>
class Worker {
public:
    std::priority_queue<Job<Args...>> jobs;
    std::mutex mtx;
    std::condition_variable cv;

    void put(Job<Args...> job);

    Job<Args...> take();

    void deleteJob(int id);
};