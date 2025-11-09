//! Platform Information Detection
//!
//! Detects platform-specific information and capabilities.

use crate::cpuid::{cpuid, is_leaf_supported};

#[derive(Debug, Clone)]
pub struct PlatformInfo {
    pub max_cpuid_leaf: u32,
    pub max_extended_leaf: u32,
    pub microcode_update: bool,
    pub machine_check: bool,
    pub mtrr: bool,
    pub page_attribute_table: bool,
    pub page_size_extension: bool,
    pub time_stamp_counter: bool,
    pub model_specific_registers: bool,
    pub apic_on_chip: bool,
    pub x2apic: bool,
    pub local_apic_timer_always_running: bool,
    pub tsc_invariant: bool,
    pub tsc_deadline: bool,
    pub xapic: bool,
}

impl PlatformInfo {
    pub fn detect() -> Self {
        let max_cpuid = cpuid(0, 0).eax;
        let max_extended = cpuid(0x8000_0000, 0).eax;

        let mut info = Self {
            max_cpuid_leaf: max_cpuid,
            max_extended_leaf: max_extended,
            microcode_update: false,
            machine_check: false,
            mtrr: false,
            page_attribute_table: false,
            page_size_extension: false,
            time_stamp_counter: false,
            model_specific_registers: false,
            apic_on_chip: false,
            x2apic: false,
            local_apic_timer_always_running: false,
            tsc_invariant: false,
            tsc_deadline: false,
            xapic: false,
        };

        if is_leaf_supported(1) {
            let result = cpuid(1, 0);

            info.time_stamp_counter = (result.edx & (1 << 4)) != 0;
            info.model_specific_registers = (result.edx & (1 << 5)) != 0;
            info.apic_on_chip = (result.edx & (1 << 9)) != 0;
            info.mtrr = (result.edx & (1 << 12)) != 0;
            info.machine_check = (result.edx & (1 << 14)) != 0;
            info.page_attribute_table = (result.edx & (1 << 16)) != 0;
            info.page_size_extension = (result.edx & (1 << 17)) != 0;

            info.x2apic = (result.ecx & (1 << 21)) != 0;
            info.tsc_deadline = (result.ecx & (1 << 24)) != 0;
            info.xapic = (result.ecx & (1 << 21)) != 0;
        }

        if is_leaf_supported(6) {
            let result = cpuid(6, 0);
            info.local_apic_timer_always_running = (result.eax & (1 << 2)) != 0;
        }

        if is_leaf_supported(0x8000_0007) {
            let result = cpuid(0x8000_0007, 0);
            info.tsc_invariant = (result.edx & (1 << 8)) != 0;
        }

        info
    }
}
