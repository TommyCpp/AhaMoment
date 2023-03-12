//
// Created by zhongyang on 3/11/23.
//

#ifndef AHAMOMENT_TIMER_H
#define AHAMOMENT_TIMER_H

#include <cstdlib>
#include <iostream>
#include "Job.h"
#include "Worker.h"

// timer generate job every 100ms, the execution time of the job is between 0 to 500ms from the job's creation time. everytime 
// timer generate a job it will print it's job id, current time and future execution time.
template <typename... Args>
class Timer {
public:
    Timer() {
        srand(time(nullptr));
        id = 0;
    }

    Job<Args...> generateJob(std::function<void(Args...)> jobFunc, std::tuple<Args...> jobArgs);

private:
    int id;

    long long getCurrentTime();


};

#endif //AHAMOMENT_TIMER_H
