#include "Job.h"

Job::Job(int jobId, int jobExecutionTime) {
    id = jobId;
    executionTime = jobExecutionTime;
}

bool Job::operator<(const Job& other) const {
    return executionTime > other.executionTime;
}

long long Job::getDelay() const {
    auto now = std::chrono::system_clock::now();
    return std::chrono::duration_cast<std::chrono::milliseconds>(getExecutionTime() - now.time_since_epoch()).count();
}

std::chrono::system_clock::time_point Job::getExecutionTime() const {
    return std::chrono::system_clock::time_point(std::chrono::milliseconds(executionTime));
}