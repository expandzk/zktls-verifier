use ethers::types::Address;
use prost::Message;
use protos::api::ClaimTunnelResponse;
use std::str::FromStr;
use verifier::verify_attestor_claim;

#[tokio::test]
async fn test_valid_signatures() {
    let response_data = include_bytes!("files/proof_data.data");
    let response = ClaimTunnelResponse::decode(response_data.as_ref()).unwrap();

    let expected_url =
        "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
    let valid_signers =
        vec![Address::from_str("0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266").unwrap()];
    let result = verify_attestor_claim(&response, &valid_signers, expected_url);
    println!("result: {:?}", result);
    assert!(result.is_ok());
}
