//! Rust bindings for [libpcuid](https://github.com/anrieff/libcpuid)
//! CPU detection and feature extraction library.
//!
//! `rust-cpuid` provides a high-level interface for getting information
//! about the features of the CPU that runs your code. All the essential
//! work is done by the `libcpuid` C library and exposed through Rust's
//! FFI mechanism as a simple and convenient API.
//!
//! # Available features
//!
//! * CPU vendor, brand and codename detection
//! * information about number of cores (both physical and logical)
//! * cache size
//! * clock frequency
//!
//! # Installation
//!
//! First - download, and build libcpuid as [described in the
//! readme](https://github.com/anrieff/libcpuid). Install it by running `make
//! install` (you may want to run `ldconfig` afterwards).
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! cpuid = "*"
//! ```
//!
//! # Example
//!
//! ```rust
//! extern crate cpuid;
//!
//! fn main () {
//!     match cpuid::identify() {
//!         Ok(info) => {
//!             println!("Found: {} CPU, model: {}", info.vendor, info.codename);
//!             println!("The full brand string is: {}", info.brand);
//!             println!("Hardware AES support: {}", if info.has_feature(cpuid::CpuFeature::AES) { "yes" } else { "no" });
//!         },
//!         Err(err) => println!("cpuid error: {}", err),
//!     };
//!     match cpuid::clock_frequency() {
//!         Some(frequency) => println!("CPU speed: {} MHz", frequency),
//!         None => println!("Couldn't get CPU speed."),
//!     };
//! }
//! ```

extern crate libc;

use std::ffi::CStr;
use std::str;

mod ffi;

/// A struct holding information about CPU features.
///
/// This data structure is returned by `identify()`. You can consult
/// [libcpuid docs for cpu_id_t](http://libcpuid.sourceforge.net/doxy/structcpu__id__t.html)
/// for more detailed descriptions of these fields.
pub struct CpuInfo {
    /// CPU vendor string, for example *GenuineIntel*.
    pub vendor: String,
    /// Brand string, for example *Intel(R) Core(TM) i5-2410M CPU @ 2.30GHz*.
    pub brand: String,
    /// Brief CPU codename, such as *Sandy Bridge (Core i5)*.
    pub codename: String,
    /// Number of physical cores of the current CPU.
    pub num_cores: i32,
    /// Number of logical processors (may include HyperThreading or such).
    pub num_logical_cpus: i32,
    /// Total number of logical processors.
    pub total_logical_cpus: i32,
    /// L1 data cache size in kB. `Some(0)` if the CPU lacks cache, `None` if it couldn't be determined.
    pub l1_data_cache: Option<i32>,
    /// L1 instruction cache size in kB. `Some(0)` if the CPU lacks cache, `None` if it couldn't be determined.
    pub l1_instruction_cache: Option<i32>,
    /// L2 cache size in kB. `Some(0)` if the CPU lacks L2 cache, `None` if it couldn't be determined.
    pub l2_cache: Option<i32>,
    /// L3 cache size in kB. `Some(0)` if the CPU lacks L3 cache, `None` if it couldn't be determined.
    pub l3_cache: Option<i32>,
    flags: [u8; ffi::CPU_FLAGS_MAX],
}

/// CPU feature identifiers.
#[derive(Clone, Copy)]
pub enum CpuFeature {
    FloatingPointUnit = 0,
    VirtualModeExtension,
    DebugingExtension,
    PageSizeExtension,
    TimestampCounter,
    ModelSpecificRegisters,
    PhysicalAddressExtension,
    MachineCheckException,
    CMPXCHG8B,
    APIC,
    MemoryTypeRangeRegisters,
    SysenterSysexit,
    PageGlobalEnable,
    MachineCheckArchitecture,
    CMOV,
    PageAttributeTable,
    PageAddressExtension36bit,
    ProcessorSerialNumber,
    CLFLUSH,
    DebugStore,
    ACPI,
    MMX,
    FXSAVE,
    SSE,
    SSE2,
    SelfSnoop,
    HyperThreading,
    ThermalMonitor,
    IA64,
    PendingBreakEnable,
    SSE3,
    PCLMULQDQ,
    DebugStore64,
    MONITOR,
    CplQualifiedDebugStore,
    VirtualisationTechnology,
    SaferModeExceptions,
    EnhancedSpeedStep,
    ThermalMonitor2,
    SSSE3,
    ContextId,
    CMPXCHG16B,
    SendTaskPriorityMessages,
    PerformanceCapabilitiesMSR,
    DirectCacheAccess,
    SSE41,
    SSE42,
    SyscallSysret,
    ExecuteDisableBit,
    MOVBE,
    POPCNT,
    AES,
    XSAVE,
    OSXSAVE,
    AdvancedVectorExtensions,
    MMXExtensions,
    AMD3DNow,
    AMD3DNowExtended,
    NoExecuteBit,
    FXSAVEOptimizations,
    RDTSCP,
    LongMode,
    LAHFLongMode,
    CoreMultiProcessingLegacyMode,
    AMDSecureVirtualMachine,
    LZCNT,
    MisalignedSSE,
    SSE4a,
    PREFETCH,
    OsVisibleWorkaround,
    InstructionBasedSampling,
    SSE5,
    SKINIT,
    WatchdogTimer,
    TemperatureSensor,
    FrequencyIDControl,
    VoltageIDControl,
    THERMTRIP,
    AMDThermalControl,
    SoftwareThermalControl,
    Multiplier100Mhz,
    HardwarePstateControl,
    ConstantTSCTicks,
    XOP,
    FMA3,
    FMA4,
    TrailingBitManipulation,
    FPConvert16Bit,
    RDRAND,
    X2APIC,
    CorePerformanceBoost,
    MPERF,
    ProcessorFeedbackInterface,
    ProcessorAccumulator,
    AVX2,
    BMI1,
    BMI2,
    HardwareLockElision,
    RestrictedTransactionalMemory,
    AVX512F,
    AVX512DQ,
    AVX512PF,
    AVX512ER,
    AVX512CD,
    SHASupport,
    AVX512BW,
    AVX512VL,
    SGX,
    RDSEED,
    ArbitraryPrecision,
    NumCpuFeatures,
}

