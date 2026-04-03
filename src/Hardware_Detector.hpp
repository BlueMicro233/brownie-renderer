#ifndef RAYTRACING_HARDWARE_DETECTOR_H
#define RAYTRACING_HARDWARE_DETECTOR_H

#endif //RAYTRACING_HARDWARE_DETECTOR_H

#include <string>

struct CPUInfo
{
    std::string model;
    unsigned logicalCores;
    unsigned perfCores;
    unsigned effiCores;

    void CPU_Detect();
};