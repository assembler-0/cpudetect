//! MSR (Model-Specific Register) Information
//!
//! Provides information about MSR support (read-only, no actual MSR access).
//! Cross-platform safe - only reports capabilities, doesn't access MSRs.

use crate::cpuid::{cpuid, is_leaf_supported};

#[derive(Debug, Clone)]
pub struct MsrInfo {
    pub msr_support: bool,
    pub rdmsr_wrmsr: bool,
    pub msr_platform_info: bool,
    pub msr_temperature: bool,
    pub msr_perf_status: bool,
    pub msr_perf_ctl: bool,
    pub msr_misc_enable: bool,
    pub msr_energy_perf_bias: bool,
    pub msr_turbo_ratio_limit: bool,
}

impl MsrInfo {
    pub fn detect() -> Self {
        let mut info = Self {
            msr_support: false,
            rdmsr_wrmsr: false,
            msr_platform_info: false,
            msr_temperature: false,
            msr_perf_status: false,
            msr_perf_ctl: false,
            msr_misc_enable: false,
            msr_energy_perf_bias: false,
            msr_turbo_ratio_limit: false,
        };

        if is_leaf_supported(1) {
            let result = cpuid(1, 0);
            info.msr_support = (result.edx & (1 << 5)) != 0;
            info.rdmsr_wrmsr = info.msr_support;
        }

        if is_leaf_supported(6) {
            let result = cpuid(6, 0);
            info.msr_temperature = (result.eax & (1 << 0)) != 0;
            info.msr_turbo_ratio_limit = (result.eax & (1 << 1)) != 0;
            info.msr_energy_perf_bias = (result.ecx & (1 << 3)) != 0;
        }

        if is_leaf_supported(7) {
            let result = cpuid(7, 0);
            info.msr_platform_info = (result.ecx & (1 << 15)) != 0;
        }

        info.msr_perf_status = info.msr_support;
        info.msr_perf_ctl = info.msr_support;
        info.msr_misc_enable = info.msr_support;

        info
    }
}
