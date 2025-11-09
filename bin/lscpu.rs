use cpudetect::*;

fn main() {
    let cpu = CpuInfo::detect();

    print_header();
    print_vendor_info(&cpu.vendor);
    print_topology_info(&cpu.topology);
    print_frequency_info(&cpu.frequency);
    print_address_info(&cpu.address);
    print_cache_info(&cpu.cache);
    print_tlb_info(&cpu.tlb);
    print_power_info(&cpu.power);
    print_platform_info(&cpu.platform);
    print_msr_info(&cpu.msr);
    print_features(&cpu.features);
}

fn print_boxed_header(title: &str) {
    let header_len = title.len();
    let top_border = format!("┌{}┐", "─".repeat(header_len + 2));
    let middle_line = format!("│ {} │", title);
    let bottom_border = format!("└{}┘", "─".repeat(header_len + 2));

    println!("{}", top_border);
    println!("{}", middle_line);
    println!("{}\n", bottom_border);
}

fn print_header() {
    println!("CPU Information");
    println!("===============\n");
}

fn print_vendor_info(vendor: &VendorInfo) {
    print_boxed_header("CPU Identification");
    println!("  Vendor:    {}", vendor.vendor_string);
    println!("  Brand:     {}", vendor.brand_string);
    println!("  Family:    {:#x}", vendor.family);
    println!("  Model:     {:#x}", vendor.model);
    println!("  Stepping:  {}\n", vendor.stepping);
}

fn print_topology_info(topology: &CpuTopology) {
    print_boxed_header("CPU Topology");
    println!("  Logical Processors:  {}", topology.logical_processors);
    println!("  Physical Cores:      {}", topology.physical_cores);
    println!("  Threads per Core:    {}", topology.threads_per_core);
    println!(
        "  Hyper-Threading:     {}",
        if topology.has_hyperthreading {
            "Yes"
        } else {
            "No"
        }
    );
    println!(
        "  Hybrid Architecture: {}\n",
        if topology.hybrid { "Yes" } else { "No" }
    );
}

fn print_frequency_info(freq: &FrequencyInfo) {
    print_boxed_header("Frequency Information");
    if let Some(base) = freq.base_mhz {
        println!("  Base Frequency:  {} MHz", base);
    }
    if let Some(max) = freq.max_mhz {
        println!("  Max Frequency:   {} MHz", max);
    }
    if let Some(bus) = freq.bus_mhz {
        println!("  Bus Frequency:   {} MHz", bus);
    }
    if let Some(tsc) = freq.tsc_mhz {
        println!("  TSC Frequency:   {} MHz", tsc);
    }
    println!();
}

fn print_address_info(addr: &AddressInfo) {
    print_boxed_header("Address Sizes");
    println!("  Physical Address:  {} bits", addr.physical_bits);
    println!("  Virtual Address:   {} bits", addr.virtual_bits);
    if let Some(guest) = addr.guest_physical_bits {
        println!("  Guest Physical:    {} bits", guest);
    }
    println!();
}

fn print_cache_info(caches: &[CacheInfo]) {
    print_boxed_header("Cache Hierarchy");

    for cache in caches {
        let level_str = format!("{:?}", cache.level);
        let type_str = format!("{:?}", cache.cache_type);
        let size_kb = cache.size / 1024;

        println!("  {} {} Cache:", level_str, type_str);
        println!("    Size:           {} KB", size_kb);
        println!("    Associativity:  {}-way", cache.ways);
        println!("    Line Size:      {} bytes", cache.line_size);
        println!("    Sets:           {}", cache.sets);
        println!("    Shared by:      {} threads", cache.shared_by);
    }

    let total_cache: u64 = caches.iter().map(|c| c.size).sum();
    println!(
        "  Total Cache: {} KB ({:.2} MB)\n",
        total_cache / 1024,
        total_cache as f64 / 1024.0 / 1024.0
    );
}

