use cpudetect::*;
use colored::*;

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

fn print_gradient_header(title: &str, icon: &str, color: Color) {
    let width = 70;
    let title_with_icon = format!("{} {}", icon, title);
    let padding = (width - title_with_icon.len() - 2) / 2;
    
    println!("\n{}", "═".repeat(width).color(color).bold());
    println!("{}{}{}", 
        " ".repeat(padding),
        title_with_icon.color(color).bold(),
        " ".repeat(width - padding - title_with_icon.len()));
    println!("{}", "═".repeat(width).color(color).bold());
}

fn print_header() {
    println!("\n{}", "╔══════════════════════════════════════════════════════════════════════╗".bright_cyan().bold());
    println!("{}", "║                                                                      ║".bright_cyan().bold());
    println!("{}", "║                CPUDETECT - lscpu rust re-implementation              ║".bright_cyan().bold());
    println!("{}", "║                        Modern System Analysis                        ║".bright_cyan().bold());
    println!("{}", "║                                                                      ║".bright_cyan().bold());
    println!("{}", "╚══════════════════════════════════════════════════════════════════════╝".bright_cyan().bold());
}

fn print_vendor_info(vendor: &VendorInfo) {
    print_gradient_header("CPU IDENTIFICATION", "🔍", Color::BrightMagenta);
    
    println!("\n  {} {:<12} {}", "●".bright_magenta(), "Vendor:".bright_white().bold(), vendor.vendor_string.bright_yellow());
    println!("  {} {:<12} {}", "●".bright_magenta(), "Brand:".bright_white().bold(), vendor.brand_string.bright_green());
    println!("  {} {:<12} {}", "●".bright_magenta(), "Family:".bright_white().bold(), format!("{:#x}", vendor.family).bright_cyan());
    println!("  {} {:<12} {}", "●".bright_magenta(), "Model:".bright_white().bold(), format!("{:#x}", vendor.model).bright_cyan());
    println!("  {} {:<12} {}", "●".bright_magenta(), "Stepping:".bright_white().bold(), vendor.stepping.to_string().bright_cyan());
}

fn print_topology_info(topology: &CpuTopology) {
    print_gradient_header("CPU TOPOLOGY", "⚙️", Color::BrightBlue);
    
    println!("\n  {} {:<22} {}", "◆".bright_blue(), "Logical Processors:".bright_white().bold(), topology.logical_processors.to_string().bright_yellow().bold());
    println!("  {} {:<22} {}", "◆".bright_blue(), "Physical Cores:".bright_white().bold(), topology.physical_cores.to_string().bright_green().bold());
    println!("  {} {:<22} {}", "◆".bright_blue(), "Threads per Core:".bright_white().bold(), topology.threads_per_core.to_string().bright_cyan());
    
    let ht_status = if topology.has_hyperthreading {
        format!("{} Enabled", "✓".bright_green())
    } else {
        format!("{} Disabled", "✗".bright_red())
    };
    println!("  {} {:<22} {}", "◆".bright_blue(), "Hyper-Threading:".bright_white().bold(), ht_status);
    
    let hybrid_status = if topology.hybrid {
        format!("{} Yes (P-cores + E-cores)", "✓".bright_green())
    } else {
        format!("{} No", "✗".truecolor(100, 100, 100))
    };
    println!("  {} {:<22} {}", "◆".bright_blue(), "Hybrid Architecture:".bright_white().bold(), hybrid_status);
}

fn print_frequency_info(freq: &FrequencyInfo) {
    print_gradient_header("FREQUENCY INFORMATION", "⚡", Color::BrightYellow);
    
    println!();
    if let Some(base) = freq.base_mhz {
        println!("  {} {:<18} {} {}", "⚡".bright_yellow(), "Base Frequency:".bright_white().bold(), base.to_string().bright_green().bold(), "MHz".truecolor(150, 150, 150));
    }
    if let Some(max) = freq.max_mhz {
        println!("  {} {:<18} {} {}", "⚡".bright_yellow(), "Max Frequency:".bright_white().bold(), max.to_string().bright_red().bold(), "MHz".truecolor(150, 150, 150));
    }
    if let Some(bus) = freq.bus_mhz {
        println!("  {} {:<18} {} {}", "⚡".bright_yellow(), "Bus Frequency:".bright_white().bold(), bus.to_string().bright_cyan(), "MHz".truecolor(150, 150, 150));
    }
    if let Some(tsc) = freq.tsc_mhz {
        println!("  {} {:<18} {} {}", "⚡".bright_yellow(), "TSC Frequency:".bright_white().bold(), tsc.to_string().bright_magenta(), "MHz".truecolor(150, 150, 150));
    }
}

