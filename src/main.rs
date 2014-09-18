extern crate cpuid;

#[cfg(not(test))]
fn main () {
    println!("cpuid is present: {}", cpuid::is_present());
    println!("cpuid version: {}", cpuid::version());
    let info = cpuid::identify();
    match info {
        Err(err) => println!("cpuid error: {}", err),
        Ok(info) => {
            println!("Found: {} CPU", info.vendor);
            println!("Processor model is: {}", info.codename);
            println!("The full brand string is: {}", info.brand);
            println!("The processor has {} cores and {} logical processors", info.num_cores, info.num_logical_cpus);
        }
    }
}
