use libc::{c_int, c_char};

#[link(name = "cpuid")]
extern {
    pub fn cpuid_present() -> c_int;
    pub fn cpuid_lib_version() -> *const c_char;
    pub fn cpuid_error() -> *const c_char;
}