/// CPU SGX feature identifiers.
#[derive(Clone, Copy)]
pub enum CpuSGXFeature {
    IntelSGX1,
    IntelSGX2,
    NumSGXFeature,
}

impl CpuInfo {
    /// Checks if current CPU supports given feature.
    ///
    /// See `CpuFeature` for a list of available feature identifiers.
    pub fn has_feature(&self, feature: CpuFeature) -> bool {
        self.flags[feature as usize] == 1u8
    }
}

/// Checks if the CPUID instruction is present.
pub fn is_present() -> bool {
    unsafe { ffi::cpuid_present() == 1 }
}

/// Returns libcpuid version string.
pub fn version() -> String {
    unsafe {
        let ptr = ffi::cpuid_lib_version();
        let bytes = CStr::from_ptr(ptr).to_bytes();
        str::from_utf8(bytes).ok().expect("Invalid UTF8 string").to_string()
    }
}

/// Returns last libcpuid error string.
pub fn error() -> String {
    unsafe {
        let ptr = ffi::cpuid_error();
        let bytes = CStr::from_ptr(ptr).to_bytes();
        str::from_utf8(bytes).ok().expect("Invalid UTF8 string").to_string()
    }
}

/// Tries to identify the current CPU and its features.
///
/// In case of successful detection, a `CpuInfo` struct is returned (wrapped
/// with `Ok`) which contains all available data about the processor.
/// If libcpuid encounters an error, `identify` returns an `Err` with
/// the error message inside.
pub fn identify() -> Result<CpuInfo, String> {
    let mut raw: ffi::cpu_raw_data_t = Default::default();
    let raw_result = unsafe { ffi::cpuid_get_raw_data(&mut raw) };
    if raw_result != 0 {
        return Err(error());
    }
    let mut data: ffi::cpu_id_t = Default::default();
    let identify_result = unsafe { ffi::cpu_identify(&mut raw, &mut data) };
    if identify_result != 0 {
        Err(error())
    } else {
        Ok(CpuInfo {
            vendor: String::from_utf8(data.vendor_str.iter().map(|&x| x as u8).collect())
                        .ok()
                        .expect("Invalid vendor string"),
            brand: String::from_utf8(data.brand_str.iter().map(|&x| x as u8).collect())
                       .ok()
                       .expect("Invalid brand string"),
            codename: String::from_utf8(data.cpu_codename.iter().map(|&x| x as u8).collect())
                          .ok()
                          .expect("Invalid codename string"),
            num_cores: data.num_cores,
            num_logical_cpus: data.num_logical_cpus,
            total_logical_cpus: data.total_logical_cpus,
            l1_data_cache: if data.l1_data_cache != -1 {
                Some(data.l1_data_cache)
            } else {
                None
            },
            l1_instruction_cache: if data.l1_instruction_cache != -1 {
                Some(data.l1_instruction_cache)
            } else {
                None
            },
            l2_cache: if data.l2_cache != -1 {
                Some(data.l2_cache)
            } else {
                None
            },
            l3_cache: if data.l3_cache != -1 {
                Some(data.l3_cache)
            } else {
                None
            },
            flags: data.flags,
        })
    }
}

/// Gets the CPU clock frequency in MHz.
///
/// The underlying implementation uses several methods to discover CPU
/// speed, including direct measurement. If all these methods fail, function
/// returns `None`.
pub fn clock_frequency() -> Option<i32> {
    let frequency = unsafe { ffi::cpu_clock() };
    if frequency != -1 {
        Some(frequency)
    } else {
        None
    }
}

#[test]
fn test_is_present() {
    assert!(is_present());
}
