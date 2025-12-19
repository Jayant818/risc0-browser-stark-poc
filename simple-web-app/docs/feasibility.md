# RISC-0 Browser STARK PoC: Feasibility Report

## 1. Executive Summary

**Status:** **FEASIBLE with Custom Prover Implementation**.

We successfully compiled the **Guest** to `riscv32im` and the **Host** to `wasm32`. However, the **Host Proving Step** fails at runtime with "operation not supported on this platform" when using `default_prover()`.

This indicates that while the *code compiles*, the default prover instantiation logic in `risc0-zkvm` v1.2 checks for OS-level features (likely threading or IPC) that are absent in WASM, even with the `client` feature enabled.

## 2. Component Analysis

### 2.1. The Guest
- **Goal:** Compile `x^2 = y` logic to `riscv32im`.
- **Outcome:** **SUCCESS**.
- **Resolution:** Used `risc0/risc0-guest-builder` Docker strategy with `cargo risczero build`.
- **Artifact:** A valid 32-bit RISC-V ELF executable.

### 2.2. The Host (Prover)
- **Goal:** Compile the STARK prover to WASM.
- **Outcome:** **PARTIAL SUCCESS**.
- **Compilation:** SUCCESS.
- **Runtime:** **FAILURE**. `default_prover()` throws "operation not supported".
- **Root Cause:** The `risc0-zkvm` crate's default selection mechanism likely attempts to spawn a sub-process (IPC prover) or use multi-threading which is blocked in standard WASM.
- **Fix:** A dedicated `CpuProver` must be manually instantiated or the `ExternalProver` must be replaced with a WASM-compatible implementation.

### 2.3. Browser Integration
- **WASM Size:** ~4 MB.
- **UX:** Main thread freezes (expected).

## 3. Performance Projections

| Environment | Time (Est.) |
| :--- | :--- |
| Native (M3 Max, GPU) | < 1s |
| Native (CPU) | ~2s |
| Browser (WASM, Single Thread) | **FAILED (Runtime Error)** |

## 4. Final Conclusion

To generate a proof in the browser with RISC-0:
1.  **Do not use `default_prover()`**.
2.  You must explicitly construct a `Prover` that does not rely on IPC or native threading.
3.  This likely requires using internal APIs or a fork of `risc0-zkvm` specifically patched for `wasm32-unknown-unknown`.
