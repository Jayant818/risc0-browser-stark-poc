# RISC-0 Browser STARK PoC

A research Proof-of-Concept for generating RISC-0 STARK proofs entirely within a web browser using WebAssembly.

## Architecture

1.  **Guest (`/guest`):** A minimal Rust program asserting `x^2 = y`. Compiles to `riscv32im` ELF.
2.  **Host (`/host`):** A Rust library exposing the RISC-0 Prover via `wasm-bindgen`. Compiles to `wasm32`.
3.  **Web (`/web`):** A Vite + TypeScript application that loads the WASM Prover and triggers proof generation.

## Prerequisities

- Rust Nightly
- `wasm32-unknown-unknown` target
- `riscv32im-unknown-none-elf` target (for guest)
- Node.js & npm (for web)

## Setup & Build

### 1. Build the Host (Prover)
```bash
cd host
cargo build --release --target wasm32-unknown-unknown
# Note: You need wasm-bindgen-cli to generate the final JS glue
# wasm-bindgen target/wasm32-unknown-unknown/release/browser_prover.wasm --out-dir pkg --target web
```

### 2. Build the Guest
*Note: This often requires the `cargo-risc0` toolchain.*
```bash
cd guest
cargo build --release --target riscv32im-unknown-none-elf
# cp target/riscv32im-unknown-none-elf/release/factors_guest ../web/public/guest.elf
```

### 3. Run Web Interface
```bash
cd web
npm install
npm run start
```

## Status

See `docs/feasibility.md` for detailed analysis.
- **Prover Compilation:** SUCCESS (WASM compatible).
- **Performance:** Expect significant slowdown vs native.
- **UX:** UI will freeze without Web Workers.
