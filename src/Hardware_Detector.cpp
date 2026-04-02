#include "Hardware_Detector.hpp"
#include <cpuid.h>
#include <iostream>
#include <cstring>
#include <thread>

// x86-based CPU info detection
std::string Get_CPU_Model()
{
    unsigned int eax, ebx, ecx, edx;
    char model[0x40];

    __cpuid(0x80000002, eax, ebx, ecx, edx);
    memcpy(model, &eax, 4); memcpy(model+4, &ebx, 4);
    memcpy(model+8, &ecx, 4); memcpy(model+12, &edx, 4);

    __cpuid(0x80000003, eax, ebx, ecx, edx);
    memcpy(model+16, &eax, 4); memcpy(model+20, &ebx, 4);
    memcpy(model+24, &ecx, 4); memcpy(model+28, &edx, 4);

    __cpuid(0x80000004, eax, ebx, ecx, edx);
    memcpy(model+32, &eax, 4); memcpy(model+36, &ebx, 4);
    memcpy(model+40, &ecx, 4); memcpy(model+44, &edx, 4);

    model[48] = '\0';

    return model;
}

bool Supports_AVX()
{
    unsigned int eax, ebx, ecx, edx;
    __cpuid(1, eax, ebx, ecx, edx);
    return (ecx & (1 << 28));
}

bool Supports_AVX2()
{
    unsigned int eax, ebx, ecx, edx;
    __cpuid_count(7, 0, eax, ebx, ecx, edx);
    return (ebx & (1 << 5)); // AVX2 bit
}

void CPUInfo::CPU_Detect()
{
    model = Get_CPU_Model();
    logicalCores = std::thread::hardware_concurrency();
    hasAVX = Supports_AVX();
    hasAVX2 = Supports_AVX2();
}

