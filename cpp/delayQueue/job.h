#pragma once

#include <chrono>
#include <functional>

template <typename... Args>
class Job {
public:
    int id;
    long long executionTime;
    std::function<void(Args...)> jobFunc_;
    std::tuple<Args...> jobArgs_;

    Job(int id, long long jobExecutionTime, std::function<void(Args...)> jobFunc, std::tuple<Args...> jobArgs);


    bool operator<(const Job& other) const;

    bool operator==(const Job& other) const;

    long long getDelay() const;

    void execute();
};