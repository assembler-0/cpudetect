//! CPU Cache Detection
//! 
//! Detects CPU cache hierarchy, sizes, and associativity.

use crate::cpuid::{cpuid, is_leaf_supported};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheLevel {
    L1,
    L2,
    L3,
    L4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheType {
    Data,
    Instruction,
    Unified,
}

#[derive(Debug, Clone)]
pub struct CacheInfo {
    pub level: CacheLevel,
    pub cache_type: CacheType,
    pub size: u64,
    pub ways: u32,
    pub line_size: u32,
    pub sets: u32,
    pub shared_by: u32,
}

impl CacheInfo {
    pub fn detect_all() -> Vec<Self> {
        let mut caches = Vec::new();

        if is_leaf_supported(4) {
            detect_intel_caches(&mut caches);
        } else if is_leaf_supported(0x8000_0005) {
            detect_amd_caches(&mut caches);
        }

        caches
    }
}

impl fmt::Display for CacheInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level = match self.level {
            CacheLevel::L1 => "L1",
            CacheLevel::L2 => "L2",
            CacheLevel::L3 => "L3",
            CacheLevel::L4 => "L4",
        };

        let cache_type = match self.cache_type {
            CacheType::Data => "Data",
            CacheType::Instruction => "Instruction",
            CacheType::Unified => "Unified",
        };

        write!(
            f,
            "{} {} Cache: {} KB, {}-way, {}-byte lines, {} sets, shared by {} threads",
            level,
            cache_type,
            self.size / 1024,
            self.ways,
            self.line_size,
            self.sets,
            self.shared_by
        )
    }
}

fn detect_intel_caches(caches: &mut Vec<CacheInfo>) {
    for index in 0..32 {
        let result = cpuid(4, index);
        let cache_type_bits = result.eax & 0x1F;

        if cache_type_bits == 0 {
            break;
        }

        let cache_type = match cache_type_bits {
            1 => CacheType::Data,
            2 => CacheType::Instruction,
            3 => CacheType::Unified,
            _ => continue,
        };

        let level = match (result.eax >> 5) & 0x7 {
            1 => CacheLevel::L1,
            2 => CacheLevel::L2,
            3 => CacheLevel::L3,
            4 => CacheLevel::L4,
            _ => continue,
        };

        let ways = ((result.ebx >> 22) & 0x3FF) + 1;
        let partitions = ((result.ebx >> 12) & 0x3FF) + 1;
        let line_size = (result.ebx & 0xFFF) + 1;
        let sets = result.ecx + 1;
        let shared_by = ((result.eax >> 14) & 0xFFF) + 1;

        let size = (ways * partitions * line_size * sets) as u64;

        caches.push(CacheInfo {
            level,
            cache_type,
            size,
            ways,
            line_size,
            sets,
            shared_by,
        });
    }
}

fn detect_amd_caches(caches: &mut Vec<CacheInfo>) {
    if is_leaf_supported(0x8000_0005) {
        let result = cpuid(0x8000_0005, 0);
        
        // L1 Data Cache
        let l1d_size = ((result.ecx >> 24) & 0xFF) as u64 * 1024;
        let l1d_ways = ((result.ecx >> 16) & 0xFF) as u32;
        let l1d_line_size = (result.ecx & 0xFF) as u32;
        
        if l1d_size > 0 {
            caches.push(CacheInfo {
                level: CacheLevel::L1,
                cache_type: CacheType::Data,
                size: l1d_size,
                ways: l1d_ways,
                line_size: l1d_line_size,
                sets: (l1d_size / (l1d_ways as u64 * l1d_line_size as u64)) as u32,
                shared_by: 1,
            });
        }

        // L1 Instruction Cache
        let l1i_size = ((result.edx >> 24) & 0xFF) as u64 * 1024;
        let l1i_ways = ((result.edx >> 16) & 0xFF) as u32;
        let l1i_line_size = (result.edx & 0xFF) as u32;
        
        if l1i_size > 0 {
            caches.push(CacheInfo {
                level: CacheLevel::L1,
                cache_type: CacheType::Instruction,
                size: l1i_size,
                ways: l1i_ways,
                line_size: l1i_line_size,
                sets: (l1i_size / (l1i_ways as u64 * l1i_line_size as u64)) as u32,
                shared_by: 1,
            });
        }
    }

    if is_leaf_supported(0x8000_0006) {
        let result = cpuid(0x8000_0006, 0);
        
        // L2 Cache
        let l2_size = ((result.ecx >> 16) & 0xFFFF) as u64 * 1024;
        let l2_ways = ((result.ecx >> 12) & 0xF) as u32;
        let l2_line_size = (result.ecx & 0xFF) as u32;
        
        if l2_size > 0 {
            caches.push(CacheInfo {
                level: CacheLevel::L2,
                cache_type: CacheType::Unified,
                size: l2_size,
                ways: l2_ways,
                line_size: l2_line_size,
                sets: (l2_size / (l2_ways as u64 * l2_line_size as u64)) as u32,
                shared_by: 1,
            });
        }

        // L3 Cache
        let l3_size = ((result.edx >> 18) & 0x3FFF) as u64 * 512 * 1024;
        let l3_ways = ((result.edx >> 12) & 0xF) as u32;
        let l3_line_size = (result.edx & 0xFF) as u32;
        
        if l3_size > 0 {
            caches.push(CacheInfo {
                level: CacheLevel::L3,
                cache_type: CacheType::Unified,
                size: l3_size,
                ways: l3_ways,
                line_size: l3_line_size,
                sets: (l3_size / (l3_ways as u64 * l3_line_size as u64)) as u32,
                shared_by: 1,
            });
        }
    }
}
