//! CPU Topology Detection
//! 
//! Detects CPU core count, threading, and topology information.

use crate::cpuid::{cpuid, is_leaf_supported};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreType {
    Performance,
    Efficient,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct CpuTopology {
    pub logical_processors: u32,
    pub physical_cores: u32,
    pub threads_per_core: u32,
    pub has_hyperthreading: bool,
    pub hybrid: bool,
}

impl CpuTopology {
    pub fn detect() -> Self {
        let mut logical_processors = 1;
        let mut physical_cores = 1;
        let mut has_hyperthreading = false;
        let mut hybrid = false;

        if is_leaf_supported(1) {
            let result = cpuid(1, 0);
            has_hyperthreading = (result.edx & (1 << 28)) != 0;
            logical_processors = ((result.ebx >> 16) & 0xFF) as u32;
        }

        // Try leaf 0xB for x2APIC topology
        if is_leaf_supported(0xB) {
            physical_cores = detect_cores_leaf_b();
        } else if is_leaf_supported(4) {
            // Fallback to leaf 4
            let result = cpuid(4, 0);
            physical_cores = ((result.eax >> 26) & 0x3F) as u32 + 1;
        }

        // Check for hybrid architecture (Intel 12th gen+)
        if is_leaf_supported(7) {
            let result = cpuid(7, 0);
            hybrid = (result.edx & (1 << 15)) != 0;
        }

        let threads_per_core = if physical_cores > 0 {
            logical_processors / physical_cores
        } else {
            1
        };

        Self {
            logical_processors,
            physical_cores,
            threads_per_core,
            has_hyperthreading,
            hybrid,
        }
    }
}

impl fmt::Display for CpuTopology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CPU Topology:")?;
        writeln!(f, "  Logical Processors: {}", self.logical_processors)?;
        writeln!(f, "  Physical Cores: {}", self.physical_cores)?;
        writeln!(f, "  Threads per Core: {}", self.threads_per_core)?;
        writeln!(f, "  Hyper-Threading: {}", if self.has_hyperthreading { "Yes" } else { "No" })?;
        write!(f, "  Hybrid Architecture: {}", if self.hybrid { "Yes" } else { "No" })
    }
}

fn detect_cores_leaf_b() -> u32 {
    let mut cores = 1;
    
    for subleaf in 0..10 {
        let result = cpuid(0xB, subleaf);
        if result.ecx == 0 {
            break;
        }
        
        let level_type = (result.ecx >> 8) & 0xFF;
        if level_type == 2 {
            cores = result.ebx & 0xFFFF;
            break;
        }
    }
    
    cores
}
