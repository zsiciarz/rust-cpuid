use std::default::Default;
use libc::{c_int, c_char, uint8_t, uint32_t, int32_t};

pub static MAX_CPUID_LEVEL: uint = 32u;
pub static MAX_EXT_CPUID_LEVEL: uint = 32u;
pub static MAX_INTELFN4_LEVEL: uint = 4u;
pub static MAX_INTELFN11_LEVEL: uint = 4u;
pub static VENDOR_STR_MAX: uint = 16u;
pub static BRAND_STR_MAX: uint = 64u;
pub static CPU_FLAGS_MAX: uint = 128u;
pub static CPU_HINTS_MAX: uint = 16u;

#[repr(C)]
pub struct cpu_raw_data_t {
    pub basic_cpuid: [[uint32_t, ..4u], ..MAX_CPUID_LEVEL],
    pub ext_cpuid: [[uint32_t, ..4u], ..MAX_EXT_CPUID_LEVEL],
    pub intel_fn4: [[uint32_t, ..4u], ..MAX_INTELFN4_LEVEL],
    pub intel_fn11: [[uint32_t, ..4u], ..MAX_INTELFN11_LEVEL],
}

impl Default for cpu_raw_data_t {
    fn default() -> cpu_raw_data_t {
        cpu_raw_data_t {
            basic_cpuid: [[0, ..4u], ..MAX_CPUID_LEVEL],
            ext_cpuid: [[0, ..4u], ..MAX_EXT_CPUID_LEVEL],
            intel_fn4: [[0, ..4u], ..MAX_INTELFN4_LEVEL],
            intel_fn11: [[0, ..4u], ..MAX_INTELFN11_LEVEL],
        }
    }
}

#[repr(C)]
pub struct cpu_id_t {
    pub vendor_str: [c_char, ..VENDOR_STR_MAX],
    pub brand_str: [c_char, ..BRAND_STR_MAX],
    pub vendor: int32_t,
    pub flags: [uint8_t, ..CPU_FLAGS_MAX],
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
    pub cpu_codename: [c_char, ..64],
    pub sse_size: int32_t,
    pub detection_hints: [uint8_t, ..CPU_HINTS_MAX],
}

impl Default for cpu_id_t {
    fn default() -> cpu_id_t {
        cpu_id_t {
            vendor_str: [0, ..VENDOR_STR_MAX],
            brand_str: [0, ..BRAND_STR_MAX],
            vendor: 0,
            flags: [0u8, ..CPU_FLAGS_MAX],
            family: 0,
            model: 0,
            stepping: 0,
            ext_family: 0,
            ext_model: 0,
            num_cores: 0,
            num_logical_cpus: 0,
            total_logical_cpus: 0,
            l1_data_cache: 0,
            l1_instruction_cache: 0,
            l2_cache: 0,
            l3_cache: 0,
            l1_assoc: 0,
            l2_assoc: 0,
            l3_assoc: 0,
            l1_cacheline: 0,
            l2_cacheline: 0,
            l3_cacheline: 0,
            cpu_codename: [0, ..64],
            sse_size: 0,
            detection_hints: [0u8, ..CPU_HINTS_MAX],
        }
    }
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
