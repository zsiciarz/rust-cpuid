extern crate libc;

mod ffi;

pub fn is_present() -> bool {
    unsafe {
        ffi::cpuid_present() == 1
    }
}

#[test]
fn test_is_present() {
    assert!(is_present());
}
