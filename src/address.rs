//! Address Size Detection
//!
//! Detects physical and virtual address bit widths.

use crate::cpuid::{cpuid, is_leaf_supported};

#[derive(Debug, Clone)]
pub struct AddressInfo {
    pub physical_bits: u32,
    pub virtual_bits: u32,
    pub guest_physical_bits: Option<u32>,
}

impl AddressInfo {
    pub fn detect() -> Self {
        let mut info = Self {
            physical_bits: 36,
            virtual_bits: 48,
            guest_physical_bits: None,
        };

        if is_leaf_supported(0x8000_0008) {
            let result = cpuid(0x8000_0008, 0);
            info.physical_bits = (result.eax & 0xFF) as u32;
            info.virtual_bits = ((result.eax >> 8) & 0xFF) as u32;

            let guest_phys = ((result.eax >> 16) & 0xFF) as u32;
            if guest_phys > 0 {
                info.guest_physical_bits = Some(guest_phys);
            }
        }

        info
    }
}
