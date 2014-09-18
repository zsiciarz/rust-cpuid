extern crate libc;

use std::c_str::CString;

mod ffi;

pub fn is_present() -> bool {
    unsafe {
        ffi::cpuid_present() == 1
    }
}

pub fn version() -> String {
    let version_string = unsafe {
        let ptr = ffi::cpuid_lib_version();
        CString::new(ptr, false)
    };
    version_string.as_str().unwrap().to_string()
}

pub fn error() -> String {
    let error_string = unsafe {
        let ptr = ffi::cpuid_error();
        CString::new(ptr, false)
    };
    error_string.as_str().unwrap().to_string()
}

pub fn identify() -> bool {
    let mut raw = ffi::cpu_raw_data_t {
        basic_cpuid: [[0, ..ffi::MAX_CPUID_LEVEL], ..4u],
        ext_cpuid: [[0, ..ffi::MAX_EXT_CPUID_LEVEL], ..4u],
        intel_fn4: [[0, ..ffi::MAX_INTELFN4_LEVEL], ..4u],
        intel_fn11: [[0, ..ffi::MAX_INTELFN11_LEVEL], ..4u],
    };
    let result = unsafe {
        ffi::cpuid_get_raw_data(&mut raw)
    };
    // TODO: use raw in the call to cpu_identify
    result == 0
}

#[test]
fn test_is_present() {
    assert!(is_present());
}
