# ZK Age Check POC

A Zero-Knowledge Proof (ZKP) Proof of Concept that allows a user to verify they are **18 or older** without revealing their exact birth year.

This project runs entirely in the browser (Client-Side Proving) using Rust compiled to WebAssembly (WASM).

## 🎯 Goal

Prove: $18 \le (CurrentYear - BirthYear) \le 150$

**Privacy Guarantee:** The Verifier (and the server) only sees a mathematical proof that the condition holds. The `BirthYear` input remains private on the client device.

## 🛠 Tech Stack

*   **ZKP Backend:** Rust 🦀 using the [Stwo](https://github.com/starkware-libs/stwo) library (STARK-based).
*   **Compilation:** `wasm-pack` (Rust $\to$ WASM).
*   **Frontend:** React (TypeScript) + Vite.
*   **Fields:** M31 (Mersenne-31) Finite Fields.

## 🧮 How It Works (The Math)

Since standard comparison operators ($<, \ge$) don't work directly in Finite Fields due to modular arithmetic wrapping, we use the **Polynomial Root Method** for this POC.

### The Constraint
We define a polynomial constraint that is satisfied *only* if the calculated Age is one of the valid values {18, 19, ..., 150}.

$$ C(x) = (x - 18)(x - 19)\dots(x - 150) = 0 $$

If $C(Age) == 0$, then Age must be in the valid range.

### The Flow
1.  **User Input:** Enters Birth Year (e.g., 2000).
2.  **Witness Generation:** Client calculates `Age = 2025 - 2000 = 25`.
3.  **Proving (WASM):**
    *   The `Age` is committed to a trace column.
    *   The constraint $C(Age) = 0$ is applied.
    *   A STARK proof is generated.
4.  **Verification:** The proof is verified (locally in this demo, but typically sent to a server/verifier).
5.  **Output:** A Green "Verified" badge if the proof holds.

## 📂 Project Structure

```text
zk-age-check/
├── app/                  # Frontend (React + Vite)
│   ├── src/              # UI Components
│   └── vite.config.ts    # Configured for WASM & File Access
├── prover/               # Backend (Rust + Stwo)
│   ├── src/lib.rs        # ZK Logic & WASM Bindings
│   ├── Cargo.toml        # Dependencies
│   └── rust-toolchain.toml # Nightly Rust config
```

## 🚀 Setup & Running

### Prerequisites
*   **Rust (Nightly):** `rustup toolchain install nightly-2025-07-14`
*   **Wasm-Pack:** `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`
*   **Node.js & NPM**

### 1. Build the Prover (WASM)
Compile the Rust ZK logic into a WASM package that the browser can load.

```bash
cd prover
wasm-pack build --target web
```
*Note: This creates a `pkg/` directory containing the .wasm binary and JS glue code.*

### 2. Run the Frontend
Install dependencies and start the development server.

```bash
cd ../app
npm install
npm run dev
```

### 3. Usage
1.  Open the local URL (usually `http://localhost:5174`).
2.  Enter a birth year.
    *   **Try 2000:** (Age 25) -> ✅ Verified.
    *   **Try 2015:** (Age 10) -> ❌ Blocked (Client-side pre-check).
    *   **Try invalid ranges:** -> ❌ Blocked.
3.  Click "Verify Age".
4.  Expand "View ZK Proof Details" to see the raw STARK proof string.

## 🔧 Troubleshooting

### "Not enough twiddles!"
If you encounter this panic in the browser console, it relates to the FFT domain precomputation.
*   **Cause:** The high degree of our constraint polynomial (deg 133) combined with SIMD packing in the WASM backend requires a larger precomputed twiddle factor buffer than standard examples.
*   **Fix:** We increased the domain size in `prover/src/lib.rs`:
    ```rust
    // Increased offset from +1 to +9 to cover SIMD packing overhead
    CanonicCoset::new(log_n_rows + 9 + config.fri_config.log_blowup_factor)
    ```

### 403 Forbidden (WASM Load)
*   **Cause:** Vite blocks access to files outside the project root by default.
*   **Fix:** We configured `vite.config.ts` to allow serving from `..`:
    ```typescript
    server: { fs: { allow: ['..'] } }
    ```
