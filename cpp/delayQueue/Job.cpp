#include "Job.h"


template<typename... Args>
bool Job<Args...>::operator<(const Job<Args...> &other) const {
    return executionTime > other.executionTime;
}

template<typename... Args>
long long Job<Args...>::getDelay() const {
    auto now = std::chrono::system_clock::now();
    // return ms of delay between now and executionTime
    auto ms = std::chrono::duration_cast<std::chrono::milliseconds>(now.time_since_epoch()).count();
    return executionTime - ms;
}

template<typename... Args>
bool Job<Args...>::operator==(const Job<Args...> &other) const {
    return id == other.id;
}

template<typename... Args>
Job<Args...>::Job(int id, long long jobExecutionTime, std::function<void(Args...)> jobFunc, std::tuple<Args...> jobArgs)
        :id(id), jobFunc_(jobFunc), jobArgs_(jobArgs), executionTime(jobExecutionTime) {}


template<typename... Args>
void Job<Args...>::execute() {
    std::apply(jobFunc_, jobArgs_);
}

template class Job<int, std::string>;