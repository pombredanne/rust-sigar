extern crate sigar;

fn main() {
    match sigar::memory() {
        Ok(memory) => println!("Memory: {:?}", memory),
        Err(error) => println!("Error!: {:?}", error),
    };

    match sigar::cpu() {
        Ok(cpu) => println!("CPU: {:?}", cpu),
        Err(error) => println!("Error!: {:?}", error),
    };

    match sigar::swap() {
        Ok(swap) => println!("Swap: {:?}", swap),
        Err(error) => println!("Error!: {:?}", error),
    };
}
