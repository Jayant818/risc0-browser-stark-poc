import init, { init_prover, prove_fib } from '../../pkg/stwo_simple_poc';

let isInitialized = false;

self.onmessage = async (e: MessageEvent) => {
    const { type, payload } = e.data;

    try {
        if (type === 'INIT') {
            if (!isInitialized) {
                await init();
                init_prover();
                isInitialized = true;
                self.postMessage({ type: 'STATUS', payload: 'READY' });
            }
            return;
        }

        if (type === 'PROVE') {
            if (!isInitialized) throw new Error('Worker not initialized');
            
            // Generate proof
            const resultJson = prove_fib(payload);
            self.postMessage({ type: 'PROOF_COMPLETE', payload: resultJson });
        }
    } catch (err: any) {
        self.postMessage({ type: 'ERROR', payload: err.toString() });
    }
};
