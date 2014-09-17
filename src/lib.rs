extern crate libc;

use libc::c_int;

#[link(name = "cpuid")]
extern {
    fn cpuid_present() -> c_int;
}

pub fn is_present() -> bool {
    unsafe {
        cpuid_present() == 1
    }
}

#[test]
fn test_is_present() {
    assert!(is_present());
}
