extern crate sigar;

fn main() {
    match sigar::memory() {
        Ok(memory) => println!("Memory: {:?}", memory),
        Err(error) => println!("Error!: {:?}", error),
    };
}
