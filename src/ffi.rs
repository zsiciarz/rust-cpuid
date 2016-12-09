use std::default::Default;
use libc::{c_int, c_char, uint8_t, uint32_t, int32_t, uint64_t};

pub const VENDOR_STR_MAX: usize = 16;
pub const BRAND_STR_MAX: usize = 64;
pub const CPU_FLAGS_MAX: usize = 128;
pub const MAX_CPUID_LEVEL: usize = 32;
pub const MAX_EXT_CPUID_LEVEL: usize = 32;
pub const MAX_INTELFN4_LEVEL: usize = 8;
pub const MAX_INTELFN11_LEVEL: usize = 4;
pub const MAX_INTELFN12H_LEVEL: usize = 4;
pub const MAX_INTELFN14H_LEVEL: usize = 4;
pub const CPU_HINTS_MAX: usize = 16;
pub const SGX_FLAGS_MAX: usize = 14;

#[repr(C)]
pub struct cpu_raw_data_t {
    pub basic_cpuid: [[uint32_t; 4]; MAX_CPUID_LEVEL],
    pub ext_cpuid: [[uint32_t; 4]; MAX_EXT_CPUID_LEVEL],
    pub intel_fn4: [[uint32_t; 4]; MAX_INTELFN4_LEVEL],
    pub intel_fn11: [[uint32_t; 4]; MAX_INTELFN11_LEVEL],
    pub intel_fn12h: [[uint32_t; 4]; MAX_INTELFN12H_LEVEL],
    pub intel_fn14h: [[uint32_t; 4]; MAX_INTELFN14H_LEVEL],
}

impl Default for cpu_raw_data_t {
    fn default() -> cpu_raw_data_t {
        cpu_raw_data_t {
            basic_cpuid: [[0; 4]; MAX_CPUID_LEVEL],
            ext_cpuid: [[0; 4]; MAX_EXT_CPUID_LEVEL],
            intel_fn4: [[0; 4]; MAX_INTELFN4_LEVEL],
            intel_fn11: [[0; 4]; MAX_INTELFN11_LEVEL],
            intel_fn12h: [[0; 4]; MAX_INTELFN12H_LEVEL],
            intel_fn14h: [[0; 4]; MAX_INTELFN14H_LEVEL],
        }
    }
}

#[repr(C)]
pub struct cpu_sgx_t {
    pub present: uint32_t,
    pub max_enclave_32bit: uint8_t,
    pub max_enclave_64bit: uint8_t,
    pub flags: [uint8_t; SGX_FLAGS_MAX],
    pub num_epc_sections: c_int,
    pub misc_select: uint32_t,
    pub secs_attributes: uint64_t,
    pub secs_xfrm: uint64_t,
}

impl Default for cpu_sgx_t {
    fn default() -> cpu_sgx_t {
        cpu_sgx_t {
            present: 0u32,
            max_enclave_32bit: 0u8,
            max_enclave_64bit: 0u8,
            flags: [0u8; SGX_FLAGS_MAX],
            num_epc_sections: 0i32,
            misc_select: 0u32,
            secs_attributes: 0u64,
            secs_xfrm: 0u64,
        }
    }
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
    pub l4_cache: int32_t,
    pub l1_assoc: int32_t,
    pub l2_assoc: int32_t,
    pub l3_assoc: int32_t,
    pub l4_assoc: int32_t,
    pub l1_cacheline: int32_t,
    pub l2_cacheline: int32_t,
    pub l3_cacheline: int32_t,
    pub l4_cacheline: int32_t,
    pub cpu_codename: [c_char; 64],
    pub sse_size: int32_t,
    pub detection_hints: [uint8_t; CPU_HINTS_MAX],
    pub sgx: cpu_sgx_t,
}

impl Default for cpu_id_t {
    fn default() -> cpu_id_t {
        cpu_id_t {
            vendor_str: [0; VENDOR_STR_MAX],
            brand_str: [0; BRAND_STR_MAX],
            vendor: 0,
            flags: [0u8; CPU_FLAGS_MAX],
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
            l4_cache: 0,
            l1_assoc: 0,
            l2_assoc: 0,
            l3_assoc: 0,
            l4_assoc: 0,
            l1_cacheline: 0,
            l2_cacheline: 0,
            l3_cacheline: 0,
            l4_cacheline: 0,
            cpu_codename: [0; 64],
            sse_size: 0,
            detection_hints: [0u8; CPU_HINTS_MAX],
            sgx: Default::default(),
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
