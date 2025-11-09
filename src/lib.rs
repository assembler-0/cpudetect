//! CPU Feature Detection Library for x86_64
//!
//! A clean, modular library for detecting CPU features and capabilities.
//! Follows Unix philosophy: each module does one thing well.

#![cfg(target_arch = "x86_64")]

pub mod address;
pub mod cache;
pub mod cpuid;
pub mod features;
pub mod frequency;
pub mod msr;
pub mod platform;
pub mod power;
pub mod thermal;
pub mod tlb;
pub mod topology;
pub mod vendor;

pub use address::AddressInfo;
pub use cache::{CacheInfo, CacheLevel, CacheType};
pub use features::{CpuFeatures, Feature, FeatureCategory, FeatureSet};
pub use frequency::FrequencyInfo;
pub use msr::MsrInfo;
pub use platform::PlatformInfo;
pub use power::PowerInfo;
pub use tlb::{TlbEntry, TlbInfo};
pub use topology::{CoreType, CpuTopology};
pub use vendor::{CpuVendor, VendorInfo};

use std::fmt;

#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub vendor: VendorInfo,
    pub features: CpuFeatures,
    pub topology: CpuTopology,
    pub cache: Vec<CacheInfo>,
    pub power: PowerInfo,
    pub frequency: FrequencyInfo,
    pub address: AddressInfo,
    pub tlb: TlbInfo,
    pub platform: PlatformInfo,
    pub msr: MsrInfo,
}

impl CpuInfo {
    pub fn detect() -> Self {
        Self {
            vendor: VendorInfo::detect(),
            features: CpuFeatures::detect(),
            topology: CpuTopology::detect(),
            cache: CacheInfo::detect_all(),
            power: PowerInfo::detect(),
            frequency: FrequencyInfo::detect(),
            address: AddressInfo::detect(),
            tlb: TlbInfo::detect(),
            platform: PlatformInfo::detect(),
            msr: MsrInfo::detect(),
        }
    }
}

impl fmt::Display for CpuInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.vendor)?;
        writeln!(f, "\n{}", self.topology)?;
        writeln!(f, "\n{}", self.features)?;
        writeln!(f, "\nCache Information:")?;
        for cache in &self.cache {
            writeln!(f, "  {}", cache)?;
        }
        writeln!(
            f,
            "\nFrequency: Base={:?} MHz, Max={:?} MHz",
            self.frequency.base_mhz, self.frequency.max_mhz
        )?;
        writeln!(
            f,
            "\nAddress Sizes: Physical={} bits, Virtual={} bits",
            self.address.physical_bits, self.address.virtual_bits
        )?;
        Ok(())
    }
}