fn print_tlb_info(tlb: &TlbInfo) {
    if tlb.entries.is_empty() {
        return;
    }

    print_boxed_header("TLB Information");
    for entry in &tlb.entries {
        println!(
            "  {} TLB ({} pages): {} entries, {}",
            entry.tlb_type,
            entry.page_size,
            entry.entries,
            entry.associativity
        );
    }
    println!();
}

fn print_power_info(power: &PowerInfo) {
    print_boxed_header("Power Management");
    if power.digital_thermal_sensor {
        println!("  Digital Thermal Sensor");
    }
    if power.turbo_boost {
        println!("  Turbo Boost");
    }
    if power.turbo_boost_max_3 {
        println!("  Turbo Boost Max 3.0");
    }
    if power.arat {
        println!("  APIC Timer Always Running");
    }
    if power.hwp {
        println!("  Hardware P-States (HWP)");
    }
    if power.hwp_notification {
        println!("  HWP Notification");
    }
    if power.hwp_activity_window {
        println!("  HWP Activity Window");
    }
    if power.hwp_epp {
        println!("  HWP Energy Performance Preference");
    }
    if power.hwp_package {
        println!("  HWP Package Level Control");
    }
    if power.hdc {
        println!("  Hardware Duty Cycling");
    }
    if power.thread_director {
        println!("  Thread Director");
    }
    if power.pln {
        println!("  Power Limit Notification");
    }
    if power.pts {
        println!("  Package Thermal Status");
    }
    println!();
}

fn print_platform_info(platform: &PlatformInfo) {
    print_boxed_header("Platform Information");
    println!("  Max CPUID Leaf:      {:#x}", platform.max_cpuid_leaf);
    println!("  Max Extended Leaf:   {:#x}", platform.max_extended_leaf);
    if platform.time_stamp_counter {
        println!("  Time Stamp Counter");
    }
    if platform.model_specific_registers {
        println!("  Model Specific Registers");
    }
    if platform.apic_on_chip {
        println!("  APIC On-Chip");
    }
    if platform.x2apic {
        println!("  x2APIC");
    }
    if platform.tsc_invariant {
        println!("  TSC Invariant");
    }
    if platform.tsc_deadline {
        println!("  TSC Deadline Timer");
    }
    println!();
}

fn print_msr_info(msr: &MsrInfo) {
    print_boxed_header("MSR Support");
    if msr.msr_support {
        println!("  Model-Specific Registers Supported");
    }
    if msr.rdmsr_wrmsr {
        println!("  RDMSR/WRMSR Instructions");
    }
    if msr.msr_platform_info {
        println!("  Platform Info MSR");
    }
    if msr.msr_temperature {
        println!("  Temperature MSR");
    }
    if msr.msr_perf_status {
        println!("  Performance Status MSR");
    }
    if msr.msr_perf_ctl {
        println!("  Performance Control MSR");
    }
    if msr.msr_energy_perf_bias {
        println!("  Energy Performance Bias MSR");
    }
    println!();
}

fn print_features(features: &CpuFeatures) {
    print_boxed_header("CPU Features");

    let categories = [
        (features::FeatureCategory::Simd, "SIMD & Vector"),
        (features::FeatureCategory::Cryptography, "Cryptography"),
        (features::FeatureCategory::Security, "Security"),
        (features::FeatureCategory::Virtualization, "Virtualization"),
        (features::FeatureCategory::Performance, "Performance"),
        (features::FeatureCategory::Memory, "Memory"),
        (features::FeatureCategory::Debug, "Debug"),
        (features::FeatureCategory::Power, "Power"),
        (features::FeatureCategory::System, "System"),
    ];

    for (category, name) in &categories {
        let category_features = features.features_by_category(*category);
        if !category_features.is_empty() {
            println!("  {}:", name);

            let mut line = String::from("    ");
            let mut count = 0;

            for feature in category_features {
                if count > 0 && count % 8 == 0 {
                    println!("{}", line);
                    line = String::from("    ");
                }

                line.push_str(&format!("{} ", feature.name));
                count += 1;
            }

            if !line.trim().is_empty() && line.trim() != "" {
                println!("{}", line);
            }
        }
    }

    let total_features = features.all_supported().len();
    println!("\n  Total Features: {}\n", total_features);
}