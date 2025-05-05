use alloy_sol_types::sol;

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        uint32 a;
        uint32 b;
        uint32 sum;
    }
}

/// Compute the sum of two numbers (wrapping around on overflows), using normal Rust code.
pub fn add(a: u32, b: u32) -> u32 {
    a.wrapping_add(b)
}

/// Test function to verify the addition is correct
pub fn test_addition(a: u32, b: u32) -> bool {
    let sum = add(a, b);
    sum == a.wrapping_add(b)
}