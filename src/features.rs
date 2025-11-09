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

        // Leaf 7 subleaf 3
        if is_leaf_supported(7) {
            detect_leaf7_sub3(&mut all_features);
        }

        // Leaf 6: Thermal and Power Management
        if is_leaf_supported(6) {
            detect_thermal_power(&mut all_features);
        }

        // Leaf 0xA: Performance Monitoring
        if is_leaf_supported(0xA) {
            detect_perfmon(&mut all_features);
        }

        // Leaf 0x10: Resource Director Technology
        if is_leaf_supported(0x10) {
            detect_rdt(&mut all_features);
        }

        // Leaf 0x12: SGX Extended
        if is_leaf_supported(0x12) {
            detect_sgx_extended(&mut all_features);
        }

        // Leaf 0x18: Deterministic Address Translation
        if is_leaf_supported(0x18) {
            detect_address_translation(&mut all_features);
        }

        // Leaf 0x24: AVX10
        if is_leaf_supported(0x24) {
            detect_avx10(&mut all_features);
        }

        // Extended leaves: Additional AMD/Intel features
        if is_leaf_supported(0x8000_0001) {
            detect_extended_features(&mut all_features);
        }

        // AMD Extended Features
        if is_leaf_supported(0x8000_0008) {
            detect_amd_extended(&mut all_features);
        }

        // AMD SVM Extended
        if is_leaf_supported(0x8000_000A) {
            detect_amd_svm(&mut all_features);
        }

        // AMD Performance Optimization
        if is_leaf_supported(0x8000_001A) {
            detect_amd_perf_optimization(&mut all_features);
        }

        // AMD Memory Encryption
        if is_leaf_supported(0x8000_001F) {
            detect_amd_memory_encryption(&mut all_features);
        }

        // AMD Extended Features 2
        if is_leaf_supported(0x8000_0021) {
            detect_amd_extended_features2(&mut all_features);
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
        (
            17,
            "CET_SSS",
            FeatureCategory::Security,
            "Shadow Stack Select",
        ),
        (
            18,
            "AVX10",
            FeatureCategory::Simd,
            "AVX10 Converged Vector ISA",
        ),
        (
            19,
            "APX_F",
            FeatureCategory::Performance,
            "Advanced Performance Extensions",
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

fn detect_leaf7_sub3(features: &mut Vec<Feature>) {
    if !is_leaf_supported(7) {
        return;
    }

    let result = cpuid(7, 3);

    let edx_features = [
        (
            0,
            "AVX10_128",
            FeatureCategory::Simd,
            "AVX10 128-bit support",
        ),
        (
            1,
            "AVX10_256",
            FeatureCategory::Simd,
            "AVX10 256-bit support",
        ),
        (
            2,
            "AVX10_512",
            FeatureCategory::Simd,
            "AVX10 512-bit support",
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

fn detect_avx10(features: &mut Vec<Feature>) {
    if !is_leaf_supported(0x24) {
        return;
    }

    let result = cpuid(0x24, 0);

    let version = result.ebx & 0xFF;
    if version > 0 {
        features.push(Feature {
            name: format!("AVX10_V{}", version),
            category: FeatureCategory::Simd,
            description: "AVX10 Version",
            supported: true,
        });
    }

    if result.ebx & (1 << 16) != 0 {
        features.push(Feature {
            name: "AVX10_128".to_string(),
            category: FeatureCategory::Simd,
            description: "AVX10 128-bit vector support",
            supported: true,
        });
    }
    if result.ebx & (1 << 17) != 0 {
        features.push(Feature {
            name: "AVX10_256".to_string(),
            category: FeatureCategory::Simd,
            description: "AVX10 256-bit vector support",
            supported: true,
        });
    }
    if result.ebx & (1 << 18) != 0 {
        features.push(Feature {
            name: "AVX10_512".to_string(),
            category: FeatureCategory::Simd,
            description: "AVX10 512-bit vector support",
            supported: true,
        });
    }
}

fn detect_thermal_power(features: &mut Vec<Feature>) {
    if !is_leaf_supported(6) {
        return;
    }

    let result = cpuid(6, 0);

    let eax_features = [
        (0, "DTHERM", FeatureCategory::Power, "Digital thermal sensor"),
        (
            1,
            "TURBO_BOOST",
            FeatureCategory::Power,
            "Intel Turbo Boost",
        ),
        (
            2,
            "ARAT",
            FeatureCategory::Power,
            "APIC-Timer-always-running",
        ),
        (4, "PLN", FeatureCategory::Power, "Power limit notification"),
        (
            5,
            "ECMD",
            FeatureCategory::Power,
            "Clock modulation duty cycle",
        ),
        (6, "PTM", FeatureCategory::Power, "Package thermal management"),
        (7, "HWP", FeatureCategory::Power, "Hardware P-states (HWP)"),
        (
            8,
            "HWP_NOTIFICATION",
            FeatureCategory::Power,
            "HWP notification",
        ),
        (
            9,
            "HWP_ACTIVITY_WINDOW",
            FeatureCategory::Power,
            "HWP activity window",
        ),
        (
            10,
            "HWP_ENERGY_PERF",
            FeatureCategory::Power,
            "HWP energy/performance",
        ),
        (
            11,
            "HWP_PACKAGE",
            FeatureCategory::Power,
            "HWP package level request",
        ),
        (13, "HDC", FeatureCategory::Power, "Hardware Duty Cycling"),
        (
            14,
            "TURBO_BOOST_3",
            FeatureCategory::Power,
            "Intel Turbo Boost Max 3.0",
        ),
        (
            15,
            "HWP_CAPABILITIES",
            FeatureCategory::Power,
            "HWP capabilities",
        ),
        (16, "HWP_PECI", FeatureCategory::Power, "HWP PECI override"),
        (17, "HWP_FLEXIBLE", FeatureCategory::Power, "Flexible HWP"),
        (
            18,
            "HWP_FAST_ACCESS",
            FeatureCategory::Power,
            "Fast access HWP request",
        ),
        (
            19,
            "HW_FEEDBACK",
            FeatureCategory::Performance,
            "HW_FEEDBACK interface",
        ),
        (
            20,
            "IGNORE_IDLE",
            FeatureCategory::Power,
            "Ignore idle logical processor HWP request",
        ),
        (
            23,
            "THREAD_DIRECTOR",
            FeatureCategory::Performance,
            "Intel Thread Director",
        ),
        (
            24,
            "THERM_INTERRUPT",
            FeatureCategory::Power,
            "IA32_THERM_INTERRUPT MSR bit 25",
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

    let ecx_features = [
        (
            0,
            "HW_FEEDBACK_PERF",
            FeatureCategory::Performance,
            "Hardware feedback performance/energy bias",
        ),
        (
            1,
            "HW_FEEDBACK_SIZE",
            FeatureCategory::Performance,
            "Hardware feedback interface size",
        ),
        (
            3,
            "PERF_PREF",
            FeatureCategory::Performance,
            "Performance-energy bias preference",
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

fn detect_perfmon(features: &mut Vec<Feature>) {
    if !is_leaf_supported(0xA) {
        return;
    }

    let result = cpuid(0xA, 0);

    let version = result.eax & 0xFF;
    if version > 0 {
        features.push(Feature {
            name: format!("PERFMON_V{}", version),
            category: FeatureCategory::Performance,
            description: "Performance Monitoring version",
            supported: true,
        });
    }

    let ebx_features = [
        (
            0,
            "PERFMON_CORE_CYCLES",
            FeatureCategory::Performance,
            "Core cycle event available",
        ),
        (
            1,
            "PERFMON_INSTR_RETIRED",
            FeatureCategory::Performance,
            "Instruction retired event available",
        ),
        (
            2,
            "PERFMON_REF_CYCLES",
            FeatureCategory::Performance,
            "Reference cycles event available",
        ),
        (
            3,
            "PERFMON_LLC_REF",
            FeatureCategory::Performance,
            "LLC reference event available",
        ),
        (
            4,
            "PERFMON_LLC_MISSES",
            FeatureCategory::Performance,
            "LLC misses event available",
        ),
        (
            5,
            "PERFMON_BR_INSTR",
            FeatureCategory::Performance,
            "Branch instruction retired event available",
        ),
        (
            6,
            "PERFMON_BR_MISPREDICT",
            FeatureCategory::Performance,
            "Branch mispredict retired event available",
        ),
    ];

    for (bit, name, category, desc) in ebx_features.iter() {
        features.push(Feature {
            name: name.to_string(),
            category: *category,
            description: desc,
            supported: (result.ebx & (1 << bit)) == 0,
        });
    }

    let edx_features = [
        (
            0,
            "PERFMON_FIXED_CTR0",
            FeatureCategory::Performance,
            "Fixed counter 0",
        ),
        (
            1,
            "PERFMON_FIXED_CTR1",
            FeatureCategory::Performance,
            "Fixed counter 1",
        ),
        (
            2,
            "PERFMON_FIXED_CTR2",
            FeatureCategory::Performance,
            "Fixed counter 2",
        ),
        (
            15,
            "PERFMON_ANYTHREAD_DEPRECATED",
            FeatureCategory::Performance,
            "AnyThread deprecation",
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

fn detect_rdt(features: &mut Vec<Feature>) {
    if !is_leaf_supported(0x10) {
        return;
    }

    let result = cpuid(0x10, 0);

    let ebx_features = [
        (
            1,
            "RDT_L3_MONITORING",
            FeatureCategory::Performance,
            "L3 Cache Monitoring",
        ),
        (
            2,
            "RDT_L2_MONITORING",
            FeatureCategory::Performance,
            "L2 Cache Monitoring",
        ),
        (
            3,
            "RDT_MBA",
            FeatureCategory::Performance,
            "Memory Bandwidth Allocation",
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

    if result.ebx & (1 << 1) != 0 {
        let l3_result = cpuid(0x10, 1);
        if l3_result.eax != 0 {
            features.push(Feature {
                name: "RDT_L3_CAT".to_string(),
                category: FeatureCategory::Performance,
                description: "L3 Cache Allocation Technology",
                supported: true,
            });
        }
        if l3_result.ecx & (1 << 2) != 0 {
            features.push(Feature {
                name: "RDT_L3_CDP".to_string(),
                category: FeatureCategory::Performance,
                description: "L3 Code/Data Prioritization",
                supported: true,
            });
        }
    }

    if result.ebx & (1 << 2) != 0 {
        let l2_result = cpuid(0x10, 2);
        if l2_result.eax != 0 {
            features.push(Feature {
                name: "RDT_L2_CAT".to_string(),
                category: FeatureCategory::Performance,
                description: "L2 Cache Allocation Technology",
                supported: true,
            });
        }
    }
}

fn detect_sgx_extended(features: &mut Vec<Feature>) {
    if !is_leaf_supported(0x12) {
        return;
    }

    let result = cpuid(0x12, 0);

    let eax_features = [
        (0, "SGX1", FeatureCategory::Security, "SGX1 leaf functions"),
        (1, "SGX2", FeatureCategory::Security, "SGX2 leaf functions"),
        (5, "ENCLV", FeatureCategory::Security, "ENCLV leaves"),
        (6, "ENCLS", FeatureCategory::Security, "ENCLS leaves"),
    ];

    for (bit, name, category, desc) in eax_features.iter() {
        features.push(Feature {
            name: name.to_string(),
            category: *category,
            description: desc,
            supported: (result.eax & (1 << bit)) != 0,
        });
    }

    if result.ebx & 1 != 0 {
        features.push(Feature {
            name: "SGX_MISCSELECT".to_string(),
            category: FeatureCategory::Security,
            description: "SGX MISCSELECT support",
            supported: true,
        });
    }

    let sub1 = cpuid(0x12, 1);
    if sub1.eax != 0 || sub1.ebx != 0 || sub1.ecx != 0 || sub1.edx != 0 {
        features.push(Feature {
            name: "SGX_ATTRIBUTES".to_string(),
            category: FeatureCategory::Security,
            description: "SGX Attributes enumeration",
            supported: true,
        });
    }
}

fn detect_address_translation(features: &mut Vec<Feature>) {
    if !is_leaf_supported(0x18) {
        return;
    }

    let result = cpuid(0x18, 0);

    if result.eax != 0 {
        features.push(Feature {
            name: "DAT_ENUM".to_string(),
            category: FeatureCategory::Memory,
            description: "Deterministic Address Translation enumeration",
            supported: true,
        });
    }
}

fn detect_amd_svm(features: &mut Vec<Feature>) {
    if !is_leaf_supported(0x8000_000A) {
        return;
    }

    let result = cpuid(0x8000_000A, 0);

    let edx_features = [
        (
            0,
            "SVM_NPT",
            FeatureCategory::Virtualization,
            "Nested Page Tables",
        ),
        (
            1,
            "SVM_LBR_VIRT",
            FeatureCategory::Virtualization,
            "LBR Virtualization",
        ),
        (2, "SVM_LOCK", FeatureCategory::Virtualization, "SVM Lock"),
        (
            3,
            "SVM_NRIP",
            FeatureCategory::Virtualization,
            "NRIP Save",
        ),
        (
            4,
            "SVM_TSC_RATE",
            FeatureCategory::Virtualization,
            "TSC Rate MSR",
        ),
        (
            5,
            "SVM_VMCB_CLEAN",
            FeatureCategory::Virtualization,
            "VMCB Clean Bits",
        ),
        (
            6,
            "SVM_FLUSH_BY_ASID",
            FeatureCategory::Virtualization,
            "Flush by ASID",
        ),
        (
            7,
            "SVM_DECODE_ASSISTS",
            FeatureCategory::Virtualization,
            "Decode Assists",
        ),
        (
            10,
            "SVM_PAUSE_FILTER",
            FeatureCategory::Virtualization,
            "Pause Intercept Filter",
        ),
        (
            12,
            "SVM_PAUSE_THRESHOLD",
            FeatureCategory::Virtualization,
            "Pause Filter Threshold",
        ),
        (
            13,
            "SVM_AVIC",
            FeatureCategory::Virtualization,
            "Advanced Virtual Interrupt Controller",
        ),
        (
            15,
            "SVM_V_VMSAVE_VMLOAD",
            FeatureCategory::Virtualization,
            "Virtual VMSAVE/VMLOAD",
        ),
        (16, "SVM_VGIF", FeatureCategory::Virtualization, "Virtual GIF"),
        (
            17,
            "SVM_GMET",
            FeatureCategory::Virtualization,
            "Guest Mode Execute Trap",
        ),
        (
            18,
            "SVM_X2AVIC",
            FeatureCategory::Virtualization,
            "x2APIC Virtual Interrupt Controller",
        ),
        (
            19,
            "SVM_SSSE_ERR",
            FeatureCategory::Virtualization,
            "Supervisor Shadow Stack",
        ),
        (
            20,
            "SVM_SPEC_CTRL",
            FeatureCategory::Security,
            "SPEC_CTRL virtualization",
        ),
        (
            21,
            "SVM_ROGPT",
            FeatureCategory::Virtualization,
            "Read-Only Guest Page Table",
        ),
        (
            23,
            "SVM_HOST_MCE_OVERRIDE",
            FeatureCategory::Virtualization,
            "Host MCE Override",
        ),
        (
            24,
            "SVM_INVLPGB",
            FeatureCategory::Virtualization,
            "INVLPGB/TLBSYNC support",
        ),
        (25, "SVM_VNMI", FeatureCategory::Virtualization, "Virtual NMI"),
        (
            26,
            "SVM_IBS_VIRT",
            FeatureCategory::Virtualization,
            "IBS Virtualization",
        ),
        (
            27,
            "SVM_EXT_LVT",
            FeatureCategory::Virtualization,
            "Extended LVT offset fault change",
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

fn detect_amd_memory_encryption(features: &mut Vec<Feature>) {
    if !is_leaf_supported(0x8000_001F) {
        return;
    }

    let result = cpuid(0x8000_001F, 0);

    let eax_features = [
        (
            0,
            "SME",
            FeatureCategory::Security,
            "Secure Memory Encryption",
        ),
        (
            1,
            "SEV",
            FeatureCategory::Security,
            "Secure Encrypted Virtualization",
        ),
        (
            2,
            "PAGE_FLUSH_MSR",
            FeatureCategory::Security,
            "Page Flush MSR",
        ),
        (
            3,
            "SEV_ES",
            FeatureCategory::Security,
            "SEV Encrypted State",
        ),
        (
            4,
            "SEV_SNP",
            FeatureCategory::Security,
            "SEV Secure Nested Paging",
        ),
        (5, "VMPL", FeatureCategory::Security, "VM Permission Levels"),
        (
            6,
            "RMPQUERY",
            FeatureCategory::Security,
            "RMPQUERY instruction",
        ),
        (
            7,
            "VMPL_SSS",
            FeatureCategory::Security,
            "VMPL Supervisor Shadow Stack",
        ),
        (8, "SECURE_TSC", FeatureCategory::Security, "Secure TSC"),
        (
            9,
            "TSC_AUX_VIRT",
            FeatureCategory::Virtualization,
            "TSC_AUX Virtualization",
        ),
        (
            10,
            "HW_CACHE_COHERENCY",
            FeatureCategory::Security,
            "Hardware cache coherency",
        ),
        (
            11,
            "64BIT_HOST",
            FeatureCategory::Security,
            "SEV 64-bit host",
        ),
        (
            12,
            "REST_INJ",
            FeatureCategory::Security,
            "Restricted Injection",
        ),
        (13, "ALT_INJ", FeatureCategory::Security, "Alternate Injection"),
        (
            14,
            "DEBUG_SWAP",
            FeatureCategory::Debug,
            "SEV Debug register swap",
        ),
        (
            15,
            "PREVENT_HOST_IBS",
            FeatureCategory::Security,
            "Prevent host IBS",
        ),
        (
            16,
            "VTE",
            FeatureCategory::Security,
            "Virtual Transparent Encryption",
        ),
        (
            17,
            "VMGEXIT_PARAM",
            FeatureCategory::Virtualization,
            "VMGEXIT parameter",
        ),
        (
            18,
            "VIRT_TOM_MSR",
            FeatureCategory::Virtualization,
            "Virtual TOM MSR",
        ),
        (
            19,
            "IBS_VIRT_GIF",
            FeatureCategory::Virtualization,
            "IBS GIF virtualization",
        ),
        (
            24,
            "VMSA_REG_PROT",
            FeatureCategory::Security,
            "VMSA register protection",
        ),
        (
            25,
            "SMT_PROTECTION",
            FeatureCategory::Security,
            "SMT protection",
        ),
        (28, "SECURE_AVIC", FeatureCategory::Security, "Secure AVIC"),
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

fn detect_amd_extended_features2(features: &mut Vec<Feature>) {
    if !is_leaf_supported(0x8000_0021) {
        return;
    }

    let result = cpuid(0x8000_0021, 0);

    let eax_features = [
        (
            0,
            "NO_NESTED_DATA_BP",
            FeatureCategory::Security,
            "No nested data breakpoints",
        ),
        (
            1,
            "FS_GS_NO_SERIALIZING",
            FeatureCategory::Performance,
            "FS/GS base non-serializing",
        ),
        (
            2,
            "LFENCE_SERIALIZING",
            FeatureCategory::Security,
            "LFENCE always serializing",
        ),
        (
            3,
            "SMM_PG_CFG_LOCK",
            FeatureCategory::Security,
            "SMM page config lock",
        ),
        (
            6,
            "NULL_SEL_CLEARS_BASE",
            FeatureCategory::System,
            "Null selector clears base",
        ),
        (7, "UAI", FeatureCategory::Memory, "Upper Address Ignore"),
        (8, "AUTO_IBRS", FeatureCategory::Security, "Automatic IBRS"),
        (
            9,
            "NO_SMM_CTL_MSR",
            FeatureCategory::Security,
            "No SMM_CTL MSR",
        ),
        (
            10,
            "FSRS",
            FeatureCategory::Performance,
            "Fast short REP STOSB",
        ),
        (
            11,
            "FSRC",
            FeatureCategory::Performance,
            "Fast short REP CMPSB",
        ),
        (
            13,
            "PREFETCH_CTL",
            FeatureCategory::Performance,
            "Prefetch control MSR",
        ),
        (
            17,
            "CPUID_DIS",
            FeatureCategory::Security,
            "CPUID disable for non-privileged",
        ),
        (
            18,
            "EPSF",
            FeatureCategory::Security,
            "Enhanced Predictive Store Forwarding",
        ),
        (
            19,
            "AGPR",
            FeatureCategory::Performance,
            "Alternate GPR for exception state",
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

fn detect_amd_perf_optimization(features: &mut Vec<Feature>) {
    if !is_leaf_supported(0x8000_001A) {
        return;
    }

    let result = cpuid(0x8000_001A, 0);

    let eax_features = [
        (0, "FP128", FeatureCategory::Simd, "128-bit FP execution"),
        (
            1,
            "MOVU",
            FeatureCategory::Simd,
            "MOVU instructions better than MOVL/MOVH",
        ),
        (2, "FP256", FeatureCategory::Simd, "256-bit FP execution"),
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
