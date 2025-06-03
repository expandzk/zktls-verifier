mod error;

pub use error::VerifyError;

use ethers::{
    core::types::{Address, Signature},
    types::H256,
    utils::hash_message,
};
use log::error;
use prost::Message;
use protos::api::{
    ClaimTunnelRequest, ClaimTunnelResponse, ProviderClaimData, claim_tunnel_response,
};
use std::collections::HashMap;
use std::convert::TryFrom;

/// Validates claim signatures in a ClaimTunnelResponse and ensures data comes from the specified URL
///
/// # Arguments
///
/// * `response` - The ClaimTunnelResponse to verify
/// * `valid_signers` - Vector of valid signer addresses
/// * `expected_url` - The expected URL that the data should come from
/// * `attestor_version` - The version of the attestor (e.g. "2.0.1")
/// * `signature_type` - The type of signature (e.g. "SERVICE_SIGNATURE_TYPE_ETH")
///
/// # Returns
///
/// * `Result<(), VerifyError>` - Ok(()) if all signatures are valid and URL matches, Err otherwise
pub fn verify_attestor_claim(
    response: &ClaimTunnelResponse,
    valid_signers: &[Address],
    expected_url: &str,
) -> Result<(), VerifyError> {
    // First validate the request data
    let request = response
        .request
        .as_ref()
        .ok_or_else(|| VerifyError::MissingField("request".to_string()))?;

    verify_request_url(request, expected_url)?;

    let signatures = response
        .signatures
        .as_ref()
        .ok_or_else(|| VerifyError::MissingField("signatures".to_string()))?;

    // Verify attestor address is in valid signers
    let attestor_address = signatures.attestor_address.parse::<Address>()?;

    if !valid_signers.contains(&attestor_address) {
        error!("Attestor address mismatch: {}", attestor_address);
        return Err(VerifyError::InvalidSigner("Attestor".to_string()));
    }

    // Verify result signature
    let result_hash = build_result_message(response)?;
    verify_ethereum_signature(&signatures.result_signature, result_hash, &attestor_address)?;

    // Verify claim signature
    let result = response
        .result
        .as_ref()
        .ok_or_else(|| VerifyError::MissingField("result".to_string()))?;
    match result {
        claim_tunnel_response::Result::Claim(claim_data) => {
            let claim_hash = build_claim_message(claim_data)?;
            verify_ethereum_signature(&signatures.claim_signature, claim_hash, &attestor_address)?;
        }
        claim_tunnel_response::Result::Error(error) => {
            error!("Error: {:?}", error);
            return Err(VerifyError::InvalidRequestData(
                "Error in claim".to_string(),
            ));
        }
    }

    Ok(())
}

/// Validates that the request data matches the expected URL
fn verify_request_url(request: &ClaimTunnelRequest, expected_url: &str) -> Result<(), VerifyError> {
    let data = request
        .data
        .as_ref()
        .ok_or_else(|| VerifyError::InvalidRequestData("Failed to parse parameters".to_string()))?;

    // Parse the parameters JSON string
    let parameters: HashMap<String, serde_json::Value> = serde_json::from_str(&data.parameters)
        .map_err(|_| VerifyError::InvalidRequestData("Failed to parse parameters".to_string()))?;

    // Extract the URL from parameters
    let actual_url = parameters
        .get("url")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            VerifyError::InvalidRequestData("URL not found in parameters".to_string())
        })?;

    // Compare with expected URL
    if actual_url != expected_url {
        return Err(VerifyError::UrlMismatch {
            expected: expected_url.to_string(),
            actual: actual_url.to_string(),
        });
    }

    Ok(())
}

fn build_result_message(response: &ClaimTunnelResponse) -> Result<H256, VerifyError> {
    let mut response_message = ClaimTunnelResponse::default();
    response_message.request = response.request.clone();
    response_message.result = response.result.clone();
    response_message.signatures = None;
    let result = response_message.encode_to_vec();
    Ok(hash_message(result))
}

fn build_claim_message(claim: &ProviderClaimData) -> Result<H256, VerifyError> {
    // Convert owner address to lowercase
    let owner = claim.owner.to_lowercase();

    // Create the lines array similar to TypeScript
    let lines = vec![
        claim.identifier.clone(),
        owner,
        claim.timestamp_s.to_string(),
        claim.epoch.to_string(),
    ];

    // Join with newlines
    let message = lines.join("\n");
    let message_hash = hash_message(message);
    Ok(message_hash)
}

fn verify_ethereum_signature(
    signature: &[u8],
    message_hash: H256,
    address: &Address,
) -> Result<(), VerifyError> {
    let signature = Signature::try_from(signature)?;
    let recovered_address = signature.recover(message_hash)?;
    if recovered_address != *address {
        error!("signature mismatch: {} != {}", recovered_address, address);
        return Err(VerifyError::InvalidSignature("Ethereum".to_string()));
    }
    Ok(())
}
