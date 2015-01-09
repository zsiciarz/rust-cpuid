use libc::{c_int, c_char, uint8_t, uint32_t, int32_t};

pub const MAX_CPUID_LEVEL: usize = 32;
pub const MAX_EXT_CPUID_LEVEL: usize = 32;
pub const MAX_INTELFN4_LEVEL: usize = 4;
pub const MAX_INTELFN11_LEVEL: usize = 4;
pub const VENDOR_STR_MAX: usize = 16;
pub const BRAND_STR_MAX: usize = 64;
pub const CPU_FLAGS_MAX: usize = 128;
pub const CPU_HINTS_MAX: usize = 16;

#[repr(C)]
pub struct cpu_raw_data_t {
    pub basic_cpuid: [[uint32_t; 4]; MAX_CPUID_LEVEL],
    pub ext_cpuid: [[uint32_t; 4]; MAX_EXT_CPUID_LEVEL],
    pub intel_fn4: [[uint32_t; 4]; MAX_INTELFN4_LEVEL],
    pub intel_fn11: [[uint32_t; 4]; MAX_INTELFN11_LEVEL],
}

#[repr(C)]
pub struct cpu_id_t {
    pub vendor_str: [c_char; VENDOR_STR_MAX],
    pub brand_str: [c_char; BRAND_STR_MAX],
    pub vendor: int32_t,
    pub flags: [uint8_t; CPU_FLAGS_MAX],
    pub family: int32_t,
    pub model: int32_t,
    pub stepping: int32_t,
    pub ext_family: int32_t,
    pub ext_model: int32_t,
    pub num_cores: int32_t,
    pub num_logical_cpus: int32_t,
    pub total_logical_cpus: int32_t,
    pub l1_data_cache: int32_t,
    pub l1_instruction_cache: int32_t,
    pub l2_cache: int32_t,
    pub l3_cache: int32_t,
    pub l1_assoc: int32_t,
    pub l2_assoc: int32_t,
    pub l3_assoc: int32_t,
    pub l1_cacheline: int32_t,
    pub l2_cacheline: int32_t,
    pub l3_cacheline: int32_t,
    pub cpu_codename: [c_char; 64],
    pub sse_size: int32_t,
    pub detection_hints: [uint8_t; CPU_HINTS_MAX],
}

#[link(name = "cpuid")]
extern {
    pub fn cpuid_present() -> c_int;
    pub fn cpuid_lib_version() -> *const c_char;
    pub fn cpuid_error() -> *const c_char;
    pub fn cpuid_get_raw_data(raw: *mut cpu_raw_data_t) -> c_int;
    pub fn cpu_identify(raw: *mut cpu_raw_data_t, data: *mut cpu_id_t) -> c_int;
    pub fn cpu_clock() -> c_int;
}
