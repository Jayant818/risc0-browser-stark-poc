import { useState, useEffect } from 'react'
import init, { prove_in_browser } from 'prover-wasm'
import './App.css'

function App() {
  const [jsonInput, setJsonInput] = useState('')
  const [status, setStatus] = useState('Loading WASM...')
  const [proof, setProof] = useState('')

  useEffect(() => {
    init().then(() => {
      setStatus('WASM Ready')
    }).catch(e => {
      setStatus(`WASM Error: ${e}`)
    })
  }, [])

  const handleProve = () => {
    setStatus('Proving...')
    // Small delay to allow UI to update if synchronous
    setTimeout(() => {
      try {
        const result = prove_in_browser(jsonInput)
        setProof(JSON.stringify(JSON.parse(result), null, 2))
        setStatus('Proof Generated!')
      } catch (e) {
        setStatus(`Error: ${e}`)
      }
    }, 100)
  }

  return (
    <div className="card">
      <h1>Cairo Prover (In-Browser POC)</h1>
      <p>Status: {status}</p>
      <textarea 
        value={jsonInput} 
        onChange={e => setJsonInput(e.target.value)}
        placeholder="Paste compiled Cairo JSON here..."
        style={{ width: '100%', height: '200px' }}
      />
      <br />
      <button onClick={handleProve} disabled={status !== 'WASM Ready'}>
        Prove
      </button>
      {proof && (
        <div style={{ textAlign: 'left', marginTop: '20px' }}>
          <h3>Proof Output:</h3>
          <pre style={{ background: '#f4f4f4', padding: '10px', overflow: 'auto' }}>{proof}</pre>
        </div>
      )}
    </div>
  )
}

export default App