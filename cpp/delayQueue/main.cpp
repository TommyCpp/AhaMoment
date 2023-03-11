#include <iostream>
#include <thread>
#include "worker.h"
#include "job.h"
#include "Timer.h"

void processJob(Worker& worker) {
    while (true) {
        Job job = worker.take();
        // process job here
        std::cout << "Processing job " << job.id << " Lag: " << job.getDelay() << "ms" << std::endl;
    }
}

void addJobs(Worker& worker) {
    Timer timer;
    for (int i = 0; i < 10000; i++) {
        Job job = timer.generateJob();
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