extern crate cpuid;

#[cfg(not(test))]
fn main () {
    println!("cpuid is present: {}", cpuid::is_present());
    println!("cpuid version: {}", cpuid::version());
    println!("cpuid error: {}", cpuid::error());
}
