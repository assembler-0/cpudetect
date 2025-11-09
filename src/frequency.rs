//! CPU Frequency Detection
//!
//! Detects CPU frequency information including base, max, and bus frequencies.

use crate::cpuid::{cpuid, is_leaf_supported};

#[derive(Debug, Clone)]
pub struct FrequencyInfo {
    pub base_mhz: Option<u32>,
    pub max_mhz: Option<u32>,
    pub bus_mhz: Option<u32>,
    pub tsc_mhz: Option<u32>,
}

impl FrequencyInfo {
    pub fn detect() -> Self {
        let mut info = Self {
            base_mhz: None,
            max_mhz: None,
            bus_mhz: None,
            tsc_mhz: None,
        };

        if is_leaf_supported(0x16) {
            let result = cpuid(0x16, 0);
            if result.eax != 0 {
                info.base_mhz = Some(result.eax & 0xFFFF);
            }
            if result.ebx != 0 {
                info.max_mhz = Some(result.ebx & 0xFFFF);
            }
            if result.ecx != 0 {
                info.bus_mhz = Some(result.ecx & 0xFFFF);
            }
        }

        if is_leaf_supported(0x15) {
            let result = cpuid(0x15, 0);
            if result.ebx != 0 && result.eax != 0 {
                let crystal_hz = if result.ecx != 0 {
                    result.ecx
                } else {
                    24_000_000
                };
                info.tsc_mhz = Some(
                    (crystal_hz as u64 * result.ebx as u64 / result.eax as u64 / 1_000_000) as u32,
                );
            }
        }

        info
    }
}
