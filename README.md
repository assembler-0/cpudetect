# cpudetect - Modern CPU Feature Detection Library

A clean, modular Rust library for comprehensive x86_64 CPU feature detection with a beautiful command-line interface. 

## Building

### Library (static and dynamic)

```bash
cargo build --release --lib
```

This produces:
- `libcpudetect.rlib` (Rust static library)
- `libcpudetect.so` (dynamic library on Linux)
- `libcpudetect.a` (C-compatible static library)

### Frontend Binary

```bash
cargo build --release --bin lscpu
```

### All

```bash
cargo build --release
```

## Features

### 🎯 Comprehensive Feature Detection (200+ features)

- ✅ **SIMD Extensions**: SSE, SSE2, SSE3, SSSE3, SSE4.1, SSE4.2, AVX, AVX2, AVX-512 (all variants), AMX
- ✅ **Cryptography**: AES-NI, SHA, PCLMULQDQ, GFNI, VAES, VPCLMULQDQ
- ✅ **Security**: 
  - Speculation control (IBRS, IBPB, STIBP, SSBD)
  - Memory protection (SMEP, SMAP, PKU, PKS, CET, UMIP)
  - Security extensions (SGX, TME, Key Locker)
  - Vulnerability mitigations (L1D_FLUSH, MD_CLEAR, etc.)
- ✅ **Virtualization**: VMX (Intel VT-x), SVM (AMD-V), hypervisor detection
- ✅ **Performance**: BMI1/2, ADX, POPCNT, ERMS, FSRM, FMA, hardware prefetchers
- ✅ **Memory Management**: 5-level paging (LA57), 1GB pages, PCID, INVPCID
- ✅ **Modern Extensions**: 
  - Intel: AMX, AVX-512 FP16, UINTR, SERIALIZE, WAITPKG, ENQCMD
  - AMD: CLZERO, WBNOINVD, SSE4a, XOP, FMA4, TBM
- ✅ **Debug & Power**: Intel PT, Architectural LBR, HWP, MONITOR/MWAIT
- ✅ **Hybrid Architecture**: P-core/E-core detection (Intel 12th gen+)

### 🎨 Beautiful Command-Line Interface

- Color-coded output by feature category
- Clean, organized presentation
- Unicode box drawing characters
- Feature grouping (SIMD, Cryptography, Security, etc.)

### 🏗️ Clean Architecture

- Modular design following Unix philosophy
- Each module does one thing well`
- Zero-cost abstractions
- Minimal dependencies


## Quick Start
- [see here](./doc/QUICKSTART.md)

## License

Apache-2.0

