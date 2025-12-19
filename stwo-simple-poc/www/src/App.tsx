import { useEffect, useState, useRef } from 'react';
import init, { verify_fib } from '../../pkg/stwo_simple_poc';
import './App.css';

// Import worker constructor directly (Vite specific)
import ProverWorker from './worker?worker';

interface ProofResult {
  proof_bytes: number[];
  claim_last_val: number;
  time_ms: number;
}

function App() {
  const [status, setStatus] = useState('Initializing...');
  const [seed, setSeed] = useState(12345);
  const [proof, setProof] = useState<string | null>(null);
  const [verification, setVerification] = useState<string>('');
  const [isProving, setIsProving] = useState(false);
  const workerRef = useRef<Worker | null>(null);

  useEffect(() => {
    // 1. Init main thread WASM (for verify)
    init().catch(console.error);

    // 2. Init Worker (for prove)
    workerRef.current = new ProverWorker();
    workerRef.current.onmessage = (e) => {
      const { type, payload } = e.data;
      if (type === 'STATUS' && payload === 'READY') setStatus('Ready');
      if (type === 'PROOF_COMPLETE') {
        setProof(payload);
        const parsed = JSON.parse(payload) as ProofResult;
        setStatus(`Proof generated in ${parsed.time_ms.toFixed(2)}ms`);
        setIsProving(false);
      }
      if (type === 'ERROR') {
        setStatus(`Error: ${payload}`);
        setIsProving(false);
      }
    };
    workerRef.current.postMessage({ type: 'INIT' });

    return () => workerRef.current?.terminate();
  }, []);

  const handleProve = () => {
    setIsProving(true);
    setStatus('Proving... (Calculating Trace & STARK)');
    setVerification('');
    workerRef.current?.postMessage({ type: 'PROVE', payload: seed });
  };

  const handleVerify = () => {
    if (!proof) return;
    try {
      const isValid = verify_fib(proof);
      setVerification(isValid ? '✅ VERIFICATION PASSED' : '❌ VERIFICATION FAILED');
    } catch (e) {
      setVerification(`Error: ${e}`);
    }
  };

  return (
    <div className="card">
      <h1>STWO Browser Prover (React)</h1>
      
      <div className="section">
        <label>Seed Input: </label>
        <input 
          type="number" 
          value={seed} 
          onChange={(e) => setSeed(parseInt(e.target.value))} 
        />
        <button onClick={handleProve} disabled={isProving || status !== 'Ready'}>
          {isProving ? 'Proving...' : 'Generate Proof'}
        </button>
        <p className="status">{status}</p>
      </div>

      {proof && (
        <div className="section">
          <h3>Proof Result</h3>
          <button onClick={handleVerify}>Verify Proof Locally</button>
          <p className="verify-result">{verification}</p>
          <pre>{JSON.stringify(JSON.parse(proof), null, 2)}</pre>
        </div>
      )}
    </div>
  );
}

export default App;