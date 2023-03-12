#include <iostream>
#include <thread>
#include "Worker.h"
#include "Job.h"
#include "Timer.h"

void print(int x, const std::string& str) {
    std::cout << "x = " << x << ", str = " << str << std::endl;
}

auto jobFunc = std::function<void(int, const std::string&)>(print);
std::tuple<int, std::string> jobArgs(42, "hello");

void processJob(Worker<int, std::string>& worker) {
    while (true) {
        Job<int, std::string> job = worker.take();
        // process job here
        job.execute();
        std::cout << "Processing job " << job.id << " Lag: " << job.getDelay() << "ms" << std::endl;
    }
}

void addJobs(Worker<int, std::string>& worker) {
    Timer<int, std::string> timer;
    for (int i = 0; i < 10000; i++) {
        auto job = timer.generateJob(jobFunc, jobArgs);
        worker.put(job);
    }
}

int main() {
    Worker<int, std::string> worker;

    std::thread t1(processJob, std::ref(worker));
    std::thread t2(addJobs, std::ref(worker));

    t1.join();
    t2.join();

    return 0;
}