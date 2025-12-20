use num_traits::Zero;
use stwo::core::air::Component;
use stwo::core::channel::Blake2sM31Channel;
use stwo::core::fields::m31::BaseField;
use stwo::core::fields::qm31::SecureField;
use stwo::core::pcs::{CommitmentSchemeVerifier, PcsConfig};
use stwo::core::poly::circle::CanonicCoset;
use stwo::core::vcs::blake2_merkle::Blake2sM31MerkleChannel;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::backend::{Col, Column};
use stwo::prover::poly::circle::{CircleEvaluation, PolyOps};
use stwo::prover::poly::BitReversedOrder;
use stwo::prover::{prove, CommitmentSchemeProver};
use stwo_constraint_framework::{EvalAtRow, FrameworkComponent, FrameworkEval, TraceLocationAllocator};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct AgeCheckResult {
    success: bool,
    message: String,
    proof_str: String,
}

#[wasm_bindgen]
impl AgeCheckResult {
    #[wasm_bindgen(getter)]
    pub fn success(&self) -> bool {
        self.success
    }

    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn proof_str(&self) -> String {
        self.proof_str.clone()
    }
}

// Component for Age Check
#[derive(Clone)]
pub struct AgeCheckEval {
    pub log_n_rows: u32,
}

impl FrameworkEval for AgeCheckEval {
    fn log_size(&self) -> u32 {
        self.log_n_rows
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_n_rows + 8
    }

    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let age = eval.next_trace_mask();
        
        let mut product = age.clone() - BaseField::from(18).into();
        
        for i in 19..=150 {
            product = product * (age.clone() - BaseField::from(i).into());
        }

        eval.add_constraint(product);
        eval
    }
}

pub type AgeCheckComponent = FrameworkComponent<AgeCheckEval>;

fn generate_trace(
    log_size: u32,
    age_claim: u32,
) -> Vec<CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>> {
    let size = 1 << log_size;
    let mut col = Col::<SimdBackend, BaseField>::zeros(size);
    
    // Fill the column with the claimed age.
    // PackedBaseField::from(BaseField::from(age_claim)) would work for SimdBackend.
    for i in 0..col.data.len() {
        col.data[i] = BaseField::from(age_claim).into();
    }

    let domain = CanonicCoset::new(log_size).circle_domain();
    vec![CircleEvaluation::new(domain, col)]
}

#[wasm_bindgen]
pub fn prove_age_valid(birth_year: u32) -> AgeCheckResult {
    console_error_panic_hook::set_once();
    
    let current_year = 2025;
    if birth_year > current_year {
        return AgeCheckResult {
            success: false,
            message: "Invalid birth year (future)".to_string(),
            proof_str: "".to_string(),
        };
    }
    let age = current_year - birth_year;

    let log_n_rows = 5; 
    
    let mut config = PcsConfig::default();
    config.fri_config.log_blowup_factor = 8; 

    let twiddles = SimdBackend::precompute_twiddles(
        CanonicCoset::new(log_n_rows + 9 + config.fri_config.log_blowup_factor)
            .circle_domain()
            .half_coset,
    );

    let prover_channel = &mut Blake2sM31Channel::default();
    let mut commitment_scheme = CommitmentSchemeProver::<
        SimdBackend,
        Blake2sM31MerkleChannel,
    >::new(config, &twiddles);

    let mut tree_builder = commitment_scheme.tree_builder();
    tree_builder.extend_evals([]);
    tree_builder.commit(prover_channel);

    let trace = generate_trace(log_n_rows, age);
    let mut tree_builder = commitment_scheme.tree_builder();
    tree_builder.extend_evals(trace);
    tree_builder.commit(prover_channel);

    let component = AgeCheckComponent::new(
        &mut TraceLocationAllocator::default(),
        AgeCheckEval {
            log_n_rows,
        },
        SecureField::zero(),
    );

    match prove::<SimdBackend, Blake2sM31MerkleChannel>(
        &[&component],
        prover_channel,
        commitment_scheme,
    ) {
        Ok(proof) => {
            let proof_str = format!("{:#?}", proof);
            
            // Verify locally before returning
            let verifier_channel = &mut Blake2sM31Channel::default();
            let mut verifier_commitment_scheme = CommitmentSchemeVerifier::<Blake2sM31MerkleChannel>::new(config);
            
            let sizes = component.trace_log_degree_bounds();
            verifier_commitment_scheme.commit(proof.commitments[0], &sizes[0], verifier_channel);
            verifier_commitment_scheme.commit(proof.commitments[1], &sizes[1], verifier_channel);
            
            match stwo::core::verifier::verify(&[&component], verifier_channel, &mut verifier_commitment_scheme, proof) {
                Ok(_) => AgeCheckResult {
                    success: true,
                    message: format!("Proof generated and verified for Age: {}.", age),
                    proof_str,
                },
                Err(e) => AgeCheckResult {
                    success: false,
                    message: format!("Proof generation succeeded but verification failed: {:?}", e),
                    proof_str,
                }
            }
        },
        Err(e) => {
            AgeCheckResult {
                success: false,
                message: format!("Proof generation failed: {:?}", e),
                proof_str: "".to_string(),
            }
        }
    }
}
