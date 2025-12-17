use wasm_bindgen::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct MockProof {
    pub proof_hex: String,
    pub status: String,
    pub execution_trace_length: u32,
}

#[wasm_bindgen]
pub fn prove_in_browser(program_json: &str) -> Result<String, JsValue> {
    console_error_panic_hook::set_once();

    // Log input size to show we received it
    let input_len = program_json.len();
    web_sys::console::log_1(&format!("Received program of size: {} bytes", input_len).into());

    // Simulate proving time (optional, but realistic)
    // WASM is synchronous usually unless async is used. 
    // We'll just return immediately for responsiveness in this synchronous export.

    // Create a mock proof
    let proof = MockProof {
        proof_hex: "0x123456789abcdef...mock_proof...".to_string(),
        status: "success".to_string(),
        execution_trace_length: 42,
    };

    serde_json::to_string(&proof)
        .map_err(|e| JsValue::from_str(&format!("Serialization failed: {}", e)))
}