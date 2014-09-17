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

#[test]
fn test_is_present() {
    assert!(is_present());
}
