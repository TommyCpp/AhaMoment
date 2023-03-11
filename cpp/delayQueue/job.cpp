#include "job.h"

Job::Job(int jobId, long long jobExecutionTime) {
    id = jobId;
    executionTime = jobExecutionTime;
}

bool Job::operator<(const Job& other) const {
    return executionTime > other.executionTime;
}

long long Job::getDelay() const {
    auto now = std::chrono::system_clock::now();
    // return ms of delay between now and executionTime
    auto ms = std::chrono::duration_cast<std::chrono::milliseconds>(now.time_since_epoch()).count();
    return executionTime - ms;
}

bool Job::operator==(const Job &other) const {
    return id == other.id;
}
