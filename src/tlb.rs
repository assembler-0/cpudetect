//! TLB (Translation Lookaside Buffer) Detection
//!
//! Detects TLB sizes and configurations.

use crate::cpuid::{cpuid, is_leaf_supported};

#[derive(Debug, Clone)]
pub struct TlbEntry {
    pub page_size: String,
    pub entries: u32,
    pub associativity: String,
    pub tlb_type: String,
}

#[derive(Debug, Clone)]
pub struct TlbInfo {
    pub entries: Vec<TlbEntry>,
}

impl TlbInfo {
    pub fn detect() -> Self {
        let mut entries = Vec::new();

        if is_leaf_supported(0x8000_0005) {
            detect_amd_l1_tlb(&mut entries);
        }

        if is_leaf_supported(0x8000_0006) {
            detect_amd_l2_tlb(&mut entries);
        }

        if is_leaf_supported(0x18) {
            detect_intel_tlb(&mut entries);
        }

        Self { entries }
    }
}

fn detect_amd_l1_tlb(entries: &mut Vec<TlbEntry>) {
    let result = cpuid(0x8000_0005, 0);

    let l1_dtlb_2m4m = (result.eax >> 16) & 0xFFFF;
    if l1_dtlb_2m4m != 0 {
        entries.push(TlbEntry {
            page_size: "2M/4M".to_string(),
            entries: (l1_dtlb_2m4m & 0xFF) as u32,
            associativity: decode_assoc((l1_dtlb_2m4m >> 8) & 0xFF),
            tlb_type: "L1 Data".to_string(),
        });
    }

    let l1_itlb_2m4m = result.eax & 0xFFFF;
    if l1_itlb_2m4m != 0 {
        entries.push(TlbEntry {
            page_size: "2M/4M".to_string(),
            entries: (l1_itlb_2m4m & 0xFF) as u32,
            associativity: decode_assoc((l1_itlb_2m4m >> 8) & 0xFF),
            tlb_type: "L1 Instruction".to_string(),
        });
    }

    let l1_dtlb_4k = (result.ebx >> 16) & 0xFFFF;
    if l1_dtlb_4k != 0 {
        entries.push(TlbEntry {
            page_size: "4K".to_string(),
            entries: (l1_dtlb_4k & 0xFF) as u32,
            associativity: decode_assoc((l1_dtlb_4k >> 8) & 0xFF),
            tlb_type: "L1 Data".to_string(),
        });
    }

    let l1_itlb_4k = result.ebx & 0xFFFF;
    if l1_itlb_4k != 0 {
        entries.push(TlbEntry {
            page_size: "4K".to_string(),
            entries: (l1_itlb_4k & 0xFF) as u32,
            associativity: decode_assoc((l1_itlb_4k >> 8) & 0xFF),
            tlb_type: "L1 Instruction".to_string(),
        });
    }
}

fn detect_amd_l2_tlb(entries: &mut Vec<TlbEntry>) {
    let result = cpuid(0x8000_0006, 0);

    let l2_dtlb_2m4m = (result.eax >> 16) & 0xFFFF;
    if l2_dtlb_2m4m != 0 {
        entries.push(TlbEntry {
            page_size: "2M/4M".to_string(),
            entries: (l2_dtlb_2m4m & 0xFFF) as u32,
            associativity: decode_assoc_l2((l2_dtlb_2m4m >> 12) & 0xF),
            tlb_type: "L2 Data".to_string(),
        });
    }

    let l2_itlb_2m4m = result.eax & 0xFFFF;
    if l2_itlb_2m4m != 0 {
        entries.push(TlbEntry {
            page_size: "2M/4M".to_string(),
            entries: (l2_itlb_2m4m & 0xFFF) as u32,
            associativity: decode_assoc_l2((l2_itlb_2m4m >> 12) & 0xF),
            tlb_type: "L2 Instruction".to_string(),
        });
    }

    let l2_dtlb_4k = (result.ebx >> 16) & 0xFFFF;
    if l2_dtlb_4k != 0 {
        entries.push(TlbEntry {
            page_size: "4K".to_string(),
            entries: (l2_dtlb_4k & 0xFFF) as u32,
            associativity: decode_assoc_l2((l2_dtlb_4k >> 12) & 0xF),
            tlb_type: "L2 Data".to_string(),
        });
    }

    let l2_itlb_4k = result.ebx & 0xFFFF;
    if l2_itlb_4k != 0 {
        entries.push(TlbEntry {
            page_size: "4K".to_string(),
            entries: (l2_itlb_4k & 0xFFF) as u32,
            associativity: decode_assoc_l2((l2_itlb_4k >> 12) & 0xF),
            tlb_type: "L2 Instruction".to_string(),
        });
    }
}

fn detect_intel_tlb(entries: &mut Vec<TlbEntry>) {
    for subleaf in 0..10 {
        let result = cpuid(0x18, subleaf);
        if result.eax == 0 {
            break;
        }

        let tlb_type = match result.edx & 0x1F {
            0 => continue,
            1 => "Data",
            2 => "Instruction",
            3 => "Unified",
            _ => continue,
        };

        let level = ((result.edx >> 5) & 0x7) as u32;
        let page_size = match (result.ebx >> 0) & 0x3 {
            0 => "4K",
            1 => "2M",
            2 => "4M",
            3 => "1G",
            _ => "Unknown",
        };

        let ways = ((result.ebx >> 16) & 0xFFFF) as u32;
        let sets = result.ecx;

        entries.push(TlbEntry {
            page_size: page_size.to_string(),
            entries: ways * sets,
            associativity: if ways == 0xFFFF {
                "Fully".to_string()
            } else {
                format!("{}-way", ways)
            },
            tlb_type: format!("L{} {}", level, tlb_type),
        });
    }
}

fn decode_assoc(val: u32) -> String {
    match val {
        0x00 => "Reserved".to_string(),
        0x01 => "1-way".to_string(),
        0x02 => "2-way".to_string(),
        0xFF => "Fully".to_string(),
        _ => format!("{}-way", val),
    }
}

fn decode_assoc_l2(val: u32) -> String {
    match val {
        0x0 => "Disabled".to_string(),
        0x1 => "1-way".to_string(),
        0x2 => "2-way".to_string(),
        0x4 => "4-way".to_string(),
        0x6 => "8-way".to_string(),
        0x8 => "16-way".to_string(),
        0xA => "32-way".to_string(),
        0xB => "48-way".to_string(),
        0xC => "64-way".to_string(),
        0xD => "96-way".to_string(),
        0xE => "128-way".to_string(),
        0xF => "Fully".to_string(),
        _ => format!("{}-way", val),
    }
}
