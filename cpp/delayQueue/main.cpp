#include <iostream>
#include <thread>
#include "worker.h"
#include "job.h"

void processJob(Worker& worker) {
    while (true) {
        Job job = worker.take();
        // process job here
        std::cout << "Processing job " << job.id << std::endl;
    }
}

void addJobs(Worker& worker) {
    for (int i = 0; i < 10; i++) {
        int executionTime = std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now().time_since_epoch()).count() + i * 1000;
        Job job(i, executionTime);
        worker.put(job);
    }
}

int main() {
    Worker worker;

    std::thread t1(processJob, std::ref(worker));
    std::thread t2(addJobs, std::ref(worker));

    t1.join();
    t2.join();

    return 0;
}