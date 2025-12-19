#![feature(allocator_api)] 

use wasm_bindgen::prelude::*;
use std::sync::OnceLock;
use serde::{Serialize, Deserialize};
use itertools::Itertools;
use stwo::core::fields::m31::BaseField;
use stwo::core::fields::FieldExpOps;
use stwo::core::poly::circle::CanonicCoset;
use stwo::core::ColumnVec;
// Use CpuBackend
use stwo::prover::backend::cpu::CpuBackend;
use stwo::prover::poly::circle::{CircleEvaluation, PolyOps};
use stwo::prover::poly::twiddles::TwiddleTree;
use stwo::prover::poly::BitReversedOrder;
use stwo_constraint_framework::{EvalAtRow, FrameworkComponent, FrameworkEval, TraceLocationAllocator};
use stwo::core::pcs::{CommitmentSchemeVerifier, PcsConfig};
use stwo::core::channel::Blake2sM31Channel;
use stwo::core::vcs::blake2_merkle::Blake2sM31MerkleChannel;
use stwo::core::channel::MerkleChannel; 
use stwo::prover::{prove, CommitmentSchemeProver, ComponentProver};
use stwo::core::verifier::verify;
use stwo::core::fields::qm31::SecureField;
use stwo::core::air::Component;
use stwo::core::proof::StarkProof;
use num_traits::{One, Zero};

// Define constants
const LOG_N_ROWS: u32 = 5; // 32 rows
const FIB_LEN: usize = 16;  // Small sequence

static TWIDDLES: OnceLock<TwiddleTree<CpuBackend>> = OnceLock::new();

#[derive(Serialize, Deserialize)]
pub struct ProofResult {
    pub proof_bytes: Vec<u8>,
    pub claim_last_val: u32,
    pub time_ms: f64,
}

// 1. Define the AIR Component
#[derive(Clone, Debug)]
pub struct FibonacciEval {
    pub log_n_rows: u32,
    pub claim_last: BaseField,
}

impl FrameworkEval for FibonacciEval {
    fn log_size(&self) -> u32 {
        self.log_n_rows
    }
    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_n_rows + 1
    }
    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let mut a = eval.next_trace_mask();
        let mut b = eval.next_trace_mask();
        
        // Constraints: a_{i+2} = a_{i+1}^2 + a_{i}^2
        for _ in 2..FIB_LEN {
            let c = eval.next_trace_mask();
            eval.add_constraint(c.clone() - (a.square() + b.square()));
            a = b;
            b = c;
        }
        
        // Boundary constraint: The last value 'b' must equal self.claim_last.
        eval.add_constraint(b - self.claim_last.into());
        
        eval
    }
}

pub type FibonacciComponent = FrameworkComponent<FibonacciEval>;

// 2. Helper to generate trace
fn generate_trace(log_size: u32, start_a: BaseField, start_b: BaseField) -> (ColumnVec<CircleEvaluation<CpuBackend, BaseField, BitReversedOrder>>, BaseField) {
    let n_rows = 1 << log_size;
    
    // CpuBackend columns are just Vec<BaseField>
    let mut trace = (0..FIB_LEN)
        .map(|_| vec![BaseField::zero(); n_rows])
        .collect_vec();
    
    let mut last_val = BaseField::zero();

    // Fill all rows with the SAME sequence
    let mut seq = Vec::with_capacity(FIB_LEN);
    let mut a = start_a;
    let mut b = start_b;
    seq.push(a);
    seq.push(b);
    for _ in 2..FIB_LEN {
        let c = a.square() + b.square();
        seq.push(c);
        a = b;
        b = c;
    }
    last_val = b;

    for col_idx in 0..FIB_LEN {
        let val = seq[col_idx];
        for row_idx in 0..n_rows {
            trace[col_idx][row_idx] = val;
        }
    }

    let domain = CanonicCoset::new(log_size).circle_domain();
    let evals = trace
        .into_iter()
        .map(|eval| CircleEvaluation::<CpuBackend, _, BitReversedOrder>::new(domain, eval))
        .collect_vec();
        
    (evals, last_val)
}

#[wasm_bindgen]
pub fn init_prover() {
    console_error_panic_hook::set_once();
    TWIDDLES.get_or_init(|| {
        let config = PcsConfig::default();
        CpuBackend::precompute_twiddles(
            CanonicCoset::new(LOG_N_ROWS + 1 + config.fri_config.log_blowup_factor)
                .circle_domain()
                .half_coset,
        )
    });
}

#[wasm_bindgen]
pub fn prove_fib(seed_val: u32) -> Result<String, JsValue> {
    
    let start = web_sys::window().unwrap().performance().unwrap().now();

    // Setup Inputs
    let a0 = BaseField::one();
    let b0 = BaseField::from_u32_unchecked(seed_val);
    
    let (trace, last_val) = generate_trace(LOG_N_ROWS, a0, b0);
    let claim_u32 = last_val.0; 

    // Config
    let config = PcsConfig::default();
    let twiddles = TWIDDLES.get().expect("Prover not initialized");

    // Channel & Commitment Scheme
    let prover_channel = &mut Blake2sM31Channel::default();
    let mut commitment_scheme = CommitmentSchemeProver::<
        CpuBackend,
        Blake2sM31MerkleChannel,
    >::new(config, twiddles);

    // Commit to trace
    let mut tree_builder = commitment_scheme.tree_builder();
    tree_builder.extend_evals(trace);
    tree_builder.commit(prover_channel);

    // Component
    let component = FibonacciComponent::new(
        &mut TraceLocationAllocator::default(),
        FibonacciEval {
            log_n_rows: LOG_N_ROWS,
            claim_last: last_val,
        },
        SecureField::zero(),
    );

    // Prove
    let proof = prove::<CpuBackend, Blake2sM31MerkleChannel>(
        &[&component],
        prover_channel,
        commitment_scheme,
    ).map_err(|e| JsValue::from_str(&format!("Proving error: {:?}", e)))?;

    let end = web_sys::window().unwrap().performance().unwrap().now();

    // Serialize
    let proof_bytes = bincode::serialize(&proof)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?;

    let result = ProofResult {
        proof_bytes,
        claim_last_val: claim_u32,
        time_ms: end - start,
    };

    Ok(serde_json::to_string(&result).unwrap())
}

#[wasm_bindgen]
pub fn verify_fib(proof_json: &str) -> bool {
    let result: ProofResult = match serde_json::from_str(proof_json) {
        Ok(r) => r,
        Err(_) => return false,
    };

    let proof: StarkProof<<Blake2sM31MerkleChannel as MerkleChannel>::H> = match bincode::deserialize(&result.proof_bytes) {
        Ok(p) => p,
        Err(_) => return false,
    };
    
    let config = PcsConfig::default();
    let verifier_channel = &mut Blake2sM31Channel::default();
    let commitment_scheme =
        &mut CommitmentSchemeVerifier::<Blake2sM31MerkleChannel>::new(config);

    let component = FibonacciComponent::new(
        &mut TraceLocationAllocator::default(),
        FibonacciEval {
            log_n_rows: LOG_N_ROWS,
            claim_last: BaseField::from_u32_unchecked(result.claim_last_val),
        },
        SecureField::zero(),
    );

    let sizes = component.trace_log_degree_bounds();
    commitment_scheme.commit(proof.commitments[0], &sizes[0], verifier_channel);
    
    verify(&[&component], verifier_channel, commitment_scheme, proof).is_ok()
}
