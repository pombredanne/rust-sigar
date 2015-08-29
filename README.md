# rust-sigar

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
