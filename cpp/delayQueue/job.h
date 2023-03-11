#pragma once

#include <chrono>

class Job {
public:
    int id;
    int executionTime;

    Job(int jobId, int jobExecutionTime);

    bool operator<(const Job& other) const;

    long long getDelay() const;

    std::chrono::system_clock::time_point getExecutionTime() const;
};