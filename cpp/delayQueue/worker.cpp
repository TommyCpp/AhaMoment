#include "Worker.h"

void Worker::put(Job job) {
    std::unique_lock<std::mutex> lock(mtx);
    jobs.push(job);

    if (jobs.top() == job) {
        cv.notify_all();
    }
}

Job Worker::take() {
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