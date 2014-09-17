use libc::{c_int, c_char};

pub static MAX_CPUID_LEVEL: uint = 32u;
pub static MAX_EXT_CPUID_LEVEL: uint = 32u;
pub static MAX_INTELFN4_LEVEL: uint = 4u;
pub static MAX_INTELFN11_LEVEL: uint = 4u;

#[link(name = "cpuid")]
extern {
    pub fn cpuid_present() -> c_int;
    pub fn cpuid_lib_version() -> *const c_char;
    pub fn cpuid_error() -> *const c_char;
}
