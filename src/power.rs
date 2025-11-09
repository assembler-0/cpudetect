//! CPU Power Management Detection
//!
//! Comprehensive power management and thermal feature detection.

use crate::cpuid::{cpuid, is_leaf_supported};

#[derive(Debug, Clone)]
pub struct PowerInfo {
    pub digital_thermal_sensor: bool,
    pub turbo_boost: bool,
    pub arat: bool,
    pub pln: bool,
    pub pts: bool,
    pub hwp: bool,
    pub hwp_notification: bool,
    pub hwp_activity_window: bool,
    pub hwp_epp: bool,
    pub hwp_package: bool,
    pub hdc: bool,
    pub turbo_boost_max_3: bool,
    pub hwp_capabilities: bool,
    pub hwp_peci: bool,
    pub flexible_hwp: bool,
    pub hwp_fast_access: bool,
    pub hw_feedback: bool,
    pub ignore_idle_hwp: bool,
    pub thread_director: bool,
    pub therm_interrupt: bool,
    pub therm_status: bool,
    pub tm2: bool,
    pub num_interrupt_thresholds: u32,
}

impl PowerInfo {
    pub fn detect() -> Self {
        let mut info = Self {
            digital_thermal_sensor: false,
            turbo_boost: false,
            arat: false,
            pln: false,
            pts: false,
            hwp: false,
            hwp_notification: false,
            hwp_activity_window: false,
            hwp_epp: false,
            hwp_package: false,
            hdc: false,
            turbo_boost_max_3: false,
            hwp_capabilities: false,
            hwp_peci: false,
            flexible_hwp: false,
            hwp_fast_access: false,
            hw_feedback: false,
            ignore_idle_hwp: false,
            thread_director: false,
            therm_interrupt: false,
            therm_status: false,
            tm2: false,
            num_interrupt_thresholds: 0,
        };

        if is_leaf_supported(1) {
            let result = cpuid(1, 0);
            info.therm_status = (result.ecx & (1 << 3)) != 0;
            info.tm2 = (result.ecx & (1 << 8)) != 0;
            info.therm_interrupt = (result.edx & (1 << 22)) != 0;
        }

        if is_leaf_supported(6) {
            let result = cpuid(6, 0);

            info.digital_thermal_sensor = (result.eax & (1 << 0)) != 0;
            info.turbo_boost = (result.eax & (1 << 1)) != 0;
            info.arat = (result.eax & (1 << 2)) != 0;
            info.pln = (result.eax & (1 << 4)) != 0;
            info.pts = (result.eax & (1 << 6)) != 0;
            info.hwp = (result.eax & (1 << 7)) != 0;
            info.hwp_notification = (result.eax & (1 << 8)) != 0;
            info.hwp_activity_window = (result.eax & (1 << 9)) != 0;
            info.hwp_epp = (result.eax & (1 << 10)) != 0;
            info.hwp_package = (result.eax & (1 << 11)) != 0;
            info.hdc = (result.eax & (1 << 13)) != 0;
            info.turbo_boost_max_3 = (result.eax & (1 << 14)) != 0;
            info.hwp_capabilities = (result.eax & (1 << 15)) != 0;
            info.hwp_peci = (result.eax & (1 << 16)) != 0;
            info.flexible_hwp = (result.eax & (1 << 17)) != 0;
            info.hwp_fast_access = (result.eax & (1 << 18)) != 0;
            info.hw_feedback = (result.eax & (1 << 19)) != 0;
            info.ignore_idle_hwp = (result.eax & (1 << 20)) != 0;
            info.thread_director = (result.eax & (1 << 23)) != 0;

            info.num_interrupt_thresholds = (result.ebx & 0xF) as u32;
        }

        info
    }
}
