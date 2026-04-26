#ifndef RAYTRACING_HARDWARE_DETECTOR_H
#define RAYTRACING_HARDWARE_DETECTOR_H

#include <string>

struct CPUInfo
{
    std::string model;
    unsigned logicalCores;
    unsigned perfCores;   // P-cores (0 if unavailable)
    unsigned effiCores;   // E-cores (0 if unavailable)
    bool hasAVX;
    bool hasAVX2;
    bool hasNEON;

    void CPU_Detect();
};

#endif
