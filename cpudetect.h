#ifndef CPUDETECT_H
#define CPUDETECT_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t family;
    uint32_t model;
    uint32_t stepping;
    int32_t vendor_enum; // 0=Intel, 1=AMD, 2=Hygon, 3=Zhaoxin, -1=Unknown
} CpuVendorFFI;

typedef struct {
    uint32_t logical_processors;
    uint32_t physical_cores;
    uint32_t threads_per_core;
    bool has_hyperthreading;
    bool hybrid;
} CpuTopologyFFI;

typedef struct {
    uint64_t basic_low;
    uint64_t basic_high;
} CpuFeaturesFFI;

typedef struct {
    uint32_t physical_bits;
    uint32_t virtual_bits;
    uint32_t guest_physical_bits; // 0 if not supported
} CpuAddressFFI;

typedef struct {
    uint32_t base_mhz; // 0 if unknown
    uint32_t max_mhz;  // 0 if unknown
    uint32_t bus_mhz;  // 0 if unknown
    uint32_t tsc_mhz;  // 0 if unknown
} CpuFrequencyFFI;

int32_t cpudetect_detect_vendor(CpuVendorFFI *out);
int32_t cpudetect_get_vendor_string(char *buffer, size_t len);
int32_t cpudetect_get_brand_string(char *buffer, size_t len);
int32_t cpudetect_detect_topology(CpuTopologyFFI *out);
int32_t cpudetect_detect_features(CpuFeaturesFFI *out);
bool cpudetect_has_feature(const char *name);
int32_t cpudetect_detect_address(CpuAddressFFI *out);
int32_t cpudetect_detect_frequency(CpuFrequencyFFI *out);

#ifdef __cplusplus
}
#endif

#endif // CPUDETECT_H
