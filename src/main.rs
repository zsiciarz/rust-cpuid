extern crate cpuid;

#[cfg(not(test))]
fn main () {
    println!("cpuid is present: {}", cpuid::is_present());
    println!("cpuid version: {}", cpuid::version());
    match cpuid::identify() {
        Ok(info) => {
            println!("Found: {} CPU, model: {}", info.vendor, info.codename);
            println!("The full brand string is: {}", info.brand);
            println!("The processor has {} cores and {} logical processors", info.num_cores, info.num_logical_cpus);
            if info.has_feature(cpuid::FloatingPointUnit) {
                println!("Floating point unit available");
            }
        },
        Err(err) => println!("cpuid error: {}", err),
    };
    match cpuid::clock_frequency() {
        Some(frequency) => println!("CPU speed: {} MHz", frequency),
        None => println!("Couldn't get CPU speed."),
    };
}
