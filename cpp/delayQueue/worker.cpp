#include "worker.h"

template <typename... Args>
void Worker<Args...>::put(Job<Args...> job) {
    std::unique_lock<std::mutex> lock(mtx);
    jobs.push(job);

    if (jobs.top() == job) {
        cv.notify_all();
    }
}

template <typename... Args>
Job<Args...> Worker<Args...>::take() {
    std::unique_lock<std::mutex> lock(mtx);

    while (jobs.empty()) {
        cv.wait(lock);
    }

    Job job = jobs.top();

    while (job.getDelay() > 0) {
        if (cv.wait_for(lock, std::chrono::milliseconds(job.getDelay())) == std::cv_status::timeout) {
            break;
        }

        job = jobs.top();
    }

    if (jobs.top() == job) {
        jobs.pop();
    }

    return job;
}

template <typename... Args>
void Worker<Args...>::deleteJob(int id) {
    std::unique_lock<std::mutex> lock(mtx);
    // delete job with id from queue
    std::priority_queue<Job<Args...>> temp;
    while (!jobs.empty()) {
        Job job = jobs.top();
        jobs.pop();
        if (job.id != id) {
            temp.push(job);
        }
    }
    jobs = temp;
}

template class Worker<int, std::string>;