fn print_address_info(addr: &AddressInfo) {
    print_gradient_header("ADDRESS SIZES", "📍", Color::BrightCyan);
    
    println!("\n  {} {:<20} {} {}", "▸".bright_cyan(), "Physical Address:".bright_white().bold(), addr.physical_bits.to_string().bright_yellow().bold(), "bits".truecolor(150, 150, 150));
    println!("  {} {:<20} {} {}", "▸".bright_cyan(), "Virtual Address:".bright_white().bold(), addr.virtual_bits.to_string().bright_green().bold(), "bits".truecolor(150, 150, 150));
    if let Some(guest) = addr.guest_physical_bits {
        println!("  {} {:<20} {} {}", "▸".bright_cyan(), "Guest Physical:".bright_white().bold(), guest.to_string().bright_magenta(), "bits".truecolor(150, 150, 150));
    }
}

fn print_cache_info(caches: &[CacheInfo]) {
    print_gradient_header("CACHE HIERARCHY", "💾", Color::BrightGreen);
    
    println!();
    for cache in caches {
        let type_str = format!("{:?}", cache.cache_type);
        let size_kb = cache.size / 1024;
        
        let (icon, color) = match cache.level {
            cpudetect::CacheLevel::L1 => ("L1", Color::BrightRed),
            cpudetect::CacheLevel::L2 => ("L2", Color::BrightYellow),
            cpudetect::CacheLevel::L3 => ("L3", Color::BrightGreen),
            cpudetect::CacheLevel::L4 => ("L4", Color::BrightCyan),
        };
        
        println!("  {} {} {} Cache", "▣".color(color).bold(), icon.color(color).bold(), type_str.bright_white().bold());
        println!("    {} {:<16} {} KB", "├─".truecolor(100, 100, 100), "Size:".truecolor(200, 200, 200), size_kb.to_string().bright_cyan());
        println!("    {} {:<16} {}-way", "├─".truecolor(100, 100, 100), "Associativity:".truecolor(200, 200, 200), cache.ways.to_string().bright_yellow());
        println!("    {} {:<16} {} bytes", "├─".truecolor(100, 100, 100), "Line Size:".truecolor(200, 200, 200), cache.line_size.to_string().bright_magenta());
        println!("    {} {:<16} {}", "├─".truecolor(100, 100, 100), "Sets:".truecolor(200, 200, 200), cache.sets.to_string().bright_green());
        println!("    {} {:<16} {} threads\n", "└─".truecolor(100, 100, 100), "Shared by:".truecolor(200, 200, 200), cache.shared_by.to_string().bright_blue());
    }

    let total_cache: u64 = caches.iter().map(|c| c.size).sum();
    println!("  {} {} {} KB {} MB {}",
        "═".repeat(3).bright_green(),
        "Total Cache:".bright_white().bold(),
        (total_cache / 1024).to_string().bright_yellow().bold(),
        format!("({:.2}", total_cache as f64 / 1024.0 / 1024.0).bright_green(),
        ")".bright_green());
}

fn print_tlb_info(tlb: &TlbInfo) {
    if tlb.entries.is_empty() {
        return;
    }

    print_gradient_header("TLB INFORMATION", "🗂️", Color::BrightMagenta);
    println!();
    for entry in &tlb.entries {
        println!("  {} {} TLB {} {} {} entries {} {}",
            "◉".bright_magenta(),
            entry.tlb_type.to_string().bright_white().bold(),
            "(".truecolor(100, 100, 100),
            entry.page_size.to_string().bright_cyan(),
            "pages):".truecolor(100, 100, 100),
            entry.entries.to_string().bright_yellow(),
            entry.associativity.to_string().truecolor(150, 150, 150));
    }
}

fn print_power_info(power: &PowerInfo) {
    print_gradient_header("POWER MANAGEMENT", "🔋", Color::BrightYellow);
    
    let features = [
        (power.digital_thermal_sensor, "Digital Thermal Sensor"),
        (power.turbo_boost, "Turbo Boost"),
        (power.turbo_boost_max_3, "Turbo Boost Max 3.0"),
        (power.arat, "APIC Timer Always Running"),
        (power.hwp, "Hardware P-States (HWP)"),
        (power.hwp_notification, "HWP Notification"),
        (power.hwp_activity_window, "HWP Activity Window"),
        (power.hwp_epp, "HWP Energy Performance Preference"),
        (power.hwp_package, "HWP Package Level Control"),
        (power.hdc, "Hardware Duty Cycling"),
        (power.thread_director, "Thread Director"),
        (power.pln, "Power Limit Notification"),
        (power.pts, "Package Thermal Status"),
    ];
    
    println!();
    for (enabled, name) in features {
        if enabled {
            println!("  {} {}", "✓".bright_green().bold(), name.bright_white());
        }
    }
}

