//! CPU Thermal and Power Management Detection
//! 
//! Detects thermal monitoring and power management features.

use crate::cpuid::{cpuid, is_leaf_supported};

#[derive(Debug, Clone)]
pub struct ThermalInfo {
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
}

impl ThermalInfo {
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
        };

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
        }

        info
    }
}
