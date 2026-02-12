//! CPU Vendor Detection
//!
//! Identifies CPU manufacturer and provides vendor-specific information.

use crate::cpuid::cpuid;
#[cfg(feature = "alloc")]
use crate::cpuid::CpuidResult;
use core::fmt;

#[cfg(feature = "alloc")]
use alloc::string::{String, ToString};
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuVendor {
    Intel,
    Amd,
    Hygon,
    Zhaoxin,
    Unknown,
}

impl CpuVendor {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Intel => "GenuineIntel",
            Self::Amd => "AuthenticAMD",
            Self::Hygon => "HygonGenuine",
            Self::Zhaoxin => "  Shanghai  ",
            Self::Unknown => "Unknown",
        }
    }
}

#[derive(Debug, Clone)]
pub struct VendorInfo {
    pub vendor: CpuVendor,
    #[cfg(feature = "alloc")]
    pub vendor_string: String,
    #[cfg(feature = "alloc")]
    pub brand_string: String,
    pub family: u32,
    pub model: u32,
    pub stepping: u32,
}

impl VendorInfo {
    pub fn detect() -> Self {
        let vendor_result = cpuid(0, 0);
        #[cfg(feature = "alloc")]
        let vendor_string = read_vendor_string(&vendor_result);
        
        #[cfg(feature = "alloc")]
        let vendor = match vendor_string.as_str() {
            "GenuineIntel" => CpuVendor::Intel,
            "AuthenticAMD" => CpuVendor::Amd,
            "HygonGenuine" => CpuVendor::Hygon,
            "  Shanghai  " => CpuVendor::Zhaoxin,
            _ => CpuVendor::Unknown,
        };
        
        #[cfg(not(feature = "alloc"))]
        let vendor = match vendor_result.ebx {
            0x756e6547 => CpuVendor::Intel, // "Genu"
            0x68747541 => CpuVendor::Amd,   // "Auth"
            _ => CpuVendor::Unknown,
        };

        let signature = cpuid(1, 0);
        let family = extract_family(signature.eax);
        let model = extract_model(signature.eax);
        let stepping = signature.eax & 0xF;

        #[cfg(feature = "alloc")]
        let brand_string = read_brand_string();

        Self {
            vendor,
            #[cfg(feature = "alloc")]
            vendor_string,
            #[cfg(feature = "alloc")]
            brand_string,
            family,
            model,
            stepping,
        }
    }
}

impl fmt::Display for VendorInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "alloc")]
        writeln!(
            f,
            "Vendor: {} ({})",
            self.vendor_string,
            self.vendor.as_str()
        )?;
        #[cfg(not(feature = "alloc"))]
        writeln!(f, "Vendor: {}", self.vendor.as_str())?;

        #[cfg(feature = "alloc")]
        writeln!(f, "Brand: {}", self.brand_string)?;
        
        write!(
            f,
            "Family: 0x{:X}, Model: 0x{:X}, Stepping: {}",
            self.family, self.model, self.stepping
        )
    }
}

#[cfg(feature = "alloc")]
fn read_vendor_string(result: &CpuidResult) -> String {
    let mut bytes = Vec::with_capacity(12);
    bytes.extend_from_slice(&result.ebx.to_le_bytes());
    bytes.extend_from_slice(&result.edx.to_le_bytes());
    bytes.extend_from_slice(&result.ecx.to_le_bytes());
    String::from_utf8_lossy(&bytes).to_string()
}

#[cfg(feature = "alloc")]
fn read_brand_string() -> String {
    let mut brand = Vec::with_capacity(48);

    for leaf in 0x8000_0002..=0x8000_0004 {
        let result = cpuid(leaf, 0);
        brand.extend_from_slice(&result.eax.to_le_bytes());
        brand.extend_from_slice(&result.ebx.to_le_bytes());
        brand.extend_from_slice(&result.ecx.to_le_bytes());
        brand.extend_from_slice(&result.edx.to_le_bytes());
    }

    String::from_utf8_lossy(&brand)
        .trim_end_matches('\0')
        .trim()
        .to_string()
}

fn extract_family(eax: u32) -> u32 {
    let base_family = (eax >> 8) & 0xF;
    let extended_family = (eax >> 20) & 0xFF;

    if base_family == 0xF {
        base_family + extended_family
    } else {
        base_family
    }
}

fn extract_model(eax: u32) -> u32 {
    let base_model = (eax >> 4) & 0xF;
    let extended_model = (eax >> 16) & 0xF;
    let family = (eax >> 8) & 0xF;

    if family == 0x6 || family == 0xF {
        (extended_model << 4) | base_model
    } else {
        base_model
    }
}
