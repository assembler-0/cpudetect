//! Raw CPUID instruction interface
//! 
//! Provides safe wrappers around the x86_64 CPUID instruction.
//! This module does one thing: execute CPUID and return results.

use std::arch::x86_64::__cpuid_count;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuidResult {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

impl CpuidResult {
    pub fn is_bit_set(&self, register: Register, bit: u32) -> bool {
        let value = match register {
            Register::Eax => self.eax,
            Register::Ebx => self.ebx,
            Register::Ecx => self.ecx,
            Register::Edx => self.edx,
        };
        (value & (1 << bit)) != 0
    }

    pub fn extract_bits(&self, register: Register, start: u32, end: u32) -> u32 {
        let value = match register {
            Register::Eax => self.eax,
            Register::Ebx => self.ebx,
            Register::Ecx => self.ecx,
            Register::Edx => self.edx,
        };
        let mask = (1u32 << (end - start + 1)) - 1;
        (value >> start) & mask
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    Eax,
    Ebx,
    Ecx,
    Edx,
}

pub fn cpuid(leaf: u32, subleaf: u32) -> CpuidResult {
    unsafe {
        let result = __cpuid_count(leaf, subleaf);
        CpuidResult {
            eax: result.eax,
            ebx: result.ebx,
            ecx: result.ecx,
            edx: result.edx,
        }
    }
}

pub fn max_cpuid_leaf() -> u32 {
    cpuid(0, 0).eax
}

pub fn max_extended_leaf() -> u32 {
    cpuid(0x8000_0000, 0).eax
}

pub fn is_leaf_supported(leaf: u32) -> bool {
    if leaf < 0x8000_0000 {
        leaf <= max_cpuid_leaf()
    } else {
        leaf <= max_extended_leaf()
    }
}
