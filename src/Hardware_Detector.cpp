#include "Hardware_Detector.hpp"
#include <iostream>
#include <sys/sysctl.h>

std::string Get_CPU_Model()
{
    char model[48];
    size_t size = sizeof(model);

    sysctlbyname("machdep.cpu.brand_string", &model, &size, NULL, 0);
    return std::string(model);
}

int Get_Logical_Cores() {
    int cores;
    size_t size = sizeof(cores);
    sysctlbyname("hw.logicalcpu", &cores, &size, NULL, 0);
    return cores;
}

int Get_Performance_Cores()
{
    int cores;
    size_t size = sizeof(cores);
    sysctlbyname("hw.perflevel0.physicalcpu", &cores, &size, NULL, 0);
    return cores;
}

int Get_Efficiency_Cores()
{
    int cores;
    size_t size = sizeof(cores);
    sysctlbyname("hw.perflevel1.physicalcpu", &cores, &size, NULL, 0);
    return cores;
}

void CPUInfo::CPU_Detect()
{
    model = Get_CPU_Model();
    logicalCores = Get_Logical_Cores();
    perfCores = Get_Performance_Cores();
    effiCores = Get_Efficiency_Cores();
}