fn print_platform_info(platform: &PlatformInfo) {
    print_gradient_header("PLATFORM INFORMATION", "🖥️", Color::BrightCyan);
    
    println!("\n  {} {:<22} {}", "◆".bright_cyan(), "Max CPUID Leaf:".bright_white().bold(), format!("{:#x}", platform.max_cpuid_leaf).bright_yellow());
    println!("  {} {:<22} {}", "◆".bright_cyan(), "Max Extended Leaf:".bright_white().bold(), format!("{:#x}", platform.max_extended_leaf).bright_yellow());
    
    println!();
    let features = [
        (platform.time_stamp_counter, "Time Stamp Counter"),
        (platform.model_specific_registers, "Model Specific Registers"),
        (platform.apic_on_chip, "APIC On-Chip"),
        (platform.x2apic, "x2APIC"),
        (platform.tsc_invariant, "TSC Invariant"),
        (platform.tsc_deadline, "TSC Deadline Timer"),
    ];
    
    for (enabled, name) in features {
        if enabled {
            println!("  {} {}", "✓".bright_green().bold(), name.bright_white());
        }
    }
}

fn print_msr_info(msr: &MsrInfo) {
    print_gradient_header("MSR SUPPORT", "📊", Color::BrightMagenta);
    
    let features = [
        (msr.msr_support, "Model-Specific Registers Supported"),
        (msr.rdmsr_wrmsr, "RDMSR/WRMSR Instructions"),
        (msr.msr_platform_info, "Platform Info MSR"),
        (msr.msr_temperature, "Temperature MSR"),
        (msr.msr_perf_status, "Performance Status MSR"),
        (msr.msr_perf_ctl, "Performance Control MSR"),
        (msr.msr_energy_perf_bias, "Energy Performance Bias MSR"),
    ];
    
    println!();
    for (enabled, name) in features {
        if enabled {
            println!("  {} {}", "✓".bright_green().bold(), name.bright_white());
        }
    }
}

fn print_features(features: &CpuFeatures) {
    print_gradient_header("CPU FEATURES", "✨", Color::BrightGreen);

    let categories = [
        (features::FeatureCategory::Simd, "SIMD & Vector", "🎯", Color::BrightRed),
        (features::FeatureCategory::Cryptography, "Cryptography", "🔐", Color::BrightYellow),
        (features::FeatureCategory::Security, "Security", "🛡️", Color::BrightMagenta),
        (features::FeatureCategory::Virtualization, "Virtualization", "☁️", Color::BrightCyan),
        (features::FeatureCategory::Performance, "Performance", "⚡", Color::BrightGreen),
        (features::FeatureCategory::Memory, "Memory", "💾", Color::BrightBlue),
        (features::FeatureCategory::Debug, "Debug", "🐛", Color::Yellow),
        (features::FeatureCategory::Power, "Power", "🔋", Color::Green),
        (features::FeatureCategory::System, "System", "⚙️", Color::Cyan),
    ];

    for (category, name, icon, color) in &categories {
        let all_category_features: Vec<&features::Feature> = features.all_features
            .iter()
            .filter(|f| f.category == *category)
            .collect();
        
        if !all_category_features.is_empty() {
            let supported_count = all_category_features.iter().filter(|f| f.supported).count();
            let total_count = all_category_features.len();
            
            println!("\n  {} {} {} {}", 
                icon,
                name.color(*color).bold(),
                format!("({}/{})", supported_count, total_count).truecolor(100, 100, 100),
                "─".repeat(50).truecolor(60, 60, 60));

            // Print supported features
            let mut count = 0;
            for feature in all_category_features.iter().filter(|f| f.supported) {
                if count % 4 == 0 {
                    print!("\n    ");
                }
                print!("{} {:<18}", "✓".bright_green(), feature.name.bright_white());
                count += 1;
            }
            if count > 0 {
                println!();
            }

            // Print missing features
            let missing: Vec<&&features::Feature> = all_category_features.iter()
                .filter(|f| !f.supported)
                .collect();
            
            if !missing.is_empty() {
                println!("\n    {} Missing features:", "⚠".bright_yellow());
                let mut count = 0;
                for feature in missing {
                    if count % 4 == 0 {
                        print!("\n    ");
                    }
                    print!("{} {:<18}", "✗".truecolor(150, 150, 150), feature.name.truecolor(120, 120, 120));
                    count += 1;
                }
                println!();
            }
        }
    }

    let total_features = features.all_supported().len();
    let total_checked = features.all_features.len();
    let missing_features = total_checked - total_features;
    
    println!("\n\n  {} {} {}",
        "═".repeat(3).bright_green().bold(),
        "Features Supported:".bright_white().bold(),
        format!("{}/{}", total_features, total_checked).bright_yellow().bold());
    
    if missing_features > 0 {
        println!("  {} {} {}",
            "═".repeat(3).truecolor(150, 150, 150),
            "Features Not Supported:".truecolor(150, 150, 150),
            missing_features.to_string().truecolor(120, 120, 120));
    }
    
    println!("\n{}", "═".repeat(70).truecolor(60, 60, 60));
    println!();
}
