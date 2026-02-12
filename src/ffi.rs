//! C FFI for cpudetect
//!
//! Provides a C-compatible interface for the CPU detection library.

use crate::*;
use core::ptr;

#[repr(C)]
pub struct CpuVendorFFI {
    pub family: u32,
    pub model: u32,
    pub stepping: u32,
    pub vendor_enum: i32, // 0=Intel, 1=AMD, 2=Hygon, 3=Zhaoxin, -1=Unknown
}

#[repr(C)]
pub struct CpuTopologyFFI {
    pub logical_processors: u32,
    pub physical_cores: u32,
    pub threads_per_core: u32,
    pub has_hyperthreading: bool,
    pub hybrid: bool,
}

#[repr(C)]
pub struct CpuFeaturesFFI {
    pub basic_low: u64,
    pub basic_high: u64,
}

#[repr(C)]
pub struct CpuAddressFFI {
    pub physical_bits: u32,
    pub virtual_bits: u32,
    pub guest_physical_bits: u32, // 0 if not supported
}

#[repr(C)]
pub struct CpuFrequencyFFI {
    pub base_mhz: u32, // 0 if unknown
    pub max_mhz: u32,  // 0 if unknown
    pub bus_mhz: u32,  // 0 if unknown
    pub tsc_mhz: u32,  // 0 if unknown
}

#[unsafe(no_mangle)]
pub extern "C" fn cpudetect_detect_vendor(out: *mut CpuVendorFFI) -> i32 {
    if out.is_null() {
        return -1;
    }

    let info = VendorInfo::detect();
    unsafe {
        (*out).family = info.family;
        (*out).model = info.model;
        (*out).stepping = info.stepping;
        (*out).vendor_enum = match info.vendor {
            CpuVendor::Intel => 0,
            CpuVendor::Amd => 1,
            CpuVendor::Hygon => 2,
            CpuVendor::Zhaoxin => 3,
            CpuVendor::Unknown => -1,
        };
    }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn cpudetect_get_vendor_string(buffer: *mut i8, len: usize) -> i32 {
    #[cfg(feature = "alloc")]
    {
        if buffer.is_null() || len == 0 {
            return -1;
        }
        let info = VendorInfo::detect();
        let s = info.vendor_string.as_bytes();
        let to_copy = core::cmp::min(len - 1, s.len());
        unsafe {
            ptr::copy_nonoverlapping(s.as_ptr(), buffer as *mut u8, to_copy);
            *buffer.add(to_copy) = 0;
        }
        to_copy as i32
    }
    #[cfg(not(feature = "alloc"))]
    {
        let _ = (buffer, len);
        -1
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cpudetect_get_brand_string(buffer: *mut i8, len: usize) -> i32 {
    #[cfg(feature = "alloc")]
    {
        if buffer.is_null() || len == 0 {
            return -1;
        }
        let info = VendorInfo::detect();
        let s = info.brand_string.as_bytes();
        let to_copy = core::cmp::min(len - 1, s.len());
        unsafe {
            ptr::copy_nonoverlapping(s.as_ptr(), buffer as *mut u8, to_copy);
            *buffer.add(to_copy) = 0;
        }
        to_copy as i32
    }
    #[cfg(not(feature = "alloc"))]
    {
        let _ = (buffer, len);
        -1
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cpudetect_detect_topology(out: *mut CpuTopologyFFI) -> i32 {
    if out.is_null() {
        return -1;
    }

    let info = CpuTopology::detect();
    unsafe {
        (*out).logical_processors = info.logical_processors;
        (*out).physical_cores = info.physical_cores;
        (*out).threads_per_core = info.threads_per_core;
        (*out).has_hyperthreading = info.has_hyperthreading;
        (*out).hybrid = info.hybrid;
    }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn cpudetect_detect_features(out: *mut CpuFeaturesFFI) -> i32 {
    if out.is_null() {
        return -1;
    }

    let info = CpuFeatures::detect();
    let bits = info.basic.bits();
    unsafe {
        (*out).basic_low = (bits & 0xFFFF_FFFF_FFFF_FFFF) as u64;
        (*out).basic_high = (bits >> 64) as u64;
    }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn cpudetect_has_feature(name: *const i8) -> bool {
    #[cfg(feature = "alloc")]
    {
        if name.is_null() {
            return false;
        }

        let c_str = unsafe { core::ffi::CStr::from_ptr(name) };
        let r_str = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };

        let features = CpuFeatures::detect();
        features.has_feature(r_str)
    }
    #[cfg(not(feature = "alloc"))]
    {
        let _ = name;
        false
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cpudetect_detect_address(out: *mut CpuAddressFFI) -> i32 {
    if out.is_null() {
        return -1;
    }

    let info = AddressInfo::detect();
    unsafe {
        (*out).physical_bits = info.physical_bits;
        (*out).virtual_bits = info.virtual_bits;
        (*out).guest_physical_bits = info.guest_physical_bits.unwrap_or(0);
    }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn cpudetect_detect_frequency(out: *mut CpuFrequencyFFI) -> i32 {
    if out.is_null() {
        return -1;
    }

    let info = FrequencyInfo::detect();
    unsafe {
        (*out).base_mhz = info.base_mhz.unwrap_or(0);
        (*out).max_mhz = info.max_mhz.unwrap_or(0);
        (*out).bus_mhz = info.bus_mhz.unwrap_or(0);
        (*out).tsc_mhz = info.tsc_mhz.unwrap_or(0);
    }
    0
}
