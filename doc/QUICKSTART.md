# Quick Start Guide

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cpudetect = { path = "/path/to/cpudetect" }
```

Or build as a standalone library:

```bash
cargo build --release
```

## Using the Command-Line Tool

The fastest way to see your CPU information:

```bash
cargo run --release --bin lscpu
```

Or after building:

```bash
./target/release/lscpu
```

## Using as a Library

### Basic Usage

```rust
use cpudetect::CpuInfo;

fn main() {
    let cpu = CpuInfo::detect();
    println!("{}", cpu);
}
```

### Check for Specific Features

```rust
use cpudetect::CpuFeatures;

fn main() {
    let features = CpuFeatures::detect();
    
    if features.has_feature("AVX2") {
        println!("AVX2 supported - can use fast vectorized code");
    }
    
    if features.has_feature("SHA") {
        println!("Hardware SHA extensions available");
    }
}
```

### Detect Topology

```rust
use cpudetect::CpuTopology;

fn main() {
    let topology = CpuTopology::detect();
    
    println!("Physical cores: {}", topology.physical_cores);
    println!("Logical processors: {}", topology.logical_processors);
    
    if topology.has_hyperthreading {
        println!("Hyper-Threading is enabled");
    }
    
    if topology.hybrid {
        println!("Hybrid CPU (P-cores + E-cores)");
    }
}
```

### Cache Information

```rust
use cpudetect::CacheInfo;

fn main() {
    let caches = CacheInfo::detect_all();
    
    for cache in &caches {
        println!("{}", cache);
    }
}
```

### Vendor Information

```rust
use cpudetect::VendorInfo;

fn main() {
    let vendor = VendorInfo::detect();
    
    println!("Vendor: {}", vendor.vendor_string);
    println!("Brand: {}", vendor.brand_string);
    println!("Family: 0x{:X}, Model: 0x{:X}, Stepping: {}", 
             vendor.family, vendor.model, vendor.stepping);
}
```

### Low-Level CPUID Access

```rust
use cpudetect::cpuid::{cpuid, max_cpuid_leaf};

fn main() {
    let max_leaf = max_cpuid_leaf();
    println!("Max CPUID leaf: 0x{:X}", max_leaf);
    
    // Get leaf 1 (basic CPU info)
    let result = cpuid(1, 0);
    println!("EAX: 0x{:08X}", result.eax);
    println!("EBX: 0x{:08X}", result.ebx);
    println!("ECX: 0x{:08X}", result.ecx);
    println!("EDX: 0x{:08X}", result.edx);
    
    // Check specific bit
    use cpudetect::cpuid::Register;
    if result.is_bit_set(Register::Edx, 26) {
        println!("SSE2 supported");
    }
}
```

## Building for Different Targets

### Static Library (.rlib)

```bash
cargo build --release --lib
# Output: target/release/libcpudetect.rlib
```

### Dynamic Library (.so / .dll / .dylib)

```bash
cargo rustc --release --lib -- --crate-type=cdylib
# Output: 
#   Linux: target/release/libcpudetect.so
#   Windows: target/release/cpudetect.dll
#   macOS: target/release/libcpudetect.dylib
```

### C-Compatible Static Library

Already configured in `Cargo.toml`:

```bash
cargo build --release
# Output: target/release/libcpudetect.a
```

### Binary

```bash
cargo build --release --bin lscpu
# Output: target/release/lscpu
```

## Examples

Run the included examples:

```bash
# Build examples manually
cd examples
rustc --edition 2021 -L ../target/release/deps \
      --extern cpudetect=../target/release/libcpudetect.rlib \
      --extern bitflags=../target/release/deps/libbitflags-*.rlib \
      check_features.rs

./check_features
```

Available examples:
- `check_features.rs` - Check for specific CPU features
- `cache_info.rs` - Display cache hierarchy
- `cpu_topology.rs` - Show CPU topology
- `raw_cpuid.rs` - Low-level CPUID access

## Common Use Cases

### Optimizing Code at Runtime

```rust
use cpudetect::CpuFeatures;

fn process_data(data: &[f32]) -> Vec<f32> {
    let features = CpuFeatures::detect();
    
    if features.has_feature("AVX2") {
        process_data_avx2(data)
    } else if features.has_feature("SSE4.2") {
        process_data_sse42(data)
    } else {
        process_data_scalar(data)
    }
}
```

### Thread Pool Sizing

```rust
use cpudetect::CpuTopology;
use std::thread;

fn optimal_thread_count() -> usize {
    let topology = CpuTopology::detect();
    
    if topology.hybrid {
        // For hybrid CPUs, use physical cores
        topology.physical_cores as usize
    } else {
        // For non-hybrid, use all logical processors
        topology.logical_processors as usize
    }
}
```

### Cache-Aware Algorithms

```rust
use cpudetect::CacheInfo;

fn get_cache_line_size() -> usize {
    let caches = CacheInfo::detect_all();
    
    caches
        .iter()
        .find(|c| matches!(c.level, cpudetect::cache::CacheLevel::L1))
        .map(|c| c.line_size as usize)
        .unwrap_or(64) // Default to 64 bytes
}
```

## Performance Tips

1. **Cache detection results**: CPUID is relatively expensive, cache results at startup
2. **Use bitflags**: The `FeatureSet` bitflags are very fast for checking multiple features
3. **Compile-time vs runtime**: Use `#[cfg(target_feature)]` when possible, this library for runtime detection

## Troubleshooting

### Binary doesn't run

Make sure you're on x86_64:
```bash
rustc --version --verbose | grep host
```

### Features not detected

Some features require OS support (e.g., AVX requires OS to save/restore YMM registers). The OSXSAVE flag indicates OS support.

### Library linking errors

Ensure bitflags dependency is available:
```bash
cargo build --release
```

## Next Steps

- Read the [Architecture documentation](../ARCHITECTURE.md)
- Check the [examples](examples/) directory
- Read module documentation: `cargo doc --open`
