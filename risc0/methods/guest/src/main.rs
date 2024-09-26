use risc0_zkvm::guest::env;
use fuel_vm_lib::fibonacci;

fn main() {
    // read the input
    let input: u32 = env::read();
    let (a, b) = fibonacci(input);
    env::commit(&a);
}


