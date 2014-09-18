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
    let raw_result = unsafe {
        ffi::cpuid_get_raw_data(&mut raw)
    };
    // TODO: use raw in the call to cpu_identify
    let mut data = ffi::cpu_id_t {
        vendor_str: [0, ..ffi::VENDOR_STR_MAX],
        brand_str: [0, ..ffi::BRAND_STR_MAX],
        vendor: 0,
        flags: [0u8, ..ffi::CPU_FLAGS_MAX],
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
        detection_hints: [0u8, ..ffi::CPU_HINTS_MAX],
    };
    let identify_result = unsafe {
        ffi::cpu_identify(&mut raw, &mut data)
    };
    // TODO: convert data to a more friendly representation and return
    identify_result == 0
}

#[test]
fn test_is_present() {
    assert!(is_present());
}
