//! CPU Feature Detection
//!
//! Comprehensive detection of x86_64 CPU features and instruction set extensions.

use crate::cpuid::{cpuid, is_leaf_supported};
use bitflags::bitflags;
use std::fmt;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct FeatureSet: u128 {
        // Basic Features (Leaf 1, EDX)
        const FPU       = 1 << 0;
        const VME       = 1 << 1;
        const DE        = 1 << 2;
        const PSE       = 1 << 3;
        const TSC       = 1 << 4;
        const MSR       = 1 << 5;
        const PAE       = 1 << 6;
        const MCE       = 1 << 7;
        const CX8       = 1 << 8;
        const APIC      = 1 << 9;
        const SEP       = 1 << 11;
        const MTRR      = 1 << 12;
        const PGE       = 1 << 13;
        const MCA       = 1 << 14;
        const CMOV      = 1 << 15;
        const PAT       = 1 << 16;
        const PSE36     = 1 << 17;
        const PSN       = 1 << 18;
        const CLFSH     = 1 << 19;
        const DS        = 1 << 21;
        const ACPI      = 1 << 22;
        const MMX       = 1 << 23;
        const FXSR      = 1 << 24;
        const SSE       = 1 << 25;
        const SSE2      = 1 << 26;
        const SS        = 1 << 27;
        const HTT       = 1 << 28;
        const TM        = 1 << 29;
        const PBE       = 1 << 31;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureCategory {
    Simd,
    Security,
    Virtualization,
    Cryptography,
    Performance,
    Debug,
    Power,
    Memory,
    System,
}

#[derive(Debug, Clone)]
pub struct Feature {
    pub name: String,
    pub category: FeatureCategory,
    pub description: &'static str,
    pub supported: bool,
}

#[derive(Debug, Clone)]
pub struct CpuFeatures {
    pub basic: FeatureSet,
    pub all_features: Vec<Feature>,
}

impl CpuFeatures {
    pub fn detect() -> Self {
        let mut basic = FeatureSet::empty();
        let mut all_features = Vec::new();

        // Leaf 1: Basic features
        if is_leaf_supported(1) {
            let result = cpuid(1, 0);
            detect_leaf1_edx(result.edx, &mut basic);
            detect_leaf1_ecx(result.ecx, &mut all_features);
        }

        // Leaf 7: Structured extended features
        if is_leaf_supported(7) {
            detect_leaf7(&mut all_features);
        }

        // Leaf 7 subleaf 1
        if is_leaf_supported(7) {
            detect_leaf7_sub1(&mut all_features);
        }

        // Leaf 7 subleaf 2
        if is_leaf_supported(7) {
            detect_leaf7_sub2(&mut all_features);
        }

        // Extended leaves: Additional AMD/Intel features
        if is_leaf_supported(0x8000_0001) {
            detect_extended_features(&mut all_features);
        }

        // AMD Extended Features
        if is_leaf_supported(0x8000_0008) {
            detect_amd_extended(&mut all_features);
        }

        // Intel specific leaves
        detect_intel_specific(&mut all_features);

        Self {
            basic,
            all_features,
        }
    }

    pub fn has_feature(&self, name: &str) -> bool {
        self.all_features
            .iter()
            .any(|f| f.name == name && f.supported)
    }

    pub fn features_by_category(&self, category: FeatureCategory) -> Vec<&Feature> {
        self.all_features
            .iter()
            .filter(|f| f.category == category && f.supported)
            .collect()
    }

    pub fn all_supported(&self) -> Vec<&Feature> {
        self.all_features.iter().filter(|f| f.supported).collect()
    }
}

impl fmt::Display for CpuFeatures {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CPU Features:")?;
        writeln!(f, "  Basic: {:?}", self.basic)?;

        let categories = [
            FeatureCategory::Simd,
            FeatureCategory::Cryptography,
            FeatureCategory::Security,
            FeatureCategory::Virtualization,
            FeatureCategory::Performance,
            FeatureCategory::Memory,
        ];

        for cat in &categories {
            let features = self.features_by_category(*cat);
            if !features.is_empty() {
                writeln!(
                    f,
                    "  {:?}: {}",
                    cat,
                    features
                        .iter()
                        .map(|fe| fe.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                )?;
            }
        }

        Ok(())
    }
}

fn detect_leaf1_edx(edx: u32, features: &mut FeatureSet) {
    if edx & (1 << 0) != 0 {
        *features |= FeatureSet::FPU;
    }
    if edx & (1 << 1) != 0 {
        *features |= FeatureSet::VME;
    }
    if edx & (1 << 2) != 0 {
        *features |= FeatureSet::DE;
    }
    if edx & (1 << 3) != 0 {
        *features |= FeatureSet::PSE;
    }
    if edx & (1 << 4) != 0 {
        *features |= FeatureSet::TSC;
    }
    if edx & (1 << 5) != 0 {
        *features |= FeatureSet::MSR;
    }
    if edx & (1 << 6) != 0 {
        *features |= FeatureSet::PAE;
    }
    if edx & (1 << 7) != 0 {
        *features |= FeatureSet::MCE;
    }
    if edx & (1 << 8) != 0 {
        *features |= FeatureSet::CX8;
    }
    if edx & (1 << 9) != 0 {
        *features |= FeatureSet::APIC;
    }
    if edx & (1 << 11) != 0 {
        *features |= FeatureSet::SEP;
    }
    if edx & (1 << 12) != 0 {
        *features |= FeatureSet::MTRR;
    }
    if edx & (1 << 13) != 0 {
        *features |= FeatureSet::PGE;
    }
    if edx & (1 << 14) != 0 {
        *features |= FeatureSet::MCA;
    }
    if edx & (1 << 15) != 0 {
        *features |= FeatureSet::CMOV;
    }
    if edx & (1 << 16) != 0 {
        *features |= FeatureSet::PAT;
    }
    if edx & (1 << 17) != 0 {
        *features |= FeatureSet::PSE36;
    }
    if edx & (1 << 19) != 0 {
        *features |= FeatureSet::CLFSH;
    }
    if edx & (1 << 23) != 0 {
        *features |= FeatureSet::MMX;
    }
    if edx & (1 << 24) != 0 {
        *features |= FeatureSet::FXSR;
    }
    if edx & (1 << 25) != 0 {
        *features |= FeatureSet::SSE;
    }
    if edx & (1 << 26) != 0 {
        *features |= FeatureSet::SSE2;
    }
    if edx & (1 << 28) != 0 {
        *features |= FeatureSet::HTT;
    }
}

fn detect_leaf1_ecx(ecx: u32, features: &mut Vec<Feature>) {
    let feature_map = [
        (
            0,
            "SSE3",
            FeatureCategory::Simd,
            "Streaming SIMD Extensions 3",
        ),
        (
            1,
            "PCLMULQDQ",
            FeatureCategory::Cryptography,
            "Carry-less multiplication",
        ),
        (2, "DTES64", FeatureCategory::Debug, "64-bit debug store"),
        (
            3,
            "MONITOR",
            FeatureCategory::Power,
            "MONITOR/MWAIT instructions",
        ),
        (
            4,
            "DS-CPL",
            FeatureCategory::Debug,
            "CPL-qualified debug store",
        ),
        (
            5,
            "VMX",
            FeatureCategory::Virtualization,
            "Virtual Machine Extensions",
        ),
        (6, "SMX", FeatureCategory::Security, "Safer Mode Extensions"),
        (
            7,
            "EIST",
            FeatureCategory::Power,
            "Enhanced Intel SpeedStep",
        ),
        (8, "TM2", FeatureCategory::Power, "Thermal Monitor 2"),
        (9, "SSSE3", FeatureCategory::Simd, "Supplemental SSE3"),
        (10, "CNXT-ID", FeatureCategory::Debug, "L1 context ID"),
        (11, "SDBG", FeatureCategory::Debug, "Silicon Debug"),
        (12, "FMA", FeatureCategory::Simd, "Fused Multiply-Add"),
        (
            13,
            "CMPXCHG16B",
            FeatureCategory::System,
            "Compare and exchange 16 bytes",
        ),
        (14, "xTPR", FeatureCategory::System, "xTPR update control"),
        (
            15,
            "PDCM",
            FeatureCategory::Performance,
            "Performance/Debug capability MSR",
        ),
        (
            17,
            "PCID",
            FeatureCategory::Memory,
            "Process-context identifiers",
        ),
        (
            18,
            "DCA",
            FeatureCategory::Performance,
            "Direct Cache Access",
        ),
        (
            19,
            "SSE4.1",
            FeatureCategory::Simd,
            "Streaming SIMD Extensions 4.1",
        ),
        (
            20,
            "SSE4.2",
            FeatureCategory::Simd,
            "Streaming SIMD Extensions 4.2",
        ),
        (21, "x2APIC", FeatureCategory::System, "x2APIC support"),
        (22, "MOVBE", FeatureCategory::System, "MOVBE instruction"),
        (
            23,
            "POPCNT",
            FeatureCategory::Performance,
            "POPCNT instruction",
        ),
        (
            24,
            "TSC-Deadline",
            FeatureCategory::System,
            "TSC deadline timer",
        ),
        (
            25,
            "AES",
            FeatureCategory::Cryptography,
            "AES instruction set",
        ),
        (26, "XSAVE", FeatureCategory::System, "XSAVE/XRSTOR"),
        (27, "OSXSAVE", FeatureCategory::System, "OS-enabled XSAVE"),
        (
            28,
            "AVX",
            FeatureCategory::Simd,
            "Advanced Vector Extensions",
        ),
        (
            29,
            "F16C",
            FeatureCategory::Simd,
            "16-bit floating-point conversion",
        ),
        (
            30,
            "RDRAND",
            FeatureCategory::Security,
            "Hardware random number generator",
        ),
        (
            31,
            "HYPERVISOR",
            FeatureCategory::Virtualization,
            "Running under hypervisor",
        ),
    ];

    for (bit, name, category, desc) in feature_map.iter() {
        features.push(Feature {
            name: name.to_string(),
            category: *category,
            description: desc,
            supported: (ecx & (1 << bit)) != 0,
        });
    }
}

fn detect_leaf7(features: &mut Vec<Feature>) {
    let result = cpuid(7, 0);

    // EBX features
    let ebx_features = [
        (
            0,
            "FSGSBASE",
            FeatureCategory::System,
            "FS/GS base access instructions",
        ),
        (1, "TSC_ADJUST", FeatureCategory::System, "TSC adjust MSR"),
        (
            2,
            "SGX",
            FeatureCategory::Security,
            "Software Guard Extensions",
        ),
        (
            3,
            "BMI1",
            FeatureCategory::Performance,
            "Bit Manipulation Instruction Set 1",
        ),
        (
            4,
            "HLE",
            FeatureCategory::Performance,
            "Hardware Lock Elision",
        ),
        (
            5,
            "AVX2",
            FeatureCategory::Simd,
            "Advanced Vector Extensions 2",
        ),
        (
            6,
            "FDP_EXCPTN_ONLY",
            FeatureCategory::Debug,
            "FPU data pointer exception only",
        ),
        (
            7,
            "SMEP",
            FeatureCategory::Security,
            "Supervisor Mode Execution Prevention",
        ),
        (
            8,
            "BMI2",
            FeatureCategory::Performance,
            "Bit Manipulation Instruction Set 2",
        ),
        (
            9,
            "ERMS",
            FeatureCategory::Performance,
            "Enhanced REP MOVSB/STOSB",
        ),
        (
            10,
            "INVPCID",
            FeatureCategory::Memory,
            "INVPCID instruction",
        ),
        (
            11,
            "RTM",
            FeatureCategory::Performance,
            "Restricted Transactional Memory",
        ),
        (
            12,
            "PQM",
            FeatureCategory::Performance,
            "Platform QoS Monitoring",
        ),
        (
            13,
            "FPU_CS_DS_DEPRECATED",
            FeatureCategory::System,
            "FPU CS/DS deprecated",
        ),
        (
            14,
            "MPX",
            FeatureCategory::Security,
            "Memory Protection Extensions",
        ),
        (
            15,
            "PQE",
            FeatureCategory::Performance,
            "Platform QoS Enforcement",
        ),
        (16, "AVX512F", FeatureCategory::Simd, "AVX-512 Foundation"),
        (
            17,
            "AVX512DQ",
            FeatureCategory::Simd,
            "AVX-512 Doubleword and Quadword",
        ),
        (
            18,
            "RDSEED",
            FeatureCategory::Security,
            "RDSEED instruction",
        ),
        (
            19,
            "ADX",
            FeatureCategory::Performance,
            "Multi-precision add-carry",
        ),
        (
            20,
            "SMAP",
            FeatureCategory::Security,
            "Supervisor Mode Access Prevention",
        ),
        (
            21,
            "AVX512_IFMA",
            FeatureCategory::Simd,
            "AVX-512 Integer FMA",
        ),
        (
            23,
            "CLFLUSHOPT",
            FeatureCategory::Performance,
            "CLFLUSHOPT instruction",
        ),
        (
            24,
            "CLWB",
            FeatureCategory::Performance,
            "Cache line writeback",
        ),
        (
            25,
            "INTEL_PT",
            FeatureCategory::Debug,
            "Intel Processor Trace",
        ),
        (26, "AVX512PF", FeatureCategory::Simd, "AVX-512 Prefetch"),
        (
            27,
            "AVX512ER",
            FeatureCategory::Simd,
            "AVX-512 Exponential and Reciprocal",
        ),
        (
            28,
            "AVX512CD",
            FeatureCategory::Simd,
            "AVX-512 Conflict Detection",
        ),
        (
            29,
            "SHA",
            FeatureCategory::Cryptography,
            "SHA-1/SHA-256 instructions",
        ),
        (
            30,
            "AVX512BW",
            FeatureCategory::Simd,
            "AVX-512 Byte and Word",
        ),
        (
            31,
            "AVX512VL",
            FeatureCategory::Simd,
            "AVX-512 Vector Length Extensions",
        ),
    ];

    for (bit, name, category, desc) in ebx_features.iter() {
        features.push(Feature {
            name: name.to_string(),
            category: *category,
            description: desc,
            supported: (result.ebx & (1 << bit)) != 0,
        });
    }

    // ECX features
    let ecx_features = [
        (
            0,
            "PREFETCHWT1",
            FeatureCategory::Performance,
            "PREFETCHWT1 instruction",
        ),
        (
            1,
            "AVX512_VBMI",
            FeatureCategory::Simd,
            "AVX-512 Vector Bit Manipulation",
        ),
        (
            2,
            "UMIP",
            FeatureCategory::Security,
            "User-Mode Instruction Prevention",
        ),
        (
            3,
            "PKU",
            FeatureCategory::Security,
            "Protection Keys for User-mode pages",
        ),
        (4, "OSPKE", FeatureCategory::Security, "OS has enabled PKU"),
        (
            5,
            "WAITPKG",
            FeatureCategory::Power,
            "TPAUSE, UMONITOR, UMWAIT",
        ),
        (
            6,
            "AVX512_VBMI2",
            FeatureCategory::Simd,
            "AVX-512 Vector Bit Manipulation 2",
        ),
        (
            7,
            "CET_SS",
            FeatureCategory::Security,
            "Control-flow Enforcement Shadow Stack",
        ),
        (
            8,
            "GFNI",
            FeatureCategory::Cryptography,
            "Galois Field instructions",
        ),
        (9, "VAES", FeatureCategory::Cryptography, "Vector AES"),
        (
            10,
            "VPCLMULQDQ",
            FeatureCategory::Cryptography,
            "Vector PCLMULQDQ",
        ),
        (
            11,
            "AVX512_VNNI",
            FeatureCategory::Simd,
            "AVX-512 Vector Neural Network Instructions",
        ),
        (
            12,
            "AVX512_BITALG",
            FeatureCategory::Simd,
            "AVX-512 Bit Algorithms",
        ),
        (
            13,
            "TME_EN",
            FeatureCategory::Security,
            "Total Memory Encryption",
        ),
        (
            14,
            "AVX512_VPOPCNTDQ",
            FeatureCategory::Simd,
            "AVX-512 Vector Population Count",
        ),
        (
            16,
            "LA57",
            FeatureCategory::Memory,
            "5-level paging support",
        ),
        (22, "RDPID", FeatureCategory::System, "Read Processor ID"),
        (23, "KL", FeatureCategory::Security, "Key Locker"),
        (
            25,
            "CLDEMOTE",
            FeatureCategory::Performance,
            "Cache line demote",
        ),
        (
            27,
            "MOVDIRI",
            FeatureCategory::Performance,
            "MOVDIRI instruction",
        ),
        (
            28,
            "MOVDIR64B",
            FeatureCategory::Performance,
            "MOVDIR64B instruction",
        ),
        (
            29,
            "ENQCMD",
            FeatureCategory::Performance,
            "Enqueue Command",
        ),
        (
            30,
            "SGX_LC",
            FeatureCategory::Security,
            "SGX Launch Configuration",
        ),
        (
            31,
            "PKS",
            FeatureCategory::Security,
            "Protection Keys for Supervisor-mode",
        ),
    ];

    for (bit, name, category, desc) in ecx_features.iter() {
        features.push(Feature {
            name: name.to_string(),
            category: *category,
            description: desc,
            supported: (result.ecx & (1 << bit)) != 0,
        });
    }

    // EDX features
    let edx_features = [
        (
            2,
            "AVX512_4VNNIW",
            FeatureCategory::Simd,
            "AVX-512 4-register Neural Network",
        ),
        (
            3,
            "AVX512_4FMAPS",
            FeatureCategory::Simd,
            "AVX-512 4-register FMA Single Precision",
        ),
        (
            4,
            "FSRM",
            FeatureCategory::Performance,
            "Fast Short REP MOV",
        ),
        (5, "UINTR", FeatureCategory::System, "User Interrupts"),
        (
            8,
            "AVX512_VP2INTERSECT",
            FeatureCategory::Simd,
            "AVX-512 VP2INTERSECT",
        ),
        (
            9,
            "SRBDS_CTRL",
            FeatureCategory::Security,
            "SRBDS mitigation control",
        ),
        (
            10,
            "MD_CLEAR",
            FeatureCategory::Security,
            "VERW clears CPU buffers",
        ),
        (
            11,
            "RTM_ALWAYS_ABORT",
            FeatureCategory::Performance,
            "RTM always aborts",
        ),
        (
            13,
            "TSX_FORCE_ABORT",
            FeatureCategory::Security,
            "TSX force abort MSR",
        ),
        (
            14,
            "SERIALIZE",
            FeatureCategory::System,
            "SERIALIZE instruction",
        ),
        (15, "HYBRID", FeatureCategory::System, "Hybrid processor"),
        (
            16,
            "TSXLDTRK",
            FeatureCategory::Performance,
            "TSX suspend load tracking",
        ),
        (
            18,
            "PCONFIG",
            FeatureCategory::Security,
            "Platform configuration",
        ),
        (
            19,
            "ARCHITECTURAL_LBR",
            FeatureCategory::Debug,
            "Architectural LBR",
        ),
        (
            20,
            "CET_IBT",
            FeatureCategory::Security,
            "Control-flow Enforcement Indirect Branch",
        ),
        (
            22,
            "AMX_BF16",
            FeatureCategory::Simd,
            "AMX tile computation on bfloat16",
        ),
        (
            23,
            "AVX512_FP16",
            FeatureCategory::Simd,
            "AVX-512 16-bit floating-point",
        ),
        (24, "AMX_TILE", FeatureCategory::Simd, "AMX tile load/store"),
        (
            25,
            "AMX_INT8",
            FeatureCategory::Simd,
            "AMX tile computation on 8-bit integers",
        ),
        (
            26,
            "IBRS_IBPB",
            FeatureCategory::Security,
            "Speculation Control IBRS/IBPB",
        ),
        (
            27,
            "STIBP",
            FeatureCategory::Security,
            "Single Thread Indirect Branch Predictors",
        ),
        (
            28,
            "L1D_FLUSH",
            FeatureCategory::Security,
            "L1D cache flush",
        ),
        (
            29,
            "IA32_ARCH_CAPABILITIES",
            FeatureCategory::Security,
            "Arch capabilities MSR",
        ),
        (
            30,
            "IA32_CORE_CAPABILITIES",
            FeatureCategory::System,
            "Core capabilities MSR",
        ),
        (
            31,
            "SSBD",
            FeatureCategory::Security,
            "Speculative Store Bypass Disable",
        ),
    ];

    for (bit, name, category, desc) in edx_features.iter() {
        features.push(Feature {
            name: name.to_string(),
            category: *category,
            description: desc,
            supported: (result.edx & (1 << bit)) != 0,
        });
    }
}

fn detect_leaf7_sub1(features: &mut Vec<Feature>) {
    let result = cpuid(7, 1);

    let eax_features = [
        (
            3,
            "RAO_INT",
            FeatureCategory::Performance,
            "RAO-INT instructions",
        ),
        (
            4,
            "AVX_VNNI",
            FeatureCategory::Simd,
            "AVX VNNI instructions",
        ),
        (
            5,
            "AVX512_BF16",
            FeatureCategory::Simd,
            "AVX-512 BFLOAT16 instructions",
        ),
        (
            6,
            "LASS",
            FeatureCategory::Security,
            "Linear Address Space Separation",
        ),
        (
            7,
            "CMPCCXADD",
            FeatureCategory::Performance,
            "CMPccXADD instructions",
        ),
        (
            8,
            "ARCHPERFMONEXT",
            FeatureCategory::Performance,
            "Architectural PerfMon Extended",
        ),
        (
            10,
            "FZRM",
            FeatureCategory::Simd,
            "Fast zero-length REP MOVSB",
        ),
        (
            11,
            "FSRS",
            FeatureCategory::Performance,
            "Fast short REP STOSB",
        ),
        (
            12,
            "FSRC",
            FeatureCategory::Performance,
            "Fast short REP CMPSB/SCASB",
        ),
        (
            17,
            "FRED",
            FeatureCategory::System,
            "Flexible Return and Event Delivery",
        ),
        (18, "LKGS", FeatureCategory::System, "LKGS instruction"),
        (
            19,
            "WRMSRNS",
            FeatureCategory::System,
            "WRMSRNS instruction",
        ),
        (
            21,
            "AMX_FP16",
            FeatureCategory::Simd,
            "AMX FP16 instructions",
        ),
        (
            22,
            "HRESET",
            FeatureCategory::System,
            "History reset support",
        ),
        (
            23,
            "AVX_IFMA",
            FeatureCategory::Simd,
            "AVX IFMA instructions",
        ),
        (26, "LAM", FeatureCategory::Memory, "Linear Address Masking"),
        (
            27,
            "MSRLIST",
            FeatureCategory::System,
            "RDMSRLIST and WRMSRLIST",
        ),
    ];

    for (bit, name, category, desc) in eax_features.iter() {
        features.push(Feature {
            name: name.to_string(),
            category: *category,
            description: desc,
            supported: (result.eax & (1 << bit)) != 0,
        });
    }

    let ebx_features = [(
        0,
        "PPIN",
        FeatureCategory::System,
        "Protected Processor Inventory Number",
    )];

    for (bit, name, category, desc) in ebx_features.iter() {
        features.push(Feature {
            name: name.to_string(),
            category: *category,
            description: desc,
            supported: (result.ebx & (1 << bit)) != 0,
        });
    }

    let edx_features = [
        (
            4,
            "AVX_VNNI_INT8",
            FeatureCategory::Simd,
            "AVX VNNI INT8 instructions",
        ),
        (
            5,
            "AVX_NE_CONVERT",
            FeatureCategory::Simd,
            "AVX no-exception FP conversion",
        ),
        (
            8,
            "AMX_COMPLEX",
            FeatureCategory::Simd,
            "AMX complex number support",
        ),
        (
            10,
            "AVX_VNNI_INT16",
            FeatureCategory::Simd,
            "AVX VNNI INT16 instructions",
        ),
        (
            14,
            "PREFETCHITI",
            FeatureCategory::Performance,
            "PREFETCHIT0/1 instructions",
        ),
        (
            15,
            "USER_MSR",
            FeatureCategory::System,
            "User-mode MSR access",
        ),
    ];

    for (bit, name, category, desc) in edx_features.iter() {
        features.push(Feature {
            name: name.to_string(),
            category: *category,
            description: desc,
            supported: (result.edx & (1 << bit)) != 0,
        });
    }
}

fn detect_leaf7_sub2(features: &mut Vec<Feature>) {
    let result = cpuid(7, 2);

    let edx_features = [
        (
            0,
            "PSFD",
            FeatureCategory::Security,
            "Fast Store Forwarding Predictor Disable",
        ),
        (1, "IPRED_CTRL", FeatureCategory::Security, "IPRED control"),
        (2, "RRSBA_CTRL", FeatureCategory::Security, "RRSBA control"),
        (
            3,
            "DDPD_U",
            FeatureCategory::Security,
            "Data Dependent Prefetcher Disable",
        ),
        (
            4,
            "BHI_CTRL",
            FeatureCategory::Security,
            "Branch History Injection control",
        ),
        (5, "MCDT_NO", FeatureCategory::Security, "MCDT not needed"),
    ];

    for (bit, name, category, desc) in edx_features.iter() {
        features.push(Feature {
            name: name.to_string(),
            category: *category,
            description: desc,
            supported: (result.edx & (1 << bit)) != 0,
        });
    }
}

fn detect_extended_features(features: &mut Vec<Feature>) {
    let result = cpuid(0x8000_0001, 0);

    // EDX extended features
    let edx_features = [
        (
            11,
            "SYSCALL",
            FeatureCategory::System,
            "SYSCALL/SYSRET instructions",
        ),
        (19, "MP", FeatureCategory::System, "Multiprocessor capable"),
        (20, "NX", FeatureCategory::Security, "Execute Disable bit"),
        (22, "MMXEXT", FeatureCategory::Simd, "Extended MMX"),
        (
            25,
            "FXSR_OPT",
            FeatureCategory::Performance,
            "FXSAVE/FXRSTOR optimizations",
        ),
        (26, "PDPE1GB", FeatureCategory::Memory, "1GB pages support"),
        (27, "RDTSCP", FeatureCategory::System, "RDTSCP instruction"),
        (
            29,
            "LM",
            FeatureCategory::System,
            "Long Mode (x86-64/EM64T)",
        ),
        (30, "3DNOWEXT", FeatureCategory::Simd, "Extended 3DNow!"),
        (31, "3DNOW", FeatureCategory::Simd, "3DNow! instructions"),
    ];

    for (bit, name, category, desc) in edx_features.iter() {
        features.push(Feature {
            name: name.to_string(),
            category: *category,
            description: desc,
            supported: (result.edx & (1 << bit)) != 0,
        });
    }

    // ECX extended features
    let ecx_features = [
        (
            0,
            "LAHF_LM",
            FeatureCategory::System,
            "LAHF/SAHF in 64-bit mode",
        ),
        (
            1,
            "CMP_LEGACY",
            FeatureCategory::System,
            "Core multi-processing legacy mode",
        ),
        (
            2,
            "SVM",
            FeatureCategory::Virtualization,
            "Secure Virtual Machine (AMD-V)",
        ),
        (3, "EXTAPIC", FeatureCategory::System, "Extended APIC space"),
        (
            4,
            "CR8_LEGACY",
            FeatureCategory::System,
            "CR8 in 32-bit mode",
        ),
        (
            5,
            "ABM",
            FeatureCategory::Performance,
            "Advanced Bit Manipulation (LZCNT)",
        ),
        (6, "SSE4A", FeatureCategory::Simd, "SSE4a instructions"),
        (
            7,
            "MISALIGNSSE",
            FeatureCategory::Performance,
            "Misaligned SSE mode",
        ),
        (
            8,
            "3DNOWPREFETCH",
            FeatureCategory::Performance,
            "PREFETCH/PREFETCHW",
        ),
        (9, "OSVW", FeatureCategory::System, "OS Visible Workaround"),
        (
            10,
            "IBS",
            FeatureCategory::Debug,
            "Instruction Based Sampling",
        ),
        (11, "XOP", FeatureCategory::Simd, "Extended Operations"),
        (
            12,
            "SKINIT",
            FeatureCategory::Security,
            "SKINIT/STGI instructions",
        ),
        (13, "WDT", FeatureCategory::Debug, "Watchdog Timer"),
        (
            15,
            "LWP",
            FeatureCategory::Performance,
            "Lightweight Profiling",
        ),
        (
            16,
            "FMA4",
            FeatureCategory::Simd,
            "4-operand Fused Multiply-Add",
        ),
        (
            17,
            "TCE",
            FeatureCategory::Performance,
            "Translation Cache Extension",
        ),
        (19, "NODEID_MSR", FeatureCategory::System, "NodeID MSR"),
        (
            21,
            "TBM",
            FeatureCategory::Performance,
            "Trailing Bit Manipulation",
        ),
        (
            22,
            "TOPOEXT",
            FeatureCategory::System,
            "Topology Extensions",
        ),
        (
            23,
            "PERFCTR_CORE",
            FeatureCategory::Performance,
            "Core performance counter",
        ),
        (
            24,
            "PERFCTR_NB",
            FeatureCategory::Performance,
            "Northbridge performance counter",
        ),
        (
            26,
            "DBX",
            FeatureCategory::Debug,
            "Data breakpoint extension",
        ),
        (
            27,
            "PERFTSC",
            FeatureCategory::Performance,
            "Performance TSC",
        ),
        (
            28,
            "PCX_L2I",
            FeatureCategory::Performance,
            "L2I performance counter",
        ),
        (
            29,
            "MONITORX",
            FeatureCategory::Power,
            "MONITORX/MWAITX instructions",
        ),
        (
            30,
            "ADDR_MASK_EXT",
            FeatureCategory::System,
            "Address mask extension",
        ),
    ];

    for (bit, name, category, desc) in ecx_features.iter() {
        features.push(Feature {
            name: name.to_string(),
            category: *category,
            description: desc,
            supported: (result.ecx & (1 << bit)) != 0,
        });
    }
}

fn detect_amd_extended(features: &mut Vec<Feature>) {
    let result = cpuid(0x8000_0008, 0);

    let ebx_features = [
        (
            0,
            "CLZERO",
            FeatureCategory::Performance,
            "CLZERO instruction",
        ),
        (
            1,
            "IRPERF",
            FeatureCategory::Performance,
            "Instructions retired counter",
        ),
        (
            2,
            "XSAVEERPTR",
            FeatureCategory::System,
            "XSAVE error pointers",
        ),
        (
            4,
            "RDPRU",
            FeatureCategory::Performance,
            "RDPRU instruction",
        ),
        (
            6,
            "MBE",
            FeatureCategory::Security,
            "Memory Bandwidth Enforcement",
        ),
        (
            8,
            "MCOMMIT",
            FeatureCategory::Performance,
            "MCOMMIT instruction",
        ),
        (
            9,
            "WBNOINVD",
            FeatureCategory::Performance,
            "WBNOINVD instruction",
        ),
        (
            12,
            "IBPB",
            FeatureCategory::Security,
            "Indirect Branch Prediction Barrier",
        ),
        (
            13,
            "INT_WBINVD",
            FeatureCategory::System,
            "Interruptible WBINVD",
        ),
        (
            14,
            "IBRS",
            FeatureCategory::Security,
            "Indirect Branch Restricted Speculation",
        ),
        (
            15,
            "STIBP",
            FeatureCategory::Security,
            "Single Thread Indirect Branch Predictor",
        ),
        (
            16,
            "IBRS_ALWAYS_ON",
            FeatureCategory::Security,
            "IBRS always enabled",
        ),
        (
            17,
            "STIBP_ALWAYS_ON",
            FeatureCategory::Security,
            "STIBP always enabled",
        ),
        (
            18,
            "IBRS_PREFERRED",
            FeatureCategory::Security,
            "IBRS preferred",
        ),
        (
            19,
            "IBRS_SAME_MODE",
            FeatureCategory::Security,
            "IBRS same mode protection",
        ),
        (
            20,
            "NO_EFER_LMSLE",
            FeatureCategory::System,
            "No EFER.LMSLE support",
        ),
        (
            23,
            "PPIN",
            FeatureCategory::Security,
            "Protected Processor Inventory Number",
        ),
        (
            24,
            "SSBD",
            FeatureCategory::Security,
            "Speculative Store Bypass Disable",
        ),
        (
            25,
            "VIRT_SSBD",
            FeatureCategory::Security,
            "Virtualized SSBD",
        ),
        (
            26,
            "SSB_NO",
            FeatureCategory::Security,
            "Not vulnerable to SSB",
        ),
        (
            28,
            "PSFD",
            FeatureCategory::Security,
            "Predictive Store Forward Disable",
        ),
    ];

    for (bit, name, category, desc) in ebx_features.iter() {
        features.push(Feature {
            name: name.to_string(),
            category: *category,
            description: desc,
            supported: (result.ebx & (1 << bit)) != 0,
        });
    }

    let ecx_features = [
        (
            0,
            "PERFCTR_CORE",
            FeatureCategory::Performance,
            "Core performance counters",
        ),
        (
            1,
            "PERFCTR_NB",
            FeatureCategory::Performance,
            "NB performance counters",
        ),
    ];

    for (bit, name, category, desc) in ecx_features.iter() {
        features.push(Feature {
            name: name.to_string(),
            category: *category,
            description: desc,
            supported: (result.ecx & (1 << bit)) != 0,
        });
    }
}

fn detect_intel_specific(features: &mut Vec<Feature>) {
    // Intel leaf 0xD - Extended state enumeration
    if is_leaf_supported(0xD) {
        let result = cpuid(0xD, 1);

        let eax_features = [
            (
                0,
                "XSAVEOPT",
                FeatureCategory::Performance,
                "XSAVEOPT instruction",
            ),
            (
                1,
                "XSAVEC",
                FeatureCategory::Performance,
                "XSAVEC instruction",
            ),
            (
                2,
                "XGETBV_ECX1",
                FeatureCategory::System,
                "XGETBV with ECX=1",
            ),
            (
                3,
                "XSAVES",
                FeatureCategory::System,
                "XSAVES/XRSTORS instructions",
            ),
            (
                4,
                "XFD",
                FeatureCategory::System,
                "Extended Feature Disable",
            ),
        ];

        for (bit, name, category, desc) in eax_features.iter() {
            features.push(Feature {
                name: name.to_string(),
                category: *category,
                description: desc,
                supported: (result.eax & (1 << bit)) != 0,
            });
        }
    }

    // Intel leaf 0x14 - Processor Trace
    if is_leaf_supported(0x14) {
        let result = cpuid(0x14, 0);
        let pt_features = [
            (0, "PT_LIP", "Processor Trace LIP support"),
            (1, "PT_MTC", "Processor Trace MTC support"),
            (2, "PT_PTWRITE", "Processor Trace PTWRITE support"),
            (3, "PT_POWER_EVENT", "Processor Trace Power Event support"),
        ];

        for (bit, name, desc) in pt_features.iter() {
            features.push(Feature {
                name: name.to_string(),
                category: FeatureCategory::Debug,
                description: desc,
                supported: (result.ebx & (1 << bit)) != 0,
            });
        }
    }

    // Intel leaf 0x1F - V2 Extended Topology
    if is_leaf_supported(0x1F) {
        features.push(Feature {
            name: "TOPOLOGY_V2".to_string(),
            category: FeatureCategory::System,
            description: "V2 Extended Topology Enumeration",
            supported: true,
        });
    }

    // Intel leaf 0x1A - Hybrid Information
    if is_leaf_supported(0x1A) {
        features.push(Feature {
            name: "HYBRID_INFO".to_string(),
            category: FeatureCategory::System,
            description: "Hybrid Core Information",
            supported: true,
        });
    }

    // Intel leaf 0x1B - PCONFIG
    if is_leaf_supported(0x1B) {
        features.push(Feature {
            name: "PCONFIG_ENUM".to_string(),
            category: FeatureCategory::Security,
            description: "PCONFIG Enumeration",
            supported: true,
        });
    }

    // Intel leaf 0x1C - Last Branch Records
    if is_leaf_supported(0x1C) {
        features.push(Feature {
            name: "LBR_INFO".to_string(),
            category: FeatureCategory::Debug,
            description: "Last Branch Record Information",
            supported: true,
        });
    }

    // Intel leaf 0x1D - Tile Information
    if is_leaf_supported(0x1D) {
        features.push(Feature {
            name: "TILE_INFO".to_string(),
            category: FeatureCategory::Simd,
            description: "AMX Tile Information",
            supported: true,
        });
    }

    // Intel leaf 0x1E - TMUL Information
    if is_leaf_supported(0x1E) {
        features.push(Feature {
            name: "TMUL_INFO".to_string(),
            category: FeatureCategory::Simd,
            description: "AMX TMUL Information",
            supported: true,
        });
    }
}
