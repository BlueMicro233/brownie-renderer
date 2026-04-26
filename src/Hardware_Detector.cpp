#include "Hardware_Detector.hpp"
#include <iostream>
#include <cstring>
#include <thread>
#include <fstream>
#include <sstream>

// ---------------------------------------------------------------------------
// Platform-specific CPU model string
// ---------------------------------------------------------------------------
#if defined(__APPLE__)

#include <sys/sysctl.h>

static std::string Get_CPU_Model()
{
    char buf[128];
    size_t len = sizeof(buf);
    if (sysctlbyname("machdep.cpu.brand_string", buf, &len, nullptr, 0) == 0)
        return buf;
    return "Unknown Apple CPU";
}

#elif defined(__linux__)

static std::string Get_CPU_Model()
{
    std::ifstream cpuinfo("/proc/cpuinfo");
    if (!cpuinfo.is_open())
        return "Unknown";

    std::string line;
    while (std::getline(cpuinfo, line))
    {
        if (line.find("model name") != std::string::npos)
        {
            auto pos = line.find(':');
            if (pos != std::string::npos)
            {
                // Trim leading whitespace
                std::string name = line.substr(pos + 1);
                name.erase(0, name.find_first_not_of(" \t"));
                return name;
            }
        }
    }
    return "Unknown x86 CPU";
}

#else

static std::string Get_CPU_Model()
{
    return "Unknown CPU";
}

#endif

// ---------------------------------------------------------------------------
// Platform-specific core counts
// ---------------------------------------------------------------------------
#if defined(__APPLE__)

static unsigned Get_Performance_Cores()
{
#ifdef __aarch64__
    int cores = 0;
    size_t size = sizeof(cores);
    sysctlbyname("hw.perflevel0.physicalcpu", &cores, &size, nullptr, 0);
    return static_cast<unsigned>(cores > 0 ? cores : 0);
#else
    return 0;   // Intel Macs don't expose P/E count via sysctl
#endif
}

static unsigned Get_Efficiency_Cores()
{
#ifdef __aarch64__
    int cores = 0;
    size_t size = sizeof(cores);
    sysctlbyname("hw.perflevel1.physicalcpu", &cores, &size, nullptr, 0);
    return static_cast<unsigned>(cores > 0 ? cores : 0);
#else
    return 0;
#endif
}

#else

static unsigned Get_Performance_Cores() { return 0; }
static unsigned Get_Efficiency_Cores()  { return 0; }

#endif

// ---------------------------------------------------------------------------
// SIMD feature detection
// ---------------------------------------------------------------------------
#if defined(__x86_64__) || defined(_M_X64)

#include <cpuid.h>

static bool Supports_AVX()
{
    unsigned int eax, ebx, ecx, edx;
    __cpuid(1, eax, ebx, ecx, edx);
    return (ecx & (1U << 28)) != 0;
}

static bool Supports_AVX2()
{
    unsigned int eax, ebx, ecx, edx;
    __cpuid_count(7, 0, eax, ebx, ecx, edx);
    return (ebx & (1U << 5)) != 0;
}

static bool Supports_NEON() { return false; }

#elif defined(__aarch64__)

static bool Supports_AVX()   { return false; }
static bool Supports_AVX2()  { return false; }

#if defined(__APPLE__)
// NEON is baseline on Apple Silicon
static bool Supports_NEON()  { return true; }
#elif defined(__linux__)
static bool Supports_NEON()
{
    std::ifstream cpuinfo("/proc/cpuinfo");
    if (!cpuinfo.is_open())
        return false;
    std::string line;
    while (std::getline(cpuinfo, line))
    {
        if (line.find("Features") != std::string::npos)
        {
            if (line.find("neon") != std::string::npos ||
                line.find("asimd") != std::string::npos)
                return true;
        }
    }
    return false;
}
#else
static bool Supports_NEON()  { return false; }
#endif

#else

static bool Supports_AVX()   { return false; }
static bool Supports_AVX2()  { return false; }
static bool Supports_NEON()  { return false; }

#endif

// ---------------------------------------------------------------------------
// CPUInfo::CPU_Detect
// ---------------------------------------------------------------------------
void CPUInfo::CPU_Detect()
{
    model        = Get_CPU_Model();
    logicalCores = std::thread::hardware_concurrency();
    perfCores    = Get_Performance_Cores();
    effiCores    = Get_Efficiency_Cores();
    hasAVX       = Supports_AVX();
    hasAVX2      = Supports_AVX2();
    hasNEON      = Supports_NEON();
}
