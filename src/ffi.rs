use libc::c_int;

#[link(name = "cpuid")]
extern {
    pub fn cpuid_present() -> c_int;
}
