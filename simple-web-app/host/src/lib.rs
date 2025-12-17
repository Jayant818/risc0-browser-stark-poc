use wasm_bindgen::prelude::*;
use risc0_zkvm::{default_prover, ExecutorEnv};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn init_logging() {
    console_error_panic_hook::set_once();
    log("Logging initialized");
}

#[wasm_bindgen]
pub fn prove_square(guest_elf: &[u8], x: u32) -> Result<Vec<u8>, JsError> {
    log("Step 1: Building ExecutorEnv");
    let mut builder = ExecutorEnv::builder();
    builder.write(&x)
        .map_err(|e| JsError::new(&format!("Failed to write input: {}", e)))?;
        
    let env = builder.build()
        .map_err(|e| JsError::new(&format!("Failed to build env: {}", e)))?;

    log("Step 2: Obtaining Prover (default_prover)");
    let prover = default_prover();

    log("Step 3: Starting Proof Generation (this may take a while)");
    let prove_info = prover
        .prove(env, guest_elf)
        .map_err(|e| JsError::new(&format!("Proving failed: {}", e)))?;

    log("Step 4: Proof Complete. Serializing...");
    let receipt = prove_info.receipt;
    let receipt_bytes = bincode::serialize(&receipt)
        .map_err(|e| JsError::new(&format!("Serialization failed: {}", e)))?;

    log("Step 5: Done");
    Ok(receipt_bytes)
}
