use cosmwasm_std::{Deps, StdResult};

use crate::hasher::{generate_proof_as_string, Proof};

pub fn query_hash(
    _deps: Deps, 
    proof: String,
    depth: u32,
) -> StdResult<Proof> {
    let hash: String = generate_proof_as_string(depth, proof).unwrap();
    let generated_proof = Proof {depth, hash};
    Ok(generated_proof)
}