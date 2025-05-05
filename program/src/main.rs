#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    println!("Starting addition program");
    
    // Read two inputs to the program
    let a = sp1_zkvm::io::read::<u32>();
    let b = sp1_zkvm::io::read::<u32>();
    println!("Read inputs: a = {}, b = {}", a, b);

    // Write inputs to public values
    sp1_zkvm::io::commit(&a);
    sp1_zkvm::io::commit(&b);
    println!("Committed inputs to public values");

    // Compute the sum
    let sum = a.wrapping_add(b);
    println!("Computed sum: {} + {} = {}", a, b, sum);

    // Write the output of the program
    sp1_zkvm::io::commit(&sum);
    println!("Committed sum to public values");
    
    println!("Addition program completed successfully");
}