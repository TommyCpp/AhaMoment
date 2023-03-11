#pragma once

#include <queue>
#include <mutex>
#include <condition_variable>
#include "job.h"

class Worker {
public:
    std::priority_queue<Job> jobs;
    std::mutex mtx;
    std::condition_variable cv;

    void put(Job job);

    Job take();

    void deleteJob(int id);

};