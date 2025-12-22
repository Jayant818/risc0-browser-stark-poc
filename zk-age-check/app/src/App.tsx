import { useState, useEffect } from 'react';
import init, { prove_age_valid } from "../../prover/pkg/zk_age_check_prover";
import './App.css';

function App() {
  const [birthYear, setBirthYear] = useState<string>('');
  const [status, setStatus] = useState<'idle' | 'proving' | 'verified' | 'failed'>('idle');
  const [message, setMessage] = useState<string>('');
  const [proof, setProof] = useState<string>('');
  const [wasmReady, setWasmReady] = useState(false);

  useEffect(() => {
    init().then(() => {
      setWasmReady(true);
    }).catch(err => {
      console.error("Failed to load WASM", err);
      setMessage("Failed to load ZK Prover engine.");
    });
  }, []);

  const handleVerify = async () => {
    if (!wasmReady) return;
    
    const year = parseInt(birthYear);
    const currentYear = 2025;
    const age = currentYear - year;

    if (isNaN(year) || year < 1875 || year > currentYear) {
      setStatus('failed');
      setMessage("Please enter a valid birth year.");
      return;
    }

    if (age < 18) {
      setStatus('failed');
      setMessage("Access Denied: You must be 18 or older.");
      return;
    }

    setStatus('proving');
    setMessage("Generating Zero-Knowledge Proof...");

    // Simulate a small delay for UI feedback
    setTimeout(() => {
      try {
        const result = prove_age_valid(year);
        if (result.success) {
          setStatus('verified');
          setMessage(result.message);
          setProof(result.proof_str);
        } else {
          setStatus('failed');
          setMessage(result.message);
        }
      } catch (err) {
        console.error(err);
        setStatus('failed');
        setMessage("An error occurred during proof generation.");
      }
    }, 500);
  };

  return (
    <div className="container">
      <div className="card">
        <h1>ZK Age Verifier</h1>
        
        {status === 'idle' && (
          <div className="flow">
            <p>Verify your age to access restricted content.</p>
            <div className="input-group">
              <label htmlFor="birthYear">Enter Birth Year:</label>
              <input 
                type="number" 
                id="birthYear"
                value={birthYear}
                onChange={(e) => setBirthYear(e.target.value)}
                placeholder="e.g. 1995"
              />
            </div>
            <button onClick={handleVerify} disabled={!wasmReady}>
              {wasmReady ? "Verify Age (ZK Proof)" : "Loading Prover..."}
            </button>
          </div>
        )}

        {status === 'proving' && (
          <div className="flow">
            <div className="loader"></div>
            <p>{message}</p>
            <p className="hint">This stays on your device. We never see your birth year.</p>
          </div>
        )}

        {status === 'verified' && (
          <div className="flow success">
            <div className="icon">✅</div>
            <h2>18+ Verified</h2>
            <p>{message}</p>
            <p className="hint">(Year not revealed to the server)</p>
            <div className="proof-container">
              <details>
                <summary>View ZK Proof Details</summary>
                <pre>{proof}</pre>
              </details>
            </div>
            <button onClick={() => { setStatus('idle'); setBirthYear(''); }}>Start Over</button>
          </div>
        )}

        {status === 'failed' && (
          <div className="flow error">
            <div className="icon">❌</div>
            <h2>Verification Failed</h2>
            <p>{message}</p>
            <button onClick={() => setStatus('idle')}>Try Again</button>
          </div>
        )}
      </div>
    </div>
  );
}

export default App;