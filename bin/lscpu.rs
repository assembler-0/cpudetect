use colored::*;
use cpudetect::*;

fn main() {
    let cpu = CpuInfo::detect();
    
    print_header();
    print_vendor_info(&cpu.vendor);
    print_topology_info(&cpu.topology);
    print_cache_info(&cpu.cache);
    print_features(&cpu.features);
}

fn print_header() {
    println!();
    println!("{}", "╔═══════════════════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║            CPU Feature Detection & Information               ║".bright_cyan());
    println!("{}", "╚═══════════════════════════════════════════════════════════════╝".bright_cyan());
    println!();
}

fn print_vendor_info(vendor: &VendorInfo) {
    println!("{}", "┌─ CPU Identification".bright_yellow().bold());
    println!("│");
    println!("│ {:<18} {}", "Vendor:".bright_white(), vendor.vendor_string.bright_green());
    println!("│ {:<18} {}", "Brand:".bright_white(), vendor.brand_string.bright_green());
    println!("│ {:<18} Family: {} | Model: {} | Stepping: {}", 
             "Signature:".bright_white(),
             format!("{:#x}", vendor.family).bright_cyan(),
             format!("{:#x}", vendor.model).bright_cyan(),
             vendor.stepping.to_string().bright_cyan());
    println!("│");
}

fn print_topology_info(topology: &CpuTopology) {
    println!("{}", "┌─ CPU Topology".bright_yellow().bold());
    println!("│");
    println!("│ {:<24} {}", "Logical Processors:".bright_white(), 
             topology.logical_processors.to_string().bright_green());
    println!("│ {:<24} {}", "Physical Cores:".bright_white(), 
             topology.physical_cores.to_string().bright_green());
    println!("│ {:<24} {}", "Threads per Core:".bright_white(), 
             topology.threads_per_core.to_string().bright_green());
    
    let ht_status = if topology.has_hyperthreading {
        "Enabled ✓".bright_green()
    } else {
        "Disabled ✗".bright_red()
    };
    println!("│ {:<24} {}", "Hyper-Threading:".bright_white(), ht_status);
    
    let hybrid_status = if topology.hybrid {
        "Yes (P-cores + E-cores) ⚡".bright_magenta()
    } else {
        "No".normal()
    };
    println!("│ {:<24} {}", "Hybrid Architecture:".bright_white(), hybrid_status);
    println!("│");
}

fn print_cache_info(caches: &[CacheInfo]) {
    println!("{}", "┌─ Cache Hierarchy".bright_yellow().bold());
    println!("│");
    
    for cache in caches {
        let level_str = format!("{:?}", cache.level);
        let type_str = format!("{:?}", cache.cache_type);
        let size_kb = cache.size / 1024;
        
        let level_color = match cache.level {
            cache::CacheLevel::L1 => level_str.bright_cyan(),
            cache::CacheLevel::L2 => level_str.bright_blue(),
            cache::CacheLevel::L3 => level_str.bright_magenta(),
            cache::CacheLevel::L4 => level_str.bright_yellow(),
        };
        
        println!("│ {} {} Cache:", level_color, type_str.bright_white());
        println!("│   ├─ Size: {} KB", size_kb.to_string().bright_green());
        println!("│   ├─ Associativity: {}-way", cache.ways.to_string().bright_cyan());
        println!("│   ├─ Line Size: {} bytes", cache.line_size.to_string().bright_cyan());
        println!("│   ├─ Sets: {}", cache.sets.to_string().bright_cyan());
        println!("│   └─ Shared by: {} threads", cache.shared_by.to_string().bright_cyan());
    }
    
    let total_cache: u64 = caches.iter().map(|c| c.size).sum();
    println!("│");
    println!("│ {:<18} {} KB ({:.2} MB)", 
             "Total Cache:".bright_white().bold(),
             (total_cache / 1024).to_string().bright_yellow(),
             (total_cache as f64 / 1024.0 / 1024.0).to_string().bright_yellow());
    println!("│");
}

fn print_features(features: &CpuFeatures) {
    println!("{}", "┌─ CPU Features".bright_yellow().bold());
    println!("│");
    
    // Print basic features
    println!("│ {} {}", "Basic Features:".bright_white().bold(), format!("{:?}", features.basic).bright_black());
    println!("│");
    
    // Group features by category
    let categories = [
        (features::FeatureCategory::Simd, "🔢", "SIMD & Vector Extensions"),
        (features::FeatureCategory::Cryptography, "🔐", "Cryptography"),
        (features::FeatureCategory::Security, "🛡️", "Security Features"),
        (features::FeatureCategory::Virtualization, "💻", "Virtualization"),
        (features::FeatureCategory::Performance, "⚡", "Performance"),
        (features::FeatureCategory::Memory, "🗄️", "Memory Management"),
    ];
    
    for (category, icon, name) in &categories {
        let category_features = features.features_by_category(*category);
        if !category_features.is_empty() {
            println!("│ {}  {}", icon, name.bright_white().bold());
            
            let mut line = String::from("│   ");
            let mut count = 0;
            
            for feature in category_features {
                let feature_str = format!("{} ", feature.name);
                let colored_feature = match category {
                    features::FeatureCategory::Simd => feature_str.bright_cyan(),
                    features::FeatureCategory::Cryptography => feature_str.bright_magenta(),
                    features::FeatureCategory::Security => feature_str.bright_red(),
                    features::FeatureCategory::Virtualization => feature_str.bright_blue(),
                    features::FeatureCategory::Performance => feature_str.bright_yellow(),
                    features::FeatureCategory::Memory => feature_str.bright_green(),
                    _ => feature_str.normal(),
                };
                
                if count > 0 && count % 6 == 0 {
                    println!("{}", line);
                    line = String::from("│   ");
                }
                
                line.push_str(&format!("{} ", colored_feature));
                count += 1;
            }
            
            if !line.trim_end().is_empty() {
                println!("{}", line);
            }
            println!("│");
        }
    }
    
    // Summary
    let total_features = features.all_supported().len();
    println!("│ {}: {}", 
             "Total Features Detected".bright_white().bold(),
             total_features.to_string().bright_green().bold());
    
    println!();
    println!("{}", "└────────────────────────────────────────────────────────────────".bright_black());
    println!();
}