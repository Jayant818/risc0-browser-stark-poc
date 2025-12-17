use risc0_zkvm::guest::env;

fn main() {
    // Read private input x (u32)
    let x: u32 = env::read();

    // Calculate x^2
    let y = x.checked_mul(x).expect("Overflow");

    // Commit y as public output (journal)
    env::commit(&y);
}
