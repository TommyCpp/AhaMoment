#pragma once

#include <chrono>

class Job {
public:
    int id;
    long long executionTime;

    Job(int jobId, long long jobExecutionTime);

    bool operator<(const Job& other) const;

    bool operator==(const Job& other) const;

    long long getDelay() const;

};