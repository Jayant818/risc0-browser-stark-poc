import init, { prove_square, init_logging } from './pkg/browser_prover.js';

async function run() {
    const status = document.getElementById('status')!;
    const btn = document.getElementById('proveBtn') as HTMLButtonElement;
    const output = document.getElementById('output')!;

    try {
        status.textContent = 'Initializing WASM...';
        await init();
        init_logging();
        
        status.textContent = 'WASM Ready. Fetching Guest ELF...';
        
        // Fetch the guest ELF (must be served by Vite)
        // We will assume the user manually places the built ELF in web/public/guest.elf
        // or we configure the build to do so.
        let elf: Uint8Array;
        try {
            const response = await fetch('/guest.elf');
            if (!response.ok) throw new Error(`HTTP ${response.status} ${response.statusText}`);
            const buffer = await response.arrayBuffer();
            elf = new Uint8Array(buffer);
            status.textContent = `Guest ELF loaded (${elf.length} bytes). Ready to prove.`;
        } catch (e) {
            status.innerHTML = `<span class="error">Failed to load guest.elf. Please ensure it is in the public folder.<br>Error: ${e}</span>`;
            return;
        }

        btn.disabled = false;
        
        btn.onclick = async () => {
            btn.disabled = true;
            output.textContent = '';
            status.textContent = 'Proving... (Main thread will freeze)';
            
            // Allow UI to update before freezing
            await new Promise(r => setTimeout(r, 100));

            const start = performance.now();
            try {
                // Prove x = 17 (y = 289)
                const x = 17;
                
                // Invoke WASM Prover
                const receipt = prove_square(elf, x); 
                
                const end = performance.now();
                const duration = (end - start).toFixed(2);
                
                status.innerHTML = `<span class="success">Proof Generated in ${duration}ms!</span>`;
                output.textContent = `Receipt Size: ${receipt.length} bytes\n\nRaw Data Preview:\n${Array.from(receipt).slice(0, 64).map(b => b.toString(16).padStart(2, '0')).join(' ')}...`;
                
            } catch (e) {
                console.error(e);
                status.innerHTML = `<span class="error">Proving Failed: ${e}</span>`;
            } finally {
                btn.disabled = false;
            }
        };

    } catch (e) {
        status.innerHTML = `<span class="error">WASM Initialization Error: ${e}</span>`;
    }
}

run();
