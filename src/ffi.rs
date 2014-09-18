use libc::{c_int, c_char, uint32_t};

pub static MAX_CPUID_LEVEL: uint = 32u;
pub static MAX_EXT_CPUID_LEVEL: uint = 32u;
pub static MAX_INTELFN4_LEVEL: uint = 4u;
pub static MAX_INTELFN11_LEVEL: uint = 4u;

#[repr(C)]
pub struct cpu_raw_data_t {
    pub basic_cpuid: [[uint32_t, ..MAX_CPUID_LEVEL], ..4u],
    pub ext_cpuid: [[uint32_t, ..MAX_EXT_CPUID_LEVEL], ..4u],
    pub intel_fn4: [[uint32_t, ..MAX_INTELFN4_LEVEL], ..4u],
    pub intel_fn11: [[uint32_t, ..MAX_INTELFN11_LEVEL], ..4u],
}

// TODO: definition for struct cpu_id_t

#[link(name = "cpuid")]
extern {
    pub fn cpuid_present() -> c_int;
    pub fn cpuid_lib_version() -> *const c_char;
    pub fn cpuid_error() -> *const c_char;
    pub fn cpuid_get_raw_data(raw: *mut cpu_raw_data_t) -> c_int;
    // TODO: add cpu_identify
}
