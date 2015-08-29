# rust-sigar [![Build Status](https://travis-ci.org/xoebus/rust-sigar.svg?branch=master)](https://travis-ci.org/xoebus/rust-sigar) [![](https://meritbadge.herokuapp.com/sigar)](https://crates.io/crates/sigar)

Rust bindings for [libsigar][libsigar] for portably gathering system information.

[libsigar]: https://github.com/hyperic/sigar

## usage

1. Build and install `libsigar`.
2. Use this library.

``` rust
fn main() {
    match sigar::memory() {
        Ok(memory) => println!("Memory: {:?}", memory),
        Err(error) => println!("Error!: {:?}", error),
    };
}
```
