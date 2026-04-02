#ifndef RAYTRACING_HARDWARE_DETECTOR_H
#define RAYTRACING_HARDWARE_DETECTOR_H

#endif //RAYTRACING_HARDWARE_DETECTOR_H

#include <string>

struct CPUInfo
{
    std::string model;
    unsigned logicalCores;
    bool hasAVX;
    bool hasAVX2;

    void CPU_Detect();
};