use sp1_core::{utils, SP1Prover, SP1Stdin, SP1Verifier};
use serde::{Serialize, Deserialize};

use std::env;

// The ELF to execute
// The ELF should be compiled first using the following command: `cargo prove build`
// The code for the ELF is in `check-adult/src/main.rs`
const ELF: &[u8] = include_bytes!("../../check-adult/elf/riscv32im-succinct-zkvm-elf");

#[derive(Debug, Serialize, Deserialize)]
struct Result {
    is_adult: bool
}
fn main() {
    utils::setup_tracer();

    let args = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        panic!("Usage: {} <dob_Y-m-d>", args[0]);
    }

    let today = chrono::Local::now();
    println!("Input: {}", args[1]);
    println!("Today: {}", today.format("%Y-%m-%d"));

    let mut stdin = SP1Stdin::new();
    let dob_and_today = format!("{}|{}", args[1], today.format("%Y-%m-%d"));

    // Write the input to the ELF
    stdin.write(&dob_and_today);
    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // Read the output from the ELF
    let result = proof.stdout.read::<Result>();
    println!("Result: {:#?}", result);

    // Verify the proof
    SP1Verifier::verify(ELF, &proof).expect("Verification failed");
}
