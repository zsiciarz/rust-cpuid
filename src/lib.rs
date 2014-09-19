extern crate libc;

use std::c_str::CString;
use std::default::Default;

mod ffi;

#[deriving(Show)]
pub struct CpuInfo {
    pub vendor: String,
    pub brand: String,
    pub codename: String,
    pub num_cores: int,
    pub num_logical_cpus: int,
    pub total_logical_cpus: int,
    pub l1_data_cache: Option<int>,
    pub l1_instruction_cache: Option<int>,
    pub l2_cache: Option<int>,
    pub l3_cache: Option<int>,
}

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

pub fn identify() -> Result<CpuInfo, String> {
    let mut raw: ffi::cpu_raw_data_t = Default::default();
    let raw_result = unsafe {
        ffi::cpuid_get_raw_data(&mut raw)
    };
    if raw_result != 0 {
        return Err(error());
    }
    let mut data: ffi::cpu_id_t = Default::default();
    let identify_result = unsafe {
        ffi::cpu_identify(&mut raw, &mut data)
    };
    if identify_result != 0 {
        Err(error())
    } else {
        Ok(CpuInfo {
            vendor: String::from_utf8(data.vendor_str.iter().map(|&x| x as u8).collect()).ok().expect("Invalid vendor string"),
            brand: String::from_utf8(data.brand_str.iter().map(|&x| x as u8).collect()).ok().expect("Invalid brand string"),
            codename: String::from_utf8(data.cpu_codename.iter().map(|&x| x as u8).collect()).ok().expect("Invalid codename string"),
            num_cores: data.num_cores as int,
            num_logical_cpus: data.num_logical_cpus as int,
            total_logical_cpus: data.total_logical_cpus as int,
            l1_data_cache: if data.l1_data_cache != -1 { Some(data.l1_data_cache as int) } else { None },
            l1_instruction_cache: if data.l1_instruction_cache != -1 { Some(data.l1_instruction_cache as int) } else { None },
            l2_cache: if data.l2_cache != -1 { Some(data.l2_cache as int) } else { None },
            l3_cache: if data.l3_cache != -1 { Some(data.l3_cache as int) } else { None },
        })
    }
}

#[test]
fn test_is_present() {
    assert!(is_present());
}
