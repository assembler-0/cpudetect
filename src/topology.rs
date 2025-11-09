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
        let mut threads_per_core = 1;
        let mut has_hyperthreading = false;
        let mut hybrid = false;

        // Get Hyper-Threading status from leaf 1
        if is_leaf_supported(1) {
            let result = cpuid(1, 0);
            has_hyperthreading = (result.edx & (1 << 28)) != 0;
        }

        // Prioritize leaf 0xB for topology information
        if is_leaf_supported(0xB) {
            threads_per_core = detect_threads_per_core_leaf_b();
            logical_processors = detect_logical_processors_leaf_b();
            if logical_processors > 0 && threads_per_core > 0 {
                physical_cores = logical_processors / threads_per_core;
            }
        } else {
            // Fallback if leaf 0xB is not supported
            if is_leaf_supported(1) {
                let result = cpuid(1, 0);
                // For older CPUs, EBX[23:16] might give logical processors
                logical_processors = ((result.ebx >> 16) & 0xFF) as u32;
            }
            if is_leaf_supported(4) {
                let result = cpuid(4, 0);
                physical_cores = ((result.eax >> 26) & 0x3F) as u32 + 1;
            }

            // If logical_processors is still 1 (and hyperthreading is off), set it to physical_cores
            if logical_processors == 1 && !has_hyperthreading {
                logical_processors = physical_cores;
            }
            
            // Final check for threads_per_core in fallback
            if physical_cores > 0 {
                threads_per_core = logical_processors / physical_cores;
            } else {
                threads_per_core = 1;
            }
        }

        // Check for hybrid architecture (Intel 12th gen+)
        if is_leaf_supported(7) {
            let result = cpuid(7, 0);
            hybrid = (result.edx & (1 << 15)) != 0;
        }

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

fn detect_threads_per_core_leaf_b() -> u32 {
    for subleaf in 0..10 {
        let result = cpuid(0xB, subleaf);
        let level_type = (result.ecx >> 8) & 0xFF;
        if level_type == 1 { // SMT level
            return result.ebx & 0xFFFF;
        }
        if level_type == 0 {
            break;
        }
    }
    1
}

fn detect_logical_processors_leaf_b() -> u32 {
    for subleaf in 0..10 {
        let result = cpuid(0xB, subleaf);
        let level_type = (result.ecx >> 8) & 0xFF;
        if level_type == 2 { // Core level
            return result.ebx & 0xFFFF;
        }
        if level_type == 0 {
            break;
        }
    }
    1
}
