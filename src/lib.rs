//! CPU Feature Detection Library for x86_64
//!
//! A clean, modular library for detecting CPU features and capabilities.
//! Follows Unix philosophy: each module does one thing well.

#![cfg(target_arch = "x86_64")]

pub mod cpuid;
pub mod features;
pub mod topology;
pub mod vendor;
pub mod cache;
pub mod thermal;

pub use features::{CpuFeatures, FeatureSet, Feature, FeatureCategory};
pub use vendor::{CpuVendor, VendorInfo};
pub use topology::{CpuTopology, CoreType};
pub use cache::{CacheInfo, CacheLevel, CacheType};

use std::fmt;

#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub vendor: VendorInfo,
    pub features: CpuFeatures,
    pub topology: CpuTopology,
    pub cache: Vec<CacheInfo>,
}

impl CpuInfo {
    pub fn detect() -> Self {
        Self {
            vendor: VendorInfo::detect(),
            features: CpuFeatures::detect(),
            topology: CpuTopology::detect(),
            cache: CacheInfo::detect_all(),
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
        Ok(())
    }
}
