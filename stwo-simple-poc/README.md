# STWO Simple PoC

This is a research Proof of Concept demonstrating a client-side STARK prover using the [STWO](https://github.com/starkware-libs/stwo) library compiled to WebAssembly.

## What it does
1.  **Proves:** Knowledge of a Fibonacci-like sequence computation (`a_{i+2} = a_{i+1}^2 + a_i^2`) over a small domain.
2.  **Client-Side:** The Prover runs entirely in the browser using WASM.
3.  **Circle STARKs:** Uses the STWO implementation of Circle STARKs over M31 fields.

## Architecture
-   **Rust:** Defines the AIR component, generates the execution trace, and invokes the STWO prover.
-   **WASM:** Rust code is compiled to `wasm32-unknown-unknown` and exposed to JS via `wasm-bindgen`.
-   **Web:** Simple HTML/JS interface to drive the prover.

## Build Instructions

1.  **Prerequisites:**
    -   Rust (Nightly toolchain required for STWO)
    -   `wasm-pack`

2.  **Build WASM:**
    ```bash
    cd stwo-simple-poc
    wasm-pack build --target web
    ```

3.  **Run:**
    Serve the root directory (containing `www` and `pkg`) with a static file server.
    ```bash
    # Example using python
    python3 -m http.server
    # Open http://localhost:8000/www/
    ```

## Disclaimer
This is strictly a research PoC. It uses a small, fixed trace size for demonstration and has not been audited for security.
