rust-cpuid
==========

[![Build Status](https://travis-ci.org/zsiciarz/rust-cpuid.svg?branch=master)](https://travis-ci.org/zsiciarz/rust-cpuid)

Rust bindings for [libpcuid](https://github.com/anrieff/libcpuid)
CPU detection and feature extraction library.

Usage
=====

First - download, and build libcpuid as [described in the readme](https://github.com/anrieff/libcpuid). Install it by running `make install` (you may want to run `ldconfig` afterwards).

Add to your `Cargo.toml`:

```toml
[dependencies.cpuid]
git = "https://github.com/zsiciarz/rust-cpuid.git"
```

Add `extern crate cpuid` to your crate root and you're good to go! For example:

```rust
extern crate cpuid;

fn main () {
    match cpuid::identify() {
        Ok(info) => {
            println!("Found: {} CPU, model: {}", info.vendor, info.codename);
            println!("The full brand string is: {}", info.brand);
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
```

Resources
=========

 * [Documentation](http://www.rust-ci.org/zsiciarz/rust-cpuid/doc/cpuid/)
 * [Issue tracker](https://github.com/zsiciarz/rust-cpuid/issues)
 * [CI server](https://travis-ci.org/zsiciarz/rust-cpuid)

Author
======

 * Zbigniew Siciarz (zbigniew at siciarz dot net)

License
=======

This work is released under the MIT license. A copy of the license is provided
in the LICENSE file.
