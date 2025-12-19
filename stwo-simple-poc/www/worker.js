import init, { init_prover, prove_fib } from '../pkg/stwo_simple_poc.js';

let isInitialized = false;

self.onmessage = async (e) => {
    const { type, payload } = e.data;

    if (type === 'INIT') {
        if (!isInitialized) {
            await init();
            init_prover(); // Precompute twiddles
            isInitialized = true;
            self.postMessage({ type: 'STATUS', payload: 'READY' });
        }
        return;
    }

    if (type === 'PROVE') {
        if (!isInitialized) {
            self.postMessage({ type: 'ERROR', payload: 'Worker not initialized' });
            return;
        }
        try {
            const seed = payload;
            // prove_fib is synchronous in Rust, but running in worker makes it async to UI
            const resultJson = prove_fib(seed);
            self.postMessage({ type: 'PROOF_COMPLETE', payload: resultJson });
        } catch (err) {
            self.postMessage({ type: 'ERROR', payload: err.toString() });
        }
    }
